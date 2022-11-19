use std::fmt::Debug;

pub trait Pack<V, E>
where
    E: Debug,
{
    fn pack(self) -> Result<V, E>;
}

pub trait Unpack<V, E>: Sized
where
    E: Debug,
{
    fn unpack(value: V) -> Result<Self, E>;
}

pub trait Request<A, B, E>: Pack<A, E>
where
    E: Debug,
{
    type Response: Unpack<B, E> + Debug;
}

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
