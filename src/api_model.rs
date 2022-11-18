use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    api::{ApiRequest, Pack, Unpack},
    http::{Method, Request, Response, StatusCode},
};

// Model

pub struct RequestCredentials {
    pub base_url: String,
    pub username: String,
}

pub struct RoomPostRequest {
    pub credentials: RequestCredentials,
    pub name: String,
}

#[derive(Deserialize)]
pub struct RoomPostResponse {
    pub id: String,
}

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

impl<E> Pack<Request, Error<E>> for RoomPostRequest {
    fn pack(self) -> Result<Request, Error<E>> {
        let url = format!("{}/resource/room", self.credentials.base_url);

        let mut headers = HashMap::<String, String>::new();
        headers.insert("hue-application-key".to_owned(), self.credentials.username);

        let mut body = HashMap::<String, String>::new();
        body.insert("name".to_owned(), self.name);

        let body = Some(serde_json::to_vec(&body).map_err(|e| Error::Io(e.into()))?);

        Ok(Request {
            url,
            method: Method::Post,
            headers,
            body,
        })
    }
}

impl<E> Unpack<Response, Error<E>> for ApiResult<RoomPostResponse> {
    fn unpack(value: Response) -> Result<Self, Error<E>> {
        if value.status_code != 200 {
            let body = value.body.ok_or(Error::Client(ClientError::ExpectedBody))?;

            let body = serde_json::from_slice(&body)
                .map_err(Into::into)
                .map_err(Error::Io)?;

            Ok(ApiResult::Err(value.status_code, body))
        } else {
            let body = value.body.ok_or(Error::Client(ClientError::ExpectedBody))?;

            let body = serde_json::from_slice(&body)
                .map_err(Into::into)
                .map_err(Error::Io)?;

            Ok(ApiResult::Ok(body))
        }
    }
}

impl<A, B, E> ApiRequest<A, B, Error<E>> for RoomPostRequest
where
    RoomPostRequest: Pack<A, Error<E>>,
    ApiResult<RoomPostResponse>: Unpack<B, Error<E>>,
{
    type Response = ApiResult<RoomPostResponse>;
}
