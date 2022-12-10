use webrtc::tcp::server::Server;

#[tokio::main]
async fn main() {
    let server = Server::new();
    server.run().await;
}
