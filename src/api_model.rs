use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    api::{ApiResult, ClientError, Error, Pack, Unpack, WithResponse},
    http::{Method, Request, Response},
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

// Behaviour

impl Pack<Request> for RoomPostRequest {
    fn pack<E>(self) -> Result<Request, Error<E>> {
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

impl WithResponse for RoomPostRequest {
    type Response = ApiResult<RoomPostResponse>;
}

impl Unpack<Response> for ApiResult<RoomPostResponse> {
    fn unpack<E>(value: Response) -> Result<Self, Error<E>> {
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
