use crate::contracts::network::Network;
use alloy::{
    network::EthereumWallet,
    primitives::*,
    providers::{
        ProviderBuilder, RootProvider,
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
    },
    signers::local::PrivateKeySigner,
    sol,
    sol_types::{SolCall, SolValue},
};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    BETHToETH,
    "./src/contracts/abis/BETHToETH.abi.json"
);

pub struct BETHToETHContract {
    pub instance: BETHToETHContractType,
}

impl BETHToETHContract {
    pub async fn new(network: Network, signer: PrivateKeySigner) -> Result<Self, anyhow::Error> {
        let provider = ProviderBuilder::new()
            .wallet(signer)
            .connect(network.url())
            .await?;
        Ok(BETHToETHContract {
            instance: BETHToETH::new(network.beth_to_eth_address()?, provider),
        })
    }

    pub fn create_swap_hook(
        network: Network,
        swap_amount: U256,
        recipient: Address,
    ) -> Result<Bytes, anyhow::Error> {
        let calldata = Self::create_swap_beth_with_eth_calldata(swap_amount, recipient);

        Ok((network.beth_to_eth_address()?, swap_amount, calldata)
            .abi_encode_params()
            .into())
    }

    pub fn create_swap_beth_with_eth_calldata(swap_amount: U256, recipient: Address) -> Bytes {
        BETHToETH::swapBethWithEthCall {
            _swapAmount: swap_amount,
            _recipient: recipient,
        }
        .abi_encode()
        .into()
    }
}

type BETHToETHContractType = BETHToETH::BETHToETHInstance<
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

#[cfg(test)]
mod tests {
    use alloy::primitives::utils::parse_ether;

    use super::*;

    #[test]
    fn swap_hook_test() {
        assert_eq!(
            BETHToETHContract::create_swap_hook(
                Network::Mainnet,
                parse_ether("0.0005").unwrap(),
                address!("0x59Cd66cbb73D99e126Fa557E588f3d8dE8011b2C")
            )
            .unwrap()
            .to_string(),
            "0x000000000000000000000000ba5a285806c343aad955a40fe4b6e5e607b752b60000000000000000000000000000000000000000000000000001c6bf52634000000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000441d20dd830000000000000000000000000000000000000000000000000001c6bf5263400000000000000000000000000059cd66cbb73d99e126fa557e588f3d8de8011b2c00000000000000000000000000000000000000000000000000000000".to_string()
        )
    }
}
