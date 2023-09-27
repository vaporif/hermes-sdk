use async_trait::async_trait;
use cgp_macros::derive_component;

use crate::relay::traits::chains::HasRelayChains;
use crate::std_prelude::*;

#[derive_component(PacketFilterComponent, PacketFilter<Relay>)]
#[async_trait]
pub trait CanFilterPackets: HasRelayChains {
    async fn should_relay_packet(&self, packet: &Self::Packet) -> Result<bool, Self::Error>;
}
