mod key_manager;
mod signer;
mod api;

#[tokio::main]
async fn main() {
    api::start_api_server().await;
}
