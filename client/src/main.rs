extern crate herbolution_client as client;

#[tokio::main]
async fn main() {
    client::start().expect("Failed to start game");
}