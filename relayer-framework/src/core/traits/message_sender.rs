use async_trait::async_trait;

use crate::core::traits::contexts::chain::ChainContext;
use crate::core::traits::core::Async;
use crate::core::types::aliases::{Event, Message};
use crate::std_prelude::*;

pub trait HasMessageSender: ChainContext {
    type MessageSender: MessageSender<Self>;
}

#[async_trait]
pub trait MessageSender<Context>: Async
where
    Context: ChainContext,
{
    async fn send_messages(
        context: &Context,
        messages: Vec<Message<Context>>,
    ) -> Result<Vec<Vec<Event<Context>>>, Context::Error>;
}