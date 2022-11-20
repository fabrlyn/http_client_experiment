use std::fmt::Debug;

use super::{Request, Response};

pub trait Client {
    type Error: Debug;

    fn http_execute(&self, request: Request) -> Result<Response, Self::Error>;
}
