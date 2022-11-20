use crate::{
    api::{self, syn::Response, Request, Unpack},
    http,
};

use super::Error;

impl<T> api::syn::Client for T
where
    T: http::syn::Client,
{
    type Error = Error<<T as http::syn::Client>::Error>;
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

pub trait Client<E>:
    api::syn::Client<ToPack = http::Request, ToUnpack = http::Response, Error = Error<E>>
{
}

impl<T, E> Client<E> for T where
    T: api::syn::Client<ToPack = http::Request, ToUnpack = http::Response, Error = Error<E>>
{
}
