#![allow(warnings)]
mod http;
mod log;
mod print;
mod shell;
mod utils;

#[tokio::main]
async fn main() {
    http::listen::run().await;
}
