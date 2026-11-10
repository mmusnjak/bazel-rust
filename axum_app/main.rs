extern crate hello_lib;

use axum::{Router, response::Html, routing::get};
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

async fn handler() -> Html<String> {
    let greeter = hello_lib::Greeter::new("Hello");
    let id = greeter.greeting_id_encoded();
    Html(format!("<h1>{}</h1><p>{}</p>", greeter.greeting("World"), id))
    // https://shanee.io/blog/2019/05/28/bazel-with-visual-studio-code/
}
