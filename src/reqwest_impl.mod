
    use crate::http::{self, HttpClient, Request, Response};

    impl HttpClient for reqwest::blocking::Client {
        type Error = reqwest::Error;

        fn http_execute(&self, request: Request) -> Result<http::Response, Self::Error> {
            match request.method {
                http::Method::Get => todo!(),
                http::Method::Post => {
                    let response = self.post(request.url).body(request.body.unwrap()).send()?;
                    let response = Response {
                        status_code: response.status().as_u16(),
                        body: Some(response.bytes()?.to_vec()),
                    };
                    Ok(response)
                }
                http::Method::Put => todo!(),
                http::Method::Delete => todo!(),
            }
        }
    }
