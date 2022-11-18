use reqwest::blocking::Client;

use crate::{
    api::{ApiClient, ApiRequest, Unpack},
    api_model::{self, Error},
    http::{HttpClient, Method, Request, Response},
};

impl HttpClient for Client {
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

impl ApiClient for Client {
    type Error = api_model::Error<reqwest::Error>;
    type ToPack = Request;
    type ToUnpack = Response;

    fn api_execute<R>(
        &self,
        request: R,
    ) -> Result<<R as ApiRequest<Self::ToPack, Self::ToUnpack, Self::Error>>::Response, Self::Error>
    where
        R: ApiRequest<Self::ToPack, Self::ToUnpack, api_model::Error<reqwest::Error>>,
    {
        let response = self.http_execute(request.pack()?).map_err(Error::Http)?;
        <R as ApiRequest<Self::ToPack, Self::ToUnpack, Self::Error>>::Response::unpack(response)
    }
}
