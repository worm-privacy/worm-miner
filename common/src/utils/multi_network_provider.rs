use alloy::{
    network::EthereumWallet,
    providers::{
        ProviderBuilder, RootProvider,
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
    },
    signers::local::PrivateKeySigner,
};

use crate::contracts::network::Network;

#[derive(Clone, Debug)]
pub struct MultiNetworkProvider {
    anvil: GeneralProvider,
    sepolia: GeneralProvider,
    mainnet: GeneralProvider,
}

impl MultiNetworkProvider {
    pub async fn new(signer: PrivateKeySigner) -> Result<Self, anyhow::Error> {
        let anvil = ProviderBuilder::new()
            .wallet(signer.clone())
            .connect(Network::Anvil.url())
            .await?;

        let sepolia = ProviderBuilder::new()
            .wallet(signer.clone())
            .connect(Network::Sepolia.url())
            .await?;

        let mainnet = ProviderBuilder::new()
            .wallet(signer.clone())
            .connect(Network::Mainnet.url())
            .await?;

        Ok(Self {
            anvil: anvil,
            sepolia: sepolia,
            mainnet: mainnet,
        })
    }

    pub fn get_provider(&self, network: Network) -> GeneralProvider {
        match network {
            Network::Anvil => self.anvil.clone(),
            Network::Sepolia => self.sepolia.clone(),
            Network::Mainnet => self.mainnet.clone(),
        }
    }
}

pub type GeneralProvider = FillProvider<
    JoinFill<
        JoinFill<
            alloy::providers::Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
>;
