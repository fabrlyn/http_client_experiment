use crate::http::{HttpClient, Request, Response};

pub struct MockClient {}

impl HttpClient for MockClient {
    type Error = String;

    fn http_execute(&self, _request: Request) -> Result<Response, Self::Error> {
        todo!()
    }
}
