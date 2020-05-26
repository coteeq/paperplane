use std::future::Future;
use futures::future::BoxFuture;
use crate::client::Client;
use paperplane_types::types::Update;

pub trait Handler: Send + Sync + 'static {
    fn handle(&self, _: Client, _: Update) -> BoxFuture<'static, ()>;
}

impl<C, F> Handler for C
where C: Send + Sync + 'static + Fn(Client, Update) -> F,
      F: Future<Output = ()> + 'static + Send {
    fn handle(&self, client: Client, req: Update) -> BoxFuture<'static, ()> {
        Box::pin((*self)(client, req))
    }
}
