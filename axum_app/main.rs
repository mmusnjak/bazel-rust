extern crate hello_lib;

use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    let _greeter = hello_lib::Greeter::new("Hello");
    Html("<h1>Hello World</h1>")
    // https://shanee.io/blog/2019/05/28/bazel-with-visual-studio-code/
}
