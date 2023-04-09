use anyhow::{anyhow, Result};
use spin_sdk::{
    http::{Request as SpinRequest, Response as SpinResponse},
    http_component,
};

use axum::{body::HttpBody, response::Response, Error};

mod db;
mod todo;

use bytes::BytesMut;
use futures_executor::block_on;

fn convert_spin_request_to_axum_request(
    request: http::Request<Option<bytes::Bytes>>,
) -> http::Request<String> {
    let (parts, body_option) = request.into_parts();

    let body = body_option.map_or_else(
        || String::new(),
        |b| String::from_utf8_lossy(&b).to_string(),
    );

    http::Request::from_parts(parts, body)
}

const MTU: usize = 8192;
async fn convert_axum_response_to_spin_response(
    response: Response,
) -> Result<http::Response<Option<bytes::Bytes>>, Error> {
    let (parts, mut body) = response.into_parts();
    let mut buf = BytesMut::with_capacity(MTU);
    while let Some(chunk) = body.data().await {
        buf.extend_from_slice(&chunk?);
    }
    Ok(http::Response::from_parts(parts, Some(buf.freeze())))
}

/// A simple Spin HTTP component.
#[http_component]
fn axum_on_spin(req: SpinRequest) -> Result<SpinResponse> {
    println!("{:?}", req.headers());
    let request = convert_spin_request_to_axum_request(req.into());
    let axum_response: Response = block_on(todo::app(request));
    let spin_response = block_on(convert_axum_response_to_spin_response(axum_response));
    Ok(spin_response.map_err(|err| anyhow!("Internal Server Error: {}", err))?)
}
