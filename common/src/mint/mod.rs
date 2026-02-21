pub mod error;
pub mod note_json;
pub mod nullifier;
pub mod proof_generator;
pub mod remaining_coin_hash;
pub mod witness_generator;
pub mod witness_input_file;

use alloy::{
    primitives::{Address, Bytes, U256},
    signers::local::PrivateKeySigner,
};

use crate::{
    burn::{burn_address::burn_address, burn_output::BurnOutput},
    contracts::beth::BETHContract,
    mint::{
        error::MintError,
        note_json::NoteJson,
        nullifier::compute_nullifier,
        proof_generator::generate_proof,
        remaining_coin_hash::compute_remaining_coin,
        witness_generator::generate_witness,
        witness_input_file::{WitnessInputFile, WitnessInputFileError},
    },
    utils::{ToU256, TryToFr},
};
use alloy::{
    eips::BlockId,
    providers::{Provider, ProviderBuilder},
};
use anyhow::anyhow;

/// [singer] who calls mint() of BETH and pays gas fee
/// [prover_address] who gets prover_fee
pub async fn mint(
    burn_output: BurnOutput,
    prover_address: Address,
    signer: PrivateKeySigner,
) -> Result<NoteJson, MintError> {
    let burn_extra_commitment = burn_output.extra_commitment;

    let burn_key = burn_output
        .burn_key
        .try_to_fr()
        .map_err(|e| anyhow!("burn_key.try_to_fr() {e}"))?;

    let burn_address = burn_address(
        burn_key,
        burn_output
            .reveal_amount
            .try_to_fr()
            .map_err(|e| anyhow!("{e}"))?,
        burn_extra_commitment.clone(),
    )?;

    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(burn_output.network.url())
        .await?;

    // making sure that proof and block_header are pointing to same block
    let (witness_input_file, block_number) = loop {
        let block = provider
            .get_block(BlockId::latest())
            .await?
            .ok_or(anyhow!("block not found"))?;
        let block_number = block.number();
        let proof = provider.get_proof(burn_address, vec![]).await?;

        let result = WitnessInputFile::new(
            proof,
            block.header.inner,
            burn_key,
            burn_output.reveal_amount,
            burn_extra_commitment.hash().map_err(|e| anyhow!("{e}"))?,
            prover_address,
        );
        match result {
            // retry to get proof and block again
            Err(WitnessInputFileError::NotOnSameBlock) => continue,
            // unrecoverable error
            Err(WitnessInputFileError::Other(e)) => return Err(e)?,
            // actual valid witness_file
            Ok(x) => break (x, block_number),
        }
    };

    println!("Generating witness...");
    let witness_file = generate_witness(witness_input_file, burn_address)?;

    println!("Generating proof...");
    let proof = generate_proof(witness_file)?;

    println!("Proof:\n{}", proof.to_json());

    let nullifier = compute_nullifier(burn_key)?;
    let remaining_coin =
        compute_remaining_coin(burn_key, burn_output.burn_amount, burn_output.reveal_amount)?;

    let beth = BETHContract::new(burn_output.network, provider)?;

    beth.mint(
        proof,
        U256::from(block_number),
        nullifier.to_u256(),
        remaining_coin.to_u256(),
        burn_extra_commitment.broadcaster_fee,
        burn_output.reveal_amount,
        burn_extra_commitment.receiver,
        burn_extra_commitment.prover_fee,
        prover_address,
        burn_output.receiver_hook,
        Bytes::new(),
        Bytes::new(),
    )
    .await?;

    Ok(NoteJson::new(
        burn_output.burn_key,
        burn_output.burn_amount - burn_output.reveal_amount,
        burn_output.network,
    ))
}
