use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_ibc_components::traits::types::payload::data::HasPayloadDataType;

#[derive_component(IbcTransferReceiverGetterComponent, IbcTransferReceiverGetter<Chain>)]
pub trait HasIbcTransferReceiver<Counterparty, App>: HasPayloadDataType<Counterparty, App>
where
    Counterparty: HasAddressType,
{
    fn ibc_transfer_receiver(payload_data: &Self::PayloadData) -> &Counterparty::Address;
}

impl<Chain, Counterparty, App, Provider> IbcTransferReceiverGetter<Chain, Counterparty, App>
    for WithProvider<Provider>
where
    Chain: HasPayloadDataType<Counterparty, App>,
    Counterparty: HasAddressType,
    Provider: FieldGetter<Chain::PayloadData, symbol!("receiver"), Field = Counterparty::Address>,
{
    fn ibc_transfer_receiver(payload_data: &Chain::PayloadData) -> &Counterparty::Address {
        Provider::get_field(payload_data, PhantomData)
    }
}
