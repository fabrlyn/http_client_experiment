use serde::Deserialize;

use crate::http::StatusCode;

pub mod room_get;
pub mod room_post;
pub mod sync_client;
pub mod async_client;

#[derive(Debug)]
pub struct RequestCredentials {
    pub base_url: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub errors: Vec<ApiError>,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub errors: Vec<ApiError>,
    pub data: T,
}

#[derive(Debug)]
pub enum ClientError {
    ExpectedBody,
}

#[derive(Debug)]
pub enum Error<E> {
    Hue(ApiErrorResponse),
    Io(std::io::Error),
    Client(ClientError),
    Http(E),
}

#[derive(Debug)]
pub enum ApiResult<T> {
    Ok(ApiResponse<T>),
    Err(StatusCode, ApiErrorResponse),
}
