// Behaviour

pub trait Pack<V, E> {
    fn pack(self) -> Result<V, E>;
}

pub trait Unpack<V, E>: Sized {
    fn unpack(value: V) -> Result<Self, E>;
}

pub trait ApiRequest<A, B, E>: Pack<A, E> {
    type Response: Unpack<B, E>;
}

pub trait ApiClient {
    type Error;
    type ToPack;
    type ToUnpack;

    fn api_execute<R>(
        &self,
        request: R,
    ) -> Result<<R as ApiRequest<Self::ToPack, Self::ToUnpack, Self::Error>>::Response, Self::Error>
    where
        R: ApiRequest<Self::ToPack, Self::ToUnpack, Self::Error>;
}
