use reqwest::blocking::Client;

use crate::http::{HttpClient, Method, Request, Response};

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
