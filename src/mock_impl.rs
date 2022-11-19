use serde_json::json;

use crate::http::{HttpClient, Request, Response};

pub struct MockClient {}

impl HttpClient for MockClient {
    type Error = String;

    fn http_execute(&self, _request: Request) -> Result<Response, Self::Error> {
        Ok(Response {
            status_code: 200,
            body: Some(
                json!({"data": {"id": "room-1"}, "errors": []})
                    .to_string()
                    .as_bytes()
                    .to_vec(),
            ),
        })
    }
}
