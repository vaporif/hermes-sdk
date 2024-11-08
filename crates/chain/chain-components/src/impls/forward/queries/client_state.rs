use core::marker::PhantomData;

use cgp::core::error::CanRaiseError;
use cgp::core::inner::HasInner;

use crate::traits::queries::client_state::{
    CanQueryClientState, CanQueryClientStateWithProofs, ClientStateQuerier,
    ClientStateWithProofsQuerier,
};
use crate::traits::types::client_state::HasClientStateType;
use crate::traits::types::ibc::HasIbcChainTypes;
use crate::traits::types::proof::HasCommitmentProofType;

pub struct ForwardQueryClientState;

impl<Chain, InChain, Counterparty, ClientState> ClientStateQuerier<Chain, Counterparty>
    for ForwardQueryClientState
where
    Chain:
        HasInner<Inner = InChain> + CanRaiseError<InChain::Error> + HasIbcChainTypes<Counterparty>,
    InChain: CanQueryClientState<Counterparty, ClientId = Chain::ClientId, Height = Chain::Height>,
    Counterparty: HasClientStateType<Chain, ClientState = ClientState>
        + HasClientStateType<InChain, ClientState = ClientState>,
{
    async fn query_client_state(
        chain: &Chain,
        tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
        height: &Chain::Height,
    ) -> Result<ClientState, Chain::Error> {
        let client_state = chain
            .inner()
            .query_client_state(tag, client_id, height)
            .await
            .map_err(Chain::raise_error)?;

        Ok(client_state)
    }
}

impl<Chain, InChain, Counterparty, ClientState, CommitmentProof>
    ClientStateWithProofsQuerier<Chain, Counterparty> for ForwardQueryClientState
where
    Chain: HasInner<Inner = InChain>
        + CanRaiseError<InChain::Error>
        + HasIbcChainTypes<Counterparty>
        + HasCommitmentProofType<CommitmentProof = CommitmentProof>,
    InChain: CanQueryClientStateWithProofs<
        Counterparty,
        ClientId = Chain::ClientId,
        Height = Chain::Height,
        CommitmentProof = CommitmentProof,
    >,
    Counterparty: HasClientStateType<Chain, ClientState = ClientState>
        + HasClientStateType<InChain, ClientState = ClientState>,
{
    async fn query_client_state_with_proofs(
        chain: &Chain,
        tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
        height: &Chain::Height,
    ) -> Result<(ClientState, CommitmentProof), Chain::Error> {
        let result = chain
            .inner()
            .query_client_state_with_proofs(tag, client_id, height)
            .await
            .map_err(Chain::raise_error)?;

        Ok(result)
    }
}
