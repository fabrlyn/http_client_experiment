use crate::{
    api::{
        sync::{ApiClient, Response},
        Request, Unpack,
    },
    http::{self, syn::Client},
};

use super::Error;

impl<T> ApiClient for T
where
    T: Client,
{
    type Error = Error<<T as Client>::Error>;
    type ToPack = http::Request;
    type ToUnpack = http::Response;

    fn api_execute<R>(&self, request: R) -> Result<Response<Self, R>, Self::Error>
    where
        R: Request<Self::ToPack, Self::ToUnpack, Self::Error>,
    {
        let response = self.http_execute(request.pack()?).map_err(Error::Http)?;
        <R as Request<Self::ToPack, Self::ToUnpack, Self::Error>>::Response::unpack(response)
    }
}

pub trait ApiHttpClient<E>:
    ApiClient<ToPack = http::Request, ToUnpack = http::Response, Error = Error<E>>
{
}

impl<T, E> ApiHttpClient<E> for T where
    T: ApiClient<ToPack = http::Request, ToUnpack = http::Response, Error = Error<E>>
{
}
