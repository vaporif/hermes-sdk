use cgp::prelude::*;
use hermes_chain_type_components::traits::types::time::HasTimeType;

use crate::traits::types::packet::timeout::HasPacketTimeoutType;

#[derive_component(TimeoutTimeComparerComponent, TimeoutTimeComparer<Chain>)]
pub trait CanCompareTimeoutTime<Counterparty>:
    HasTimeType + HasPacketTimeoutType<Counterparty>
{
    fn is_packet_timed_out(current_time: &Self::Time, timeout: &Self::PacketTimeout) -> bool;
}