use log::{ info, error, warn, trace };
use crate::bindings::Tdlib;
use std::{
    marker::PhantomData,
    task::{ Waker, Context, Poll },
    future::Future,
    pin::Pin,
    sync::{ Arc, Mutex },
    thread,
};
use crossbeam::channel::{
    self,
    Sender,
    Receiver,
};
use uuid::Uuid;
use std::collections::HashMap;
use serde_json::Value as JsonValue;
use crate::update::Handler;
use paperplane_types::methods::Method;


#[derive(Debug)]
pub struct RequestData {
    resp: Option<JsonValue>,
    waker: Option<Waker>,
}

type RequestDataRef = Arc<Mutex<RequestData>>;

#[derive(Debug)]
struct RequestDataToStream {
    data: RequestDataRef,
    req: JsonValue,
}

#[derive(Debug, Clone)]
pub struct RequestFuture<M: Method> {
    _response_type_holder: PhantomData<M>,
    pub data: RequestDataRef,
}

impl<M: Method> Future for RequestFuture<M> {
    type Output = Result<M::Response, serde_json::error::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut data = self.data.lock().unwrap();
        if let Some(resp) = &data.resp {
            Poll::Ready(serde_json::from_value(resp.clone()))
        } else {
            data.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[derive(Debug)]
enum JoinStreams {
    NewRequest(RequestDataToStream),
    NewResponse(String),
}

#[derive(Clone)]
pub struct Client {
    //waker_handle: std::thread::JoinHandle<()>,
    sender: Sender<JoinStreams>,
}

impl Client {
    pub fn new<H: Handler>(log_opt: Option<i32>, updater: H) -> Self {
        if let Some(log) = log_opt {
            Tdlib::set_log_verbosity_level(log);
        }
        let (tx, rx) = channel::unbounded();

        let tx_for_tg = tx.clone();
        let api = Arc::new(Tdlib::new());
        let api_for_listener = api.clone();
        let api_for_responder = api.clone();
        let sender_for_responder = tx.clone();
        let rt = tokio::runtime::Handle::try_current().expect("Must be in runtime");

        let _run_handle = thread::spawn(
            move || OneshotResponder::new(rx, api_for_responder).run(
                updater, Self { sender: sender_for_responder }, rt
            )
        );
        let _tg_handle = thread::spawn(
            move || Self::listen_tg(tx_for_tg, api_for_listener, 1.0)
        );
        Self {
            //waker_handle: _run_handle,
            sender: tx,
        }
    }

    pub fn send<R: Method>(&self, req: R) -> Result<RequestFuture<R>, serde_json::error::Error> {
        let request = RequestData {
            resp: None,
            waker: None
        };
        let fut = RequestFuture {
            _response_type_holder: PhantomData,
            data: Arc::new(Mutex::new(request))
        };

        self.sender.send(JoinStreams::NewRequest(
            RequestDataToStream {
                data: fut.data.clone(),
                req: serde_json::to_value(req.tag())?,
            }
        )).unwrap();
        Ok(fut)
    }

    fn listen_tg(tx: Sender<JoinStreams>, api: Arc<Tdlib>, timeout: f64) {
        loop {
            if let Some(msg) = api.receive(timeout) {
                tx.send(JoinStreams::NewResponse(msg)).unwrap();
            } else {
                info!("receive timed out");
            }
        }
    }
}

#[derive(Debug)]
struct OneshotResponder {
    api: Arc<Tdlib>,
    wakers_map: HashMap<Uuid, RequestDataRef>,
    rx: Receiver<JoinStreams>,
}

impl OneshotResponder {
    fn new(rx: Receiver<JoinStreams>, api: Arc<Tdlib>) -> Self {
        Self {
            api: api,
            wakers_map: HashMap::new(),
            rx: rx
        }
    }
    
    fn run<H: Handler>(&mut self, updater: H, client: Client, rt: tokio::runtime::Handle) {
        loop {
            match self.rx.recv() {
                Ok(JoinStreams::NewRequest(req_data)) => {
                    let id = loop {
                        let id = Uuid::new_v4();
                        if self.wakers_map.contains_key(&id) {
                            continue;
                        } else {
                            break id;
                        }
                    };
                    let mut request = req_data.req;
                    if !request["@extra"].is_null() {
                        warn!("overwriting @extra in request");
                    }
                    request["@extra"] = id.to_hyphenated().to_string().into();
                    self.api.send(request.to_string().as_ref());
                    self.wakers_map.insert(id, req_data.data);
                    trace!("new req:\n{:#}", request);
                },
                Ok(JoinStreams::NewResponse(resp)) => {
                    match serde_json::from_str::<JsonValue>(resp.as_ref()) {
                        Ok(val) => {
                            let typ = val["@type"].as_str().unwrap();
                            if typ.starts_with("update") {
                                match serde_json::from_value(val.clone()) {
                                    Ok(upd) => { rt.spawn(updater.handle(client.clone(), upd)); },
                                    Err(err) => {
                                        error!("Could not deser update: {}, was: {}", err, val);
                                    }
                                };
                            } else {
                                self.handle_response(val);
                            }
                        },
                        Err(e) => {
                            warn!("ignoring invalid response. err: {}, resp: {}", e, resp);
                        }
                    }
                },
                Err(e) => {
                    error!("stream closed: {}", e);
                    error!("closing thread");
                    break;
                }
            }
        }
    }

    fn handle_response(&mut self, resp: JsonValue) {
        if let Some(id_str) = resp["@extra"].as_str() {
            if let Ok(id) = Uuid::parse_str(id_str) {
                let fut_extracted = self.wakers_map
                    .remove(&id)
                    .unwrap();

                let mut fut_data = fut_extracted.lock().unwrap();
                fut_data.resp = Some(resp);
                fut_data
                    .waker.as_ref()
                    .and_then(|waker: &Waker| { waker.clone().wake(); Some(()) });
            }
        } else {
            warn!("update has invalid @extra: {}", resp);
        }
    }
}
