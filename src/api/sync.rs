use std::fmt::Debug;

use super::Request;

pub type Response<A, R> = <R as Request<
    <A as ApiClient>::ToPack,
    <A as ApiClient>::ToUnpack,
    <A as ApiClient>::Error,
>>::Response;

pub trait ApiClient {
    type Error: Debug;
    type ToPack;
    type ToUnpack;

    fn api_execute<R>(&self, request: R) -> Result<Response<Self, R>, Self::Error>
    where
        R: Request<Self::ToPack, Self::ToUnpack, Self::Error>;
}

pub trait AbstractRequest<A>:
    Request<<A as ApiClient>::ToPack, <A as ApiClient>::ToUnpack, <A as ApiClient>::Error>
where
    A: ApiClient,
{
}

impl<T, A> AbstractRequest<A> for T
where
    A: ApiClient,
    T: Request<<A as ApiClient>::ToPack, <A as ApiClient>::ToUnpack, <A as ApiClient>::Error>,
{
}
