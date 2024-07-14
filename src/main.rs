#![allow(warnings)]
mod log;
mod print;
mod server;
mod utils;

#[tokio::main]
async fn main() {
    server::listen::run().await;
}
