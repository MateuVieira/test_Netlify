use lambda_http::{handler, lambda, Body, IntoResponse, Request, RequestExt, Response};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    lambda::run(handler(greet)).await?;
    Ok(())
}

async fn greet(event: Request) -> Result<impl IntoResponse, lambda_http::Error> {
    // Extract the name from the request body
    let body: Value = match event.body() {
        Body::Empty => Value::Null, // Handle empty body gracefully
        Body::Text(text) => serde_json::from_str(text).unwrap_or_default(),
        Body::Binary(data) => serde_json::from_slice(data).unwrap_or_default(),
    };

    let name = body.get("name").and_then(Value::as_str).unwrap_or("there");

    // Construct the response
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(Body::from(format!("Hello, {}!", name)))?)
}