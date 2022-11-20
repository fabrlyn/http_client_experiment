use std::collections::HashMap;

pub mod asyn;
pub mod syn;

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
