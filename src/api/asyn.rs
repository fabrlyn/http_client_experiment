use std::fmt::Debug;

use async_trait::async_trait;

use super::Request;

pub type Response<A, R> = <R as Request<
    <A as Client>::ToPack,
    <A as Client>::ToUnpack,
    <A as Client>::Error,
>>::Response;

#[async_trait]
pub trait Client {
    type Error: Debug + Send;
    type ToPack;
    type ToUnpack;

    async fn api_execute<R>(&self, request: R) -> Result<Response<Self, R>, Self::Error>
    where
        R: Request<Self::ToPack, Self::ToUnpack, Self::Error> + Send;
}

pub trait AbstractRequest<A>:
    Request<
    <A as Client>::ToPack,
    <A as Client>::ToUnpack,
    <A as Client>::Error,
>
where
    A: Client,
{
}

impl<T, A> AbstractRequest<A> for T
where
    A: Client,
    T: Request<
        <A as Client>::ToPack,
        <A as Client>::ToUnpack,
        <A as Client>::Error,
    >,
{
}
