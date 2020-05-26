use tokio;
use paperplane::client;
use paperplane::update;
use std::env;
use paperplane_types::{ types, methods };
use log::info;

struct UpdateHandler;

fn make_lib_params() -> types::TdlibParameters {
    let cache = env::current_dir().unwrap().join("cache");
    let make_path = |p: &str| cache.join(p).to_str().map(|p| p.to_owned()).unwrap();
    types::TdlibParameters {
        use_test_dc: true,
        database_directory: make_path("database"),
        files_directory: make_path("files"),
        use_file_database: true,
        use_chat_info_database: true,
        use_message_database: true,
        use_secret_chats: false,
        api_id: env::var("API_ID").unwrap().parse().unwrap(),
        api_hash: env::var("API_HASH").unwrap(),
        system_language_code: "en".to_owned(),
        device_model: "mbia v1".to_owned(),
        system_version: "15".to_owned(),
        application_version: "0.1".to_owned(),
        enable_storage_optimizer: false,
        ignore_file_names: true,
    }
}

impl update::Handler for UpdateHandler {
    fn handle(&self, client: client::Client, req: types::Update) -> futures::future::BoxFuture<'static, ()> {
        Box::pin(async move {
            match req {
                types::Update::UpdateAuthorizationState(state) => {
                    match state.authorization_state {
                        types::AuthorizationState::AuthorizationStateWaitTdlibParameters(_) => {
                            client.send(methods::SetTdlibParameters {
                                parameters: make_lib_params(),
                            }).unwrap().await.unwrap();
                        }
                        _ => info!("auth state unknown: {:#?}", state)
                    }
                }
                _ => info!("unknown update: {:#?}", req)
            }
        })
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let tg_log: Option<i32> = env::var("TG_LOG")
        .ok()
        .and_then(|var| var.parse().ok());

    let _tg = client::Client::new(tg_log, UpdateHandler{});

    std::thread::sleep(std::time::Duration::new(2, 0));
}
