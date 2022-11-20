use serde::Deserialize;
use std::{collections::HashMap, fmt::Debug};

use crate::{
    api::{self, Pack, Unpack},
    http::{self, Method},
};

use super::{ApiResult, ClientError, Error, RequestCredentials};

#[derive(Debug)]
pub struct RoomGetRequest {
    pub credentials: RequestCredentials,
}

#[derive(Debug, Deserialize)]
pub struct RoomGetResponse {
    pub id: String,
}

impl<E> Pack<http::Request, Error<E>> for RoomGetRequest
where
    E: Debug,
{
    fn pack(self) -> Result<http::Request, Error<E>> {
        let mut headers = HashMap::<String, String>::new();
        headers.insert("hue-application-key".to_owned(), self.credentials.username);

        Ok(http::Request {
            url: format!("{}/clip/v2/resource/room", self.credentials.base_url),
            method: Method::Get,
            headers,
            body: None,
        })
    }
}

impl<E> Unpack<http::Response, Error<E>> for ApiResult<Vec<RoomGetResponse>>
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

impl<E> api::Request<http::Request, http::Response, Error<E>> for RoomGetRequest
where
    E: Debug,
{
    type Response = ApiResult<Vec<RoomGetResponse>>;
}
