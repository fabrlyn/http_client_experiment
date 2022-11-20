use async_trait::async_trait;
use hyper::{
    header::{HeaderName, InvalidHeaderName, InvalidHeaderValue},
    http::HeaderValue,
    HeaderMap,
};
use reqwest::blocking::{Client, RequestBuilder};

use crate::http::{asyn, syn, Method, Request, Response};

#[derive(Debug)]
pub enum HeaderError {
    Name(InvalidHeaderName),
    Value(InvalidHeaderValue),
}

#[derive(Debug)]
pub enum ReqwestError {
    Error(reqwest::Error),
    Header(HeaderError),
}

impl From<reqwest::Error> for ReqwestError {
    fn from(value: reqwest::Error) -> Self {
        Self::Error(value)
    }
}

impl From<InvalidHeaderValue> for ReqwestError {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::Header(HeaderError::Value(value))
    }
}

impl From<InvalidHeaderName> for ReqwestError {
    fn from(value: InvalidHeaderName) -> Self {
        Self::Header(HeaderError::Name(value))
    }
}

fn to_headers(request: &Request) -> Result<HeaderMap, ReqwestError> {
    let mut headers = HeaderMap::new();

    for (k, v) in &request.headers {
        let key = HeaderName::from_bytes(k.as_bytes())?;
        let value = HeaderValue::from_str(v.as_ref())?;
        headers.insert(key, value);
    }

    Ok(headers)
}

impl syn::Client for Client {
    type Error = ReqwestError;

    fn http_execute(&self, request: Request) -> Result<Response, Self::Error> {
        use Method::*;

        match request.method {
            Get => syn_get(self, request),
            Post => syn_post(self, request),
            Put => syn_put(self, request),
            Delete => syn_delete(self, request),
        }
    }
}

fn syn_call(request: Request, builder: RequestBuilder) -> Result<Response, ReqwestError> {
    let response = builder.headers(to_headers(&request)?).send()?;
    let status_code = response.status().as_u16();

    let body = response.bytes()?;

    Ok(Response {
        status_code,
        body: Some(body.to_vec()),
    })
}

fn syn_get(client: &Client, request: Request) -> Result<Response, ReqwestError> {
    let builder = client.get(&request.url);
    syn_call(request, builder)
}

fn syn_post(client: &Client, mut request: Request) -> Result<Response, ReqwestError> {
    let mut builder = client.post(&request.url);

    let body = request.body.take();
    if let Some(body) = body {
        builder = builder.body(body);
    }

    syn_call(request, builder)
}

fn syn_put(client: &Client, mut request: Request) -> Result<Response, ReqwestError> {
    let mut builder = client.post(&request.url);

    let body = request.body.take();
    if let Some(body) = body {
        builder = builder.body(body);
    }

    syn_call(request, builder)
}

fn syn_delete(client: &Client, request: Request) -> Result<Response, ReqwestError> {
    let builder = client.delete(&request.url);
    syn_call(request, builder)
}

#[async_trait]
impl asyn::Client for reqwest::Client {
    type Error = reqwest::Error;

    async fn http_execute(&self, request: Request) -> Result<Response, Self::Error> {
        use Method::*;

        match request.method {
            Get => {
                let response = self.get(request.url).send().await?;
                let response = Response {
                    status_code: response.status().as_u16(),
                    body: None,
                };
                Ok(response)
            }
            Post => {
                let response = self
                    .post(request.url)
                    .body(request.body.unwrap())
                    .send()
                    .await?;
                let response = Response {
                    status_code: response.status().as_u16(),
                    body: Some(response.bytes().await?.to_vec()),
                };
                Ok(response)
            }
            Put => {
                let response = self
                    .put(request.url)
                    .body(request.body.unwrap())
                    .send()
                    .await?;
                let response = Response {
                    status_code: response.status().as_u16(),
                    body: Some(response.bytes().await?.to_vec()),
                };
                Ok(response)
            }
            Delete => {
                let response = self.get(request.url).send().await?;
                let response = Response {
                    status_code: response.status().as_u16(),
                    body: None,
                };
                Ok(response)
            }
        }
    }
}
