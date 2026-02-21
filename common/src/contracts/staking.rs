use alloy::{
    network::EthereumWallet,
    providers::{RootProvider, fillers::*},
    sol,
};

use crate::{contracts::network::Network, utils::GeneralProvider};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Staking,
    "./src/contracts/abis/Staking.abi.json"
);

pub struct StakingContract {
    pub instance: StakingContractType,
}

impl StakingContract {
    pub fn new(network: Network, provider: GeneralProvider) -> Result<Self, anyhow::Error> {
        Ok(StakingContract {
            instance: Staking::new(network.staking_address()?, provider),
        })
    }
}

type StakingContractType = Staking::StakingInstance<
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
