use env_logger::Env;
use webrtc::tcp::server::Server;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    Server::new().run().await;
}
