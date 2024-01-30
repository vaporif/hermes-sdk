use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use cgp_error_eyre::ProvideEyreError;
use cgp_error_eyre::RaiseDebugError;
use hermes_relayer_components::logger::traits::has_logger::LoggerFieldComponent;
use hermes_relayer_components::logger::traits::has_logger::LoggerTypeComponent;
use hermes_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use hermes_test_components::driver::traits::channel_at::ChannelGetterAt;
use hermes_test_components::driver::traits::types::birelay_at::BiRelayTypeAtComponent;
use hermes_test_components::driver::traits::types::chain_at::ChainTypeAtComponent;
use hermes_test_components::driver::traits::types::chain_driver_at::ChainDriverGetterAt;
use hermes_test_components::driver::traits::types::chain_driver_at::ChainDriverTypeAtComponent;
use hermes_test_components::driver::traits::types::relay_at::RelayTypeAtComponent;
use hermes_test_components::driver::traits::types::relay_driver_at::RelayDriverGetterAt;
use hermes_test_components::driver::traits::types::relay_driver_at::RelayDriverTypeAtComponent;
use hermes_test_components::types::index::Index;
use hermes_test_components::types::index::Twindex;
use ibc_relayer_types::core::ics24_host::identifier::PortId;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ConnectionId};

use crate::contexts::chain_driver::CosmosChainDriver;
use crate::contexts::relay_driver::CosmosRelayDriver;
use crate::impls::types::ProvideCosmosTestTypes;

pub struct CosmosBinaryChannelTestDriver {
    pub relay_driver: CosmosRelayDriver,
    pub chain_driver_a: CosmosChainDriver,
    pub chain_driver_b: CosmosChainDriver,
    pub connection_id_a: ConnectionId,
    pub connection_id_b: ConnectionId,
    pub channel_id_a: ChannelId,
    pub channel_id_b: ChannelId,
    pub port_id_a: PortId,
    pub port_id_b: PortId,
}

pub struct CosmosBinaryChannelTestDriverComponents;

impl HasComponents for CosmosBinaryChannelTestDriver {
    type Components = CosmosBinaryChannelTestDriverComponents;
}

delegate_components! {
    CosmosBinaryChannelTestDriverComponents {
        ErrorTypeComponent: ProvideEyreError,
        ErrorRaiserComponent: RaiseDebugError,
        [
            LoggerTypeComponent,
            LoggerFieldComponent,
        ]:
            ProvideTracingLogger,
        [
            ChainTypeAtComponent,
            ChainDriverTypeAtComponent,
            RelayTypeAtComponent,
            BiRelayTypeAtComponent,
            RelayDriverTypeAtComponent,
        ]:
            ProvideCosmosTestTypes,
    }
}

impl ChainDriverGetterAt<CosmosBinaryChannelTestDriver, 0>
    for CosmosBinaryChannelTestDriverComponents
{
    fn chain_driver_at(
        driver: &CosmosBinaryChannelTestDriver,
        _index: Index<0>,
    ) -> &CosmosChainDriver {
        &driver.chain_driver_a
    }
}

impl ChainDriverGetterAt<CosmosBinaryChannelTestDriver, 1>
    for CosmosBinaryChannelTestDriverComponents
{
    fn chain_driver_at(
        driver: &CosmosBinaryChannelTestDriver,
        _index: Index<1>,
    ) -> &CosmosChainDriver {
        &driver.chain_driver_b
    }
}

impl RelayDriverGetterAt<CosmosBinaryChannelTestDriver, 0, 1>
    for CosmosBinaryChannelTestDriverComponents
{
    fn relay_driver_at(
        driver: &CosmosBinaryChannelTestDriver,
        _index: Twindex<0, 1>,
    ) -> &CosmosRelayDriver {
        &driver.relay_driver
    }
}

impl ChannelGetterAt<CosmosBinaryChannelTestDriver, 0, 1>
    for CosmosBinaryChannelTestDriverComponents
{
    fn channel_id_at(driver: &CosmosBinaryChannelTestDriver, _index: Twindex<0, 1>) -> &ChannelId {
        &driver.channel_id_a
    }

    fn port_id_at(driver: &CosmosBinaryChannelTestDriver, _index: Twindex<0, 1>) -> &PortId {
        &driver.port_id_a
    }
}

impl ChannelGetterAt<CosmosBinaryChannelTestDriver, 1, 0>
    for CosmosBinaryChannelTestDriverComponents
{
    fn channel_id_at(driver: &CosmosBinaryChannelTestDriver, _index: Twindex<1, 0>) -> &ChannelId {
        &driver.channel_id_b
    }

    fn port_id_at(driver: &CosmosBinaryChannelTestDriver, _index: Twindex<1, 0>) -> &PortId {
        &driver.port_id_b
    }
}