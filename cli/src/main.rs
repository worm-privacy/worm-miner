pub mod arg_parser;
pub mod fs;

use crate::{arg_parser::Args, fs::ParticipateOutputJson};
use alloy::{primitives::U256, providers::ProviderBuilder};
use clap::Parser;
use common::{
    burn::{burn, burn_output::BurnOutput},
    contracts::worm::WormContract,
    mint::mint,
};
use std::{path::PathBuf, process::exit, str::FromStr};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        arg_parser::Commands::Burn {
            private_key,
            amount,
            reveal,
            broadcaster_fee,
            sell_for_eth,
            receiver_address,
            prover_fee,
            out: out_file,
            network,
        } => {
            let (out, burn_address) = match burn(
                network,
                private_key,
                amount,
                reveal.unwrap_or(amount),
                broadcaster_fee,
                sell_for_eth,
                receiver_address,
                prover_fee,
            )
            .await
            {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("burn command failed: {}", e);
                    exit(1);
                }
            };
            let json = out.to_json().expect("burn.json generation failed");
            println!("burn result: {}", json);

            let out_file = match out_file {
                Some(out_file) if out_file.is_dir() => {
                    out_file.join(format!("burn_{}.json", burn_address))
                }
                Some(out_file) if out_file.is_file() => out_file
                    .parent()
                    .unwrap()
                    .join(format!("burn_{}.json", burn_address)),
                _ => PathBuf::from_str(&format!("./burn_{}.json", burn_address)).unwrap(),
            };

            println!("writing burn result in: `{}`", out_file.to_str().unwrap());
            if let Err(e) = std::fs::write(out_file, &json) {
                println!(
                    "error `{}` while writing to file, your burn.json here \n{}",
                    e, json
                )
            }
        }
        arg_parser::Commands::Mint { file, broadcaster } => {
            let file = PathBuf::from(file);
            let content = match std::fs::read_to_string(file) {
                Ok(x) => x,
                Err(e) => {
                    println!("{e}");
                    exit(1);
                }
            };
            let burn_output = match BurnOutput::from_json(&content) {
                Ok(x) => x,
                Err(e) => {
                    println!("{e}");
                    exit(1);
                }
            };
            match broadcaster {
                // TODO send request to a third party prover
                common::burn::broadcaster::Broadcaster::EndPoint(_url) => todo!(),
                common::burn::broadcaster::Broadcaster::PrivateKey(local_signer) => {
                    // In self proving mode, prover is actually receiver in case user accidentally sets prover-fee not zero
                    let prover_address = burn_output.extra_commitment.receiver;
                    let burn_key = burn_output.burn_key.to_string();
                    let out = match mint(burn_output, prover_address, local_signer).await {
                        Ok(x) => x,
                        Err(e) => {
                            println!("mint error: {e}");
                            exit(1)
                        }
                    };
                    let path = PathBuf::from_str(&format!("./note_{}.json", burn_key))
                        .expect("can't make note.json path");
                    std::fs::write(path, out.to_json().expect("can't convert note to json"))
                        .expect("can't write note.json to file");
                }
            }
        }
        arg_parser::Commands::Spend {
            note: _,
            amount: _amount,
        } => todo!(),
        arg_parser::Commands::Participate {
            private_key,
            num_epochs,
            amount_per_epoch,
            network,
        } => {
            let provider = match ProviderBuilder::new()
                .wallet(private_key)
                .connect(network.url())
                .await
            {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("{e}");
                    exit(1);
                }
            };

            let worm = match WormContract::new(network, provider) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("{e}");
                    exit(1);
                }
            };

            let current_epoch = match worm.current_epoch().await {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("{e}");
                    exit(1);
                }
            };

            match worm
                .participate(amount_per_epoch, U256::from(num_epochs))
                .await
            {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("{e}");
                    exit(1);
                }
            };

            let path = PathBuf::from_str(&format!(
                "./participate_{}_{}.json",
                num_epochs, amount_per_epoch
            ))
            .expect("can't make note.json path");

            // +1 is for extra safety in case current currentEpoch call happens one epoch before participate call
            // so we don't miss last epoch reward
            let output = ParticipateOutputJson::new(current_epoch, num_epochs + 1)
                .to_json()
                .expect("can't convert note to json");

            println!("Participated successfully:\n{}", &output);

            std::fs::write(path.clone(), output).expect("can't write note.json to file");

            println!("Participation data saved in: {}", &path.to_str().unwrap());
        }
        arg_parser::Commands::Claim {
            private_key: _,
            participate_file: _,
        } => todo!(),
    };
}
