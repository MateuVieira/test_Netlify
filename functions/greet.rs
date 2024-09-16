use lambda_http::{run, http::{StatusCode, Response}, service_fn, Error, IntoResponse, Request, RequestExt};

use serde_json::json;

use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {

    tracing_subscriber::fmt()
    .without_time()
    .with_max_level(tracing::Level::INFO)
    .init();

    run(service_fn(greet)).await
}

pub async fn greet(event: Request) -> Result<impl IntoResponse, Error> {
    let name = event.payload::<GreetPayload>()?
        .and_then(|payload| {
            if payload.name.is_empty() {
                None 
            } else {
                Some(payload.name)
            }
        })
        .ok_or_else(|| Error::from("Missing or empty name"))?;
   
    let response = Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "application/json")
    .body(json!({
        "message": format!("Hello, {}!", name),
      }).to_string())
    .map_err(Box::new)?;

    Ok(response)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GreetPayload {
    pub name: String,
}