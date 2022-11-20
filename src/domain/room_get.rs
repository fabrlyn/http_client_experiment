use serde::Deserialize;
use std::{collections::HashMap, fmt::Debug};

use crate::{
    api::{self, Pack, Unpack},
    http::{self, Method},
};

use super::{ApiResult, ClientError, Error};

#[derive(Debug)]
pub struct RoomGetRequest;

#[derive(Debug, Deserialize)]
pub struct RoomGetResponse {
    pub id: String,
}

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

impl<E> api::Request<http::Request, http::Response, Error<E>> for RoomGetRequest
where
    E: Debug,
{
    type Response = ApiResult<RoomGetResponse>;
}
