use async_trait::async_trait;

use crate::core::traits::contexts::relay::RelayContext;
use crate::core::traits::core::Async;
use crate::core::traits::target::ChainTarget;
use crate::core::types::aliases::{IbcEvent, IbcMessage};
use crate::std_prelude::*;

#[async_trait]
pub trait BatchContext: Async {
    type Error: Async;

    type Message: Async;
    type Event: Async;

    type BatchSender: Async;
    type BatchReceiver: Async;

    type ResultSender: Async;
    type ResultReceiver: Async;

    fn new_result_channel() -> (Self::ResultSender, Self::ResultReceiver);

    async fn send_batch(
        sender: &Self::BatchSender,
        messages: Vec<Self::Message>,
        result_sender: Self::ResultSender,
    ) -> Result<(), Self::Error>;

    async fn try_receive_batch(
        receiver: &Self::BatchReceiver,
    ) -> Result<Option<(Vec<Self::Message>, Self::ResultSender)>, Self::Error>;

    async fn receive_result(
        result_receiver: Self::ResultReceiver,
    ) -> Result<Result<Vec<Vec<Self::Event>>, Self::Error>, Self::Error>;

    fn send_result(
        result_sender: Self::ResultSender,
        events: Result<Vec<Vec<Self::Event>>, Self::Error>,
    ) -> Result<(), Self::Error>;
}

pub trait HasBatchContext<Target>: RelayContext
where
    Target: ChainTarget<Self>,
{
    type BatchContext: BatchContext<
        Message = IbcMessage<Target::TargetChain, Target::CounterpartyChain>,
        Event = IbcEvent<Target::TargetChain, Target::CounterpartyChain>,
        Error = Self::Error,
    >;

    fn batch_sender(&self) -> &<Self::BatchContext as BatchContext>::BatchSender;

    fn batch_receiver(&self) -> &<Self::BatchContext as BatchContext>::BatchReceiver;
}