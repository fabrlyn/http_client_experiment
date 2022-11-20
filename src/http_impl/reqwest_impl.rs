use async_trait::async_trait;
use reqwest::blocking::Client;

use crate::http::{asyn, syn, Method, Request, Response};

impl syn::Client for Client {
    type Error = reqwest::Error;

    fn http_execute(&self, request: Request) -> Result<Response, Self::Error> {
        use Method::*;

        match request.method {
            Get => todo!(),
            Post => {
                let response = self.post(request.url).body(request.body.unwrap()).send()?;
                let response = Response {
                    status_code: response.status().as_u16(),
                    body: Some(response.bytes()?.to_vec()),
                };
                Ok(response)
            }
            Put => todo!(),
            Delete => todo!(),
        }
    }
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
            Put => todo!(),
            Delete => todo!(),
        }
    }
}
