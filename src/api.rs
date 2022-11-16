use serde::Deserialize;

use crate::http::{HttpClient, Request, Response, StatusCode};

// Model

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub errors: Vec<ApiError>,
}

#[derive(Deserialize)]
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

pub enum ApiResult<T> {
    Ok(ApiResponse<T>),
    Err(StatusCode, ApiErrorResponse),
}

// Behaviour

pub trait Pack<V>: Sized {
    fn pack<E>(self) -> Result<V, Error<E>>;
}

pub trait Unpack<V>: Sized {
    fn unpack<E>(value: V) -> Result<Self, Error<E>>;
}

pub trait WithResponse: Pack<Request> {
    type Response: Unpack<Response>;
}

pub trait ApiRequest: Pack<Request> + WithResponse
where
    <Self as WithResponse>::Response: Unpack<Response>,
{
}

impl<T> ApiRequest for T
where
    T: Pack<Request> + WithResponse,
    <Self as WithResponse>::Response: Unpack<Response>,
{
}

pub trait ApiClient {
    type Error;

    fn api_execute<R>(
        &self,
        request: R,
    ) -> Result<<R as WithResponse>::Response, Error<Self::Error>>
    where
        R: ApiRequest;
}

impl<T> ApiClient for T
where
    T: HttpClient,
{
    type Error = <T as HttpClient>::Error;

    fn api_execute<R>(
        &self,
        request: R,
    ) -> Result<<R as WithResponse>::Response, Error<Self::Error>>
    where
        R: ApiRequest,
    {
        let response = self.http_execute(request.pack()?).map_err(Error::Http)?;
        <R as WithResponse>::Response::unpack(response)
    }
}
