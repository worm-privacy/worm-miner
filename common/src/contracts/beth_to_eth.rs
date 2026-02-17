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

        Ok(<(Address, U256, Bytes) as SolValue>::abi_encode(&(
            network.beth_to_eth_address()?,
            swap_amount,
            calldata.clone(),
        ))
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
