use reqwest::blocking::Client;

use crate::{
    api2::{self, ApiClient, Error, Unpack, WithResponse},
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
    type Error = api2::Error<reqwest::Error>;

    fn api_execute<R>(
        &self,
        request: R,
    ) -> Result<<R as WithResponse<Self::Error>>::Response, Self::Error>
    where
        R: WithResponse<Self::Error>,
    {
        let response = self.http_execute(request.pack()?).map_err(Error::Http)?;
        <R as WithResponse<Self::Error>>::Response::unpack(response)
    }
}
