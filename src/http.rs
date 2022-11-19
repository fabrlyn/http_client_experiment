use std::{collections::HashMap, fmt::Debug};

// Model

pub type StatusCode = u16;

pub type Body = Vec<u8>;

pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

pub struct Request {
    pub url: String,
    pub method: Method,
    pub headers: HashMap<String, String>,
    pub body: Option<Body>,
}

pub struct Response {
    pub status_code: StatusCode,
    pub body: Option<Body>,
}

// Behaviour

pub trait HttpClient {
    type Error: Debug;

    fn http_execute(&self, request: Request) -> Result<Response, Self::Error>;
}
