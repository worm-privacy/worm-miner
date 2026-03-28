use crate::{contracts::network::Network, utils::GeneralProvider};
use alloy::{
    network::EthereumWallet,
    primitives::U256,
    providers::{RootProvider, fillers::*},
    rpc::types::TransactionReceipt,
    sol,
};
use anyhow::anyhow;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Worm,
    "./src/contracts/abis/WORM.abi.json"
);

pub struct WormContract {
    pub instance: WormContractType,
}

impl WormContract {
    pub fn new(network: Network, provider: GeneralProvider) -> Result<Self, anyhow::Error> {
        Ok(WormContract {
            instance: Worm::new(network.worm_address()?, provider),
        })
    }

    pub async fn participate(
        &self,
        amount_per_epoch: U256,
        number_of_epochs: U256,
    ) -> Result<TransactionReceipt, anyhow::Error> {
        let trx = self
            .instance
            .participate(amount_per_epoch, number_of_epochs)
            .send()
            .await?;
        let receipt = trx.get_receipt().await?;
        if !receipt.status() {
            println!("receipt: {:?}", receipt);
            return Err(anyhow!("transaction mined but reverted"));
        }
        Ok(receipt)
    }

    pub async fn current_epoch(&self) -> Result<U256, anyhow::Error> {
        Ok(self.instance.currentEpoch().call().await?)
    }
}

type WormContractType = Worm::WormInstance<
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
