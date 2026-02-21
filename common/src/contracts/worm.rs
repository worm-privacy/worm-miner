use alloy::{
    network::EthereumWallet,
    providers::{RootProvider, fillers::*},
    sol,
};

use crate::{contracts::network::Network, utils::GeneralProvider};

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
