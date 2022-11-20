use std::fmt::Debug;

use async_trait::async_trait;

use super::Request;

pub type AsyncResponse<A, R> = <R as Request<
    <A as AsyncApiClient>::ToPack,
    <A as AsyncApiClient>::ToUnpack,
    <A as AsyncApiClient>::Error,
>>::Response;

#[async_trait]
pub trait AsyncApiClient {
    type Error: Debug + Send;
    type ToPack;
    type ToUnpack;

    async fn api_execute<R>(&self, request: R) -> Result<AsyncResponse<Self, R>, Self::Error>
    where
        R: Request<Self::ToPack, Self::ToUnpack, Self::Error> + Send;
}

pub trait AbstractRequest<A>:
    Request<
    <A as AsyncApiClient>::ToPack,
    <A as AsyncApiClient>::ToUnpack,
    <A as AsyncApiClient>::Error,
>
where
    A: AsyncApiClient,
{
}

impl<T, A> AbstractRequest<A> for T
where
    A: AsyncApiClient,
    T: Request<
        <A as AsyncApiClient>::ToPack,
        <A as AsyncApiClient>::ToUnpack,
        <A as AsyncApiClient>::Error,
    >,
{
}
