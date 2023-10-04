use core::pin::Pin;

use cgp_core::traits::Async;
use futures_core::stream::Stream;
use ibc_relayer_components::runtime::traits::stream::HasStreamType;

use crate::std_prelude::*;

pub trait HasAsyncStreamType: HasStreamType {
    fn from_async_stream<T>(
        stream: Pin<Box<dyn Stream<Item = T> + Send + Sync + 'static>>,
    ) -> Self::Stream<T>
    where
        T: Async;

    fn to_async_stream<T>(
        stream: Self::Stream<T>,
    ) -> Pin<Box<dyn Stream<Item = T> + Send + Sync + 'static>>
    where
        T: Async;
}
