use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(PacketHeaderTypeComponent, ProvidePacketHeaderType<Chain>)]
pub trait HasPacketHeaderType<Counterparty>: Async {
    type PacketHeader: Async;
}

impl<Chain, Counterparty, Provider, PacketHeader> ProvidePacketHeaderType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, PacketHeaderTypeComponent, Type = PacketHeader>,
    PacketHeader: Async,
{
    type PacketHeader = PacketHeader;
}
