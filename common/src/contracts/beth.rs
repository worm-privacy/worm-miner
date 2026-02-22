use crate::{
    contracts::{beth::BETH::MintParams, network::Network},
    mint::proof_generator::RapidsnarkOutput,
    utils::GeneralProvider,
};
use alloy::{
    network::EthereumWallet,
    primitives::{Address, Bytes, U256},
    providers::{RootProvider, fillers::*},
    rpc::types::TransactionReceipt,
    sol,
};
use anyhow::anyhow;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    BETH,
    "./src/contracts/abis/BETH.abi.json"
);

pub struct BETHContract {
    pub instance: BETHContractType,
}

impl BETHContract {
    pub fn new(network: Network, provider: GeneralProvider) -> Result<Self, anyhow::Error> {
        Ok(BETHContract {
            instance: BETH::new(network.beth_address()?, provider),
        })
    }

    pub async fn mint(
        &self,
        proof: RapidsnarkOutput,
        block_number: U256,
        nullifier: U256,
        remaining_coin_hash: U256,
        broadcaster_fee: U256,
        spend: U256,
        receiver: Address,
        prover_fee: U256,
        prover: Address,
        receiver_post_mint_hook: Bytes,
        broadcaster_fee_post_mint_hook: Bytes,
        prover_fee_post_mint_hook: Bytes,
    ) -> Result<U256, anyhow::Error> {
        let params = MintParams {
            pA: [proof.proof.pi_a[0], proof.proof.pi_a[1]],
            pB: [
                [proof.proof.pi_b[0][1], proof.proof.pi_b[0][0]],
                [proof.proof.pi_b[1][1], proof.proof.pi_b[1][0]],
            ],
            pC: [proof.proof.pi_c[0], proof.proof.pi_c[1]],
            blockNumber: block_number,
            nullifier,
            remainingCoin: remaining_coin_hash,
            broadcasterFee: broadcaster_fee,
            revealedAmount: spend,
            revealedAmountReceiver: receiver,
            proverFee: prover_fee,
            prover,
            receiverPostMintHook: receiver_post_mint_hook,
            broadcasterFeePostMintHook: broadcaster_fee_post_mint_hook,
            proverFeePostMintHook: prover_fee_post_mint_hook,
        };

        let trx = self.instance.mintCoin(params).send().await?;
        // we are not waiting for receipt
        // we return trx-hash to client so client can wait for receipt
        Ok(U256::from_be_slice(trx.tx_hash().as_slice()))
    }
}

type BETHContractType = BETH::BETHInstance<
    FillProvider<
        JoinFill<
            JoinFill<
                alloy::providers::Identity,
                JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
            >,
            WalletFiller<EthereumWallet>,
        >,
        RootProvider,
    >,
>;
