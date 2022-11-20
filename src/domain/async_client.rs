use async_trait::async_trait;

use crate::{
    api::{self, r#async::Response, Request, Unpack},
    http::{self},
};

use super::Error;

#[async_trait]
impl<T> api::r#async::Client for T
where
    T: http::asyn::Client + Sync,
{
    type Error = Error<<T as http::asyn::Client>::Error>;
    type ToPack = http::Request;
    type ToUnpack = http::Response;

    async fn api_execute<R>(&self, request: R) -> Result<Response<Self, R>, Self::Error>
    where
        R: Request<Self::ToPack, Self::ToUnpack, Self::Error> + Send,
    {
        let response = self
            .http_execute(request.pack()?)
            .await
            .map_err(Error::Http)?;
        <R as Request<Self::ToPack, Self::ToUnpack, Self::Error>>::Response::unpack(response)
    }
}

pub trait ApiHttpClient<E>:
    api::r#async::Client<ToPack = http::Request, ToUnpack = http::Response, Error = Error<E>>
{
}

impl<T, E> ApiHttpClient<E> for T where
    T: api::r#async::Client<ToPack = http::Request, ToUnpack = http::Response, Error = Error<E>>
{
}
