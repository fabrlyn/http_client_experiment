use async_trait::async_trait;
use std::fmt::Debug;

use super::{Request, Response};

#[async_trait]
pub trait Client {
    type Error: Debug + Send;

    async fn http_execute(&self, request: Request) -> Result<Response, Self::Error>;
}
