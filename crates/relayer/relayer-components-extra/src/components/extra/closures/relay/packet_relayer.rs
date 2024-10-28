use cgp::prelude::{CanRaiseError, HasComponents};
use hermes_chain_type_components::traits::fields::message_response_events::HasMessageResponseEvents;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_relayer_components::chain::traits::send_message::EmptyMessageResponse;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use hermes_relayer_components::error::impls::error::MaxRetryExceededError;
use hermes_relayer_components::error::traits::retry::{HasMaxErrorRetry, HasRetryableError};
use hermes_relayer_components::relay::impls::packet_relayers::general::full_relay::LogRelayPacketAction;
use hermes_relayer_components::relay::impls::packet_relayers::general::lock::LogSkipRelayLockedPacket;
use hermes_relayer_components::relay::impls::packet_relayers::general::log::LogRelayPacketStatus;
use hermes_relayer_components::relay::traits::chains::HasRelayChains;
use hermes_relayer_components::relay::traits::packet_filter::PacketFilter;
use hermes_relayer_components::relay::traits::packet_lock::HasPacketLock;
use hermes_relayer_components::relay::traits::packet_relayer::CanRelayPacket;

use crate::components::extra::closures::chain::packet_relayer::UseExtraChainComponentsForPacketRelayer;
use crate::components::extra::closures::relay::message_sender::UseExtraIbcMessageSender;
use crate::components::extra::relay::DelegatesToExtraRelayComponents;

pub trait CanUseExtraPacketRelayer: UseExtraPacketRelayer {}

pub trait UseExtraPacketRelayer: CanRelayPacket {}

impl<Relay, SrcChain, DstChain, Components, Logger> UseExtraPacketRelayer for Relay
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasLogger<Logger = Logger>
        + HasPacketLock
        + UseExtraIbcMessageSender
        + HasRetryableError
        + HasMaxErrorRetry
        + CanRaiseError<EmptyMessageResponse>
        + for<'a> CanRaiseError<MaxRetryExceededError<'a, Relay>>
        + HasComponents<Components = Components>,
    SrcChain: HasOutgoingPacketType<DstChain> + UseExtraChainComponentsForPacketRelayer<DstChain>,
    DstChain: UseExtraChainComponentsForPacketRelayer<SrcChain>
        + HasMessageResponseEvents
        + HasWriteAckEvent<SrcChain>,
    DstChain::Timeout: Ord,
    Logger: for<'a> CanLog<LogSkipRelayLockedPacket<'a, Relay>>
        + for<'a> CanLog<LogRelayPacketAction<'a, Relay>>
        + for<'a> CanLog<LogRelayPacketStatus<'a, Relay>>,
    Components: DelegatesToExtraRelayComponents + PacketFilter<Relay>,
{
}
