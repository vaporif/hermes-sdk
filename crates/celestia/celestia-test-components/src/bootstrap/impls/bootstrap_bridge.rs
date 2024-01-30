use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::chain::{HasChain, HasChainType};
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::runtime::traits::types::child_process::HasChildProcessType;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;

use crate::bootstrap::traits::bootstrap_bridge::BridgeBootstrapper;
use crate::bootstrap::traits::bridge_store_dir::HasBridgeStoreDir;
use crate::bootstrap::traits::build_bridge_driver::CanBuildBridgeDriver;
use crate::bootstrap::traits::import_bridge_key::CanImportBridgeKey;
use crate::bootstrap::traits::init_bridge_config::CanInitBridgeConfig;
use crate::bootstrap::traits::init_bridge_data::CanInitBridgeData;
use crate::bootstrap::traits::start_bridge::CanStartBridge;

pub struct BootstrapCelestiaBridge;

impl<Bootstrap, Chain, ChainDriver, Runtime> BridgeBootstrapper<Bootstrap>
    for BootstrapCelestiaBridge
where
    Bootstrap: HasChainType<Chain = Chain>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + HasRuntime<Runtime = Runtime>
        + HasBridgeStoreDir
        + CanInitBridgeData
        + CanImportBridgeKey
        + CanInitBridgeConfig
        + CanStartBridge
        + CanBuildBridgeDriver,
    ChainDriver: HasChain<Chain = Chain> + HasRuntime<Runtime = Runtime>,
    Chain: HasChainId,
    Runtime: HasFilePathType + HasChildProcessType,
{
    async fn bootstrap_bridge(
        bootstrap: &Bootstrap,
        chain_driver: &ChainDriver,
    ) -> Result<Bootstrap::BridgeDriver, Bootstrap::Error> {
        let chain_id = chain_driver.chain().chain_id();
        let bridge_store_dir = bootstrap.bridge_store_dir();

        let bridge_home_dir = Runtime::join_file_path(
            bridge_store_dir,
            &Runtime::file_path_from_string(&chain_id.to_string()),
        );

        bootstrap
            .init_bridge_data(&bridge_home_dir, chain_id)
            .await?;

        let bridge_config = bootstrap
            .init_bridge_config(&bridge_home_dir, chain_driver)
            .await?;

        bootstrap
            .import_bridge_key(&bridge_home_dir, chain_driver)
            .await?;

        let bridge_process = bootstrap
            .start_bridge(&bridge_home_dir, chain_driver)
            .await?;

        let bridge_driver = bootstrap
            .build_bridge_driver(bridge_config, bridge_process)
            .await?;

        Ok(bridge_driver)
    }
}