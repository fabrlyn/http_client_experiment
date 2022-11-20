pub mod r#async;
pub mod sync;

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
