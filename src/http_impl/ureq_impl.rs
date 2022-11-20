use std::io::Cursor;

use crate::http::{sync::HttpClient, Method, Request, Response};

pub struct Ureq;

impl HttpClient for Ureq {
    type Error = ureq::Error;

    fn http_execute(&self, request: Request) -> Result<Response, Self::Error> {
        use Method::*;

        match request.method {
            Get => {
                let mut req = ureq::get(&request.url);
                for (k, v) in request.headers {
                    req = req.set(k.as_ref(), v.as_ref());
                }

                let response = req.call()?;

                let status_code = response.status();
                let mut bytes: Vec<u8> = Vec::new();
                response.into_reader().read_to_end(&mut bytes)?;

                let response = Response {
                    status_code,
                    body: Some(bytes),
                };
                Ok(response)
            }
            Post => {
                let mut req = ureq::post(&request.url);
                for (k, v) in request.headers {
                    req = req.set(k.as_ref(), v.as_ref());
                }

                if let Some(body) = request.body {
                    let response = req.send(Cursor::new(body))?;

                    let status_code = response.status();
                    let mut bytes: Vec<u8> = Vec::new();
                    response.into_reader().read_to_end(&mut bytes)?;

                    let response = Response {
                        status_code,
                        body: Some(bytes),
                    };
                    Ok(response)
                } else {
                    todo!()
                }
            }
            Put => todo!(),
            Delete => todo!(),
        }
    }
}
