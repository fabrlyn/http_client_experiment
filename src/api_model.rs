use std::{collections::HashMap, fmt::Debug};


use serde::Deserialize;

use crate::{
    api::{self, Pack, Unpack},
    http::{self, Method, Response, StatusCode},
};

// Model

#[derive(Debug)]
pub struct RequestCredentials {
    pub base_url: String,
    pub username: String,
}

#[derive(Debug)]
pub struct RoomPostRequest {
    pub credentials: RequestCredentials,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct RoomPostResponse {
    pub id: String,
}

#[derive(Debug)]
pub struct RoomGetRequest;

#[derive(Debug, Deserialize)]
pub struct RoomGetResponse {
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

// Behaviour

impl<E> Pack<http::Request, Error<E>> for RoomGetRequest
where
    E: Debug,
{
    fn pack(self) -> Result<http::Request, Error<E>> {
        Ok(http::Request {
            url: "".to_owned(),
            method: Method::Get,
            headers: HashMap::new(),
            body: None,
        })
    }
}

impl<E> Unpack<http::Response, Error<E>> for ApiResult<RoomGetResponse>
where
    E: Debug,
{
    fn unpack(value: http::Response) -> Result<Self, Error<E>> {
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

impl<E> Pack<http::Request, Error<E>> for RoomPostRequest
where
    E: Debug,
{
    fn pack(self) -> Result<http::Request, Error<E>> {
        let url = format!("{}/resource/room", self.credentials.base_url);

        let mut headers = HashMap::<String, String>::new();
        headers.insert("hue-application-key".to_owned(), self.credentials.username);

        let mut body = HashMap::<String, String>::new();
        body.insert("name".to_owned(), self.name);

        let body = Some(serde_json::to_vec(&body).map_err(|e| Error::Io(e.into()))?);

        Ok(http::Request {
            url,
            method: Method::Post,
            headers,
            body,
        })
    }
}

impl<E> Unpack<Response, Error<E>> for ApiResult<RoomPostResponse>
where
    E: Debug,
{
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

impl<E> api::Request<http::Request, Response, Error<E>> for RoomPostRequest
where
    E: Debug,
{
    type Response = ApiResult<RoomPostResponse>;
}

impl<E> api::Request<http::Request, Response, Error<E>> for RoomGetRequest
where
    E: Debug,
{
    type Response = ApiResult<RoomGetResponse>;
}
