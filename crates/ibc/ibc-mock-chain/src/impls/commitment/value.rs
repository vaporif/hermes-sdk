use alloc::string::String;

use cgp::core::Async;
use hermes_ibc_components::traits::commitment::value::receive_packet::ReceivePacketCommitmentValueBuilder;
use hermes_ibc_components::traits::commitment::value::send_packet::SendPacketCommitmentValueBuilder;
use hermes_ibc_components::types::packet::IbcPacket;

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;

impl<Chain: Async, Counterparty: Async>
    SendPacketCommitmentValueBuilder<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>
    for MockChainComponents
{
    fn build_send_packet_commitment_value(
        packet: &IbcPacket<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>,
    ) -> Result<IbcPacket<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>, String>
    {
        Ok(packet.clone())
    }
}

impl<Chain: Async, Counterparty: Async>
    ReceivePacketCommitmentValueBuilder<
        MockChain<Chain, Counterparty>,
        MockChain<Counterparty, Chain>,
    > for MockChainComponents
{
    fn build_receive_packet_commitment_value(
        packet: &IbcPacket<MockChain<Counterparty, Chain>, MockChain<Chain, Counterparty>>,
    ) -> Result<IbcPacket<MockChain<Counterparty, Chain>, MockChain<Chain, Counterparty>>, String>
    {
        Ok(packet.clone())
    }
}