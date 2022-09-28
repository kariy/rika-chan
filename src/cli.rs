#![allow(warnings)]

mod cast;
mod commands;

use crate::cast::{Cast, SimpleCast};
use crate::commands::{App, Commands};

use cast::utils;
use clap::Parser;
use eyre::Result;
use reqwest::Url;
use starknet::providers::jsonrpc::models::BlockId;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = App::parse();

    // println!("{:?}", cli.command);

    match &cli.command {
        Commands::AddressZero => {
            println!("{}", SimpleCast::address_zero());
        }

        Commands::DecToHex { dec } => {
            println!("{}", SimpleCast::to_hex(dec)?);
        }

        Commands::Ecdsa { commands } => match commands {
            commands::EcdsaCommand::Sign {
                message,
                private_key,
            } => {
                let key = private_key.to_owned();
                let signature = SimpleCast::ecdsa_sign(&key, &message)?;
                println!("{} {}", signature.r, signature.s);
            }

            commands::EcdsaCommand::Verify {
                message,
                signature,
                verifying_key,
            } => {
                let key = verifying_key.to_owned();
                let is_valid =
                    SimpleCast::ecdsa_verify(&key, &message, &signature[0], &signature[1])?;
                println!("{}", is_valid);
            }
        },

        Commands::FromUtf8 { felt } => {
            println!("{}", SimpleCast::from_utf8(felt)?);
        }

        Commands::HexToDec { hex } => {
            println!("{}", SimpleCast::to_dec(hex)?);
        }

        Commands::Keccak { data } => {
            println!("{}", SimpleCast::keccak(data)?);
        }

        Commands::MaxSignedFelt => {
            println!("{}", SimpleCast::max_signed_felt());
        }

        Commands::MinSignedFelt => {
            println!("{}", SimpleCast::min_signed_felt())
        }

        Commands::StrToFelt { str } => {
            println!("{}", SimpleCast::str_to_felt(str)?);
        }

        Commands::MaxUnsignedFelt => {
            println!("{}", SimpleCast::max_felt());
        }

        Commands::Pedersen { x, y } => {
            println!("{}", SimpleCast::pedersen(x, y)?);
        }

        Commands::BlockNumber { rpc_url } => {
            let res = Cast::new(Url::parse(rpc_url.as_str())?)
                .block_number()
                .await?;
            println!("{:?}", res);
        }

        Commands::ChainId { rpc_url } => {
            let chain_id = Cast::new(Url::parse(rpc_url.as_str())?).chain_id().await?;
            println!("{}", chain_id);
        }

        Commands::Transaction {
            hash,
            field,
            rpc_url,
        } => {
            let res = Cast::new(Url::parse(rpc_url.as_str())?)
                .get_transaction_by_hash(hash.to_owned(), field.to_owned())
                .await?;
            println!("{}", res);
        }

        Commands::TransactionStatus { hash, rpc_url } => {
            let res = Cast::new(Url::parse(&rpc_url)?)
                .get_transaction_by_hash(hash.to_owned(), Some("status".to_string()))
                .await?;
            println!("{}", res);
        }

        Commands::TransactionReceipt {
            hash,
            field,
            rpc_url,
        } => {
            let res = Cast::new(Url::parse(&rpc_url)?)
                .get_transaction_receipt(hash.to_owned(), field.to_owned())
                .await?;
            println!("{}", res);
        }

        Commands::Block {
            id,
            full,
            field,
            rpc_url,
        } => {
            let block = Cast::new(Url::parse(rpc_url.as_str())?)
                .block(id.to_owned(), full.clone(), field.to_owned())
                .await?;

            println!("{}", block)
        }

        Commands::Age { id, rpc_url } => {
            let timestamp = Cast::new(Url::parse(rpc_url.as_str())?)
                .block(id.to_owned(), false, Some("timestamp".to_string()))
                .await?;

            println!("{}", timestamp);
        }

        Commands::CountTransactions { id, rpc_url } => {
            let total = Cast::new(Url::parse(&rpc_url)?)
                .get_block_transaction_count(id.to_owned())
                .await?;

            println!("{}", total);
        }

        Commands::Nonce {
            contract_address,
            rpc_url,
        } => {
            let nonce = Cast::new(Url::parse(&rpc_url)?)
                .get_nonce(contract_address.to_owned())
                .await?;
            println!("{}", nonce);
        }

        Commands::PendingTransactions { rpc_url } => {
            let transactions = Cast::new(Url::parse(&rpc_url)?)
                .pending_transactions()
                .await?;
            println!("{}", transactions);
        }

        Commands::Storage {
            contract_address,
            key,
            hash,
            number,
            rpc_url,
        } => {
            let block_id = if let Some(hash) = hash {
                BlockId::Hash(hash.to_owned())
            } else {
                BlockId::Number(number.unwrap())
            };

            let res = Cast::new(Url::parse(&rpc_url)?)
                .get_storage_at(contract_address.to_owned(), key.to_owned(), block_id)
                .await?;

            println!("{}", res);
        }

        Commands::Rpc {
            method,
            params,
            file,
            rpc_url,
        } => {
            let params = if let Some(path) = file {
                let content = fs::read_to_string(Path::new(path))?;
                serde_json::from_str(&content)?
            } else {
                let params = params.clone().unwrap();
                serde_json::Value::Array(
                    params
                        .into_iter()
                        .map(|value| {
                            serde_json::from_str(&value)
                                .unwrap_or(serde_json::Value::String(value.to_owned()))
                        })
                        .collect(),
                )
            };

            let res = Cast::rpc(Url::parse(&rpc_url)?, &method, &params).await?;
            println!("{}", res);
        }

        Commands::Call {
            function_name,
            abi,
            inputs,
            contract_address,
            block_id,
            rpc_url,
        } => {
            let expected_params_count = utils::count_function_inputs_from_abi(abi, function_name)?;
            let inputs = inputs.to_owned();
            let len = inputs.len();

            if expected_params_count == len as u8 {
                let res = Cast::new(Url::parse(&rpc_url)?)
                    .call(contract_address, function_name, inputs, block_id)
                    .await?;

                println!("{}", res);
            } else {
                return Err(eyre::eyre!(
                    "expected {} inputs but got {}.",
                    expected_params_count,
                    len,
                ));
            }
        }

        _ => {
            println!("{:?}", cli.command);
        }
    }

    Ok(())
}
