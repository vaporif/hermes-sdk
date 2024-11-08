use alloc::vec::Vec;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

use crate::traits::types::height::HasHeightType;
use crate::traits::types::ibc::HasIbcChainTypes;

#[derive_component(ConsensusStateHeightQuerierComponent, ConsensusStateHeightQuerier<Chain>)]
#[async_trait]
pub trait CanQueryConsensusStateHeight<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasHeightType,
{
    /**
       Query the chain to find a consensus state that has a height that is
       less than or equal the target height. This is needed as a base trusted
       height to build the headers for UpdateClient.

       Invariant: the returned height must be less than or equal to the given
       target height.
    */
    async fn find_consensus_state_height_before(
        &self,
        client_id: &Self::ClientId,
        target_height: &Counterparty::Height,
    ) -> Result<Counterparty::Height, Self::Error>;
}

#[derive_component(ConsensusStateHeightsQuerierComponent, ConsensusStateHeightsQuerier<Chain>)]
#[async_trait]
pub trait CanQueryConsensusStateHeights<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasHeightType,
{
    async fn query_consensus_state_heights(
        &self,
        client_id: &Self::ClientId,
    ) -> Result<Vec<Counterparty::Height>, Self::Error>;
}

impl<Chain, Counterparty, Components, Delegate> ConsensusStateHeightsQuerier<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasIbcChainTypes<Counterparty> + HasErrorType,
    Counterparty: HasHeightType,
    Delegate: ConsensusStateHeightsQuerier<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn query_consensus_state_heights(
        chain: &Chain,
        client_id: &Chain::ClientId,
    ) -> Result<Vec<Counterparty::Height>, Chain::Error> {
        Delegate::query_consensus_state_heights(chain, client_id).await
    }
}
