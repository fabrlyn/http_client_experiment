use async_trait::async_trait;

use crate::{
    api::{
        r#async::{AsyncApiClient, AsyncResponse},
        Request, Unpack,
    },
    http::{self, r#async::AsyncHttpClient},
};

use super::Error;

#[async_trait]
impl<T> AsyncApiClient for T
where
    T: AsyncHttpClient + Sync,
{
    type Error = Error<<T as AsyncHttpClient>::Error>;
    type ToPack = http::Request;
    type ToUnpack = http::Response;

    async fn api_execute<R>(&self, request: R) -> Result<AsyncResponse<Self, R>, Self::Error>
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
    AsyncApiClient<ToPack = http::Request, ToUnpack = http::Response, Error = Error<E>>
{
}

impl<T, E> ApiHttpClient<E> for T where
    T: AsyncApiClient<ToPack = http::Request, ToUnpack = http::Response, Error = Error<E>>
{
}
