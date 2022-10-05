#![allow(warnings)]

mod cast;
mod cli;

use crate::cast::{Cast, SimpleCast};
use crate::cli::commands::{App, Commands, EcdsaCommand, RpcArgs};

use cast::utils; use clap::Parser;
use eyre::Result;
use reqwest::Url;
use starknet::providers::jsonrpc::models::BlockId;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = App::parse();

    // println!("{:?}", cli.command);

    match &cli.command {
        Commands::AddressZero => {
            println!("{}", SimpleCast::address_zero());
        }

        Commands::DecToHex { dec } => {
            println!("{}", SimpleCast::to_hex(dec));
        }

        Commands::Ecdsa { commands } => match commands {
            EcdsaCommand::Sign {
                message,
                private_key,
            } => {
                let key = private_key.to_owned();
                let signature = SimpleCast::ecdsa_sign(&key, &message)?;
                println!("{:#x} {:#x}", signature.r, signature.s);
            }

            EcdsaCommand::Verify {
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
            println!("{}", SimpleCast::to_dec(hex));
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

        Commands::BlockNumber { starknet } => {
            let res = Cast::new(Url::parse(starknet.rpc_url.as_str())?)
                .block_number()
                .await?;
            println!("{:?}", res);
        }

        Commands::ChainId { starknet } => {
            let chain_id = Cast::new(Url::parse(starknet.rpc_url.as_str())?).chain_id().await?;
            println!("{}", chain_id);
        }

        Commands::Transaction {
            hash,
            field,
            starknet,
        } => {
            let res = Cast::new(Url::parse(starknet.rpc_url.as_str())?)
                .get_transaction_by_hash(hash.to_owned(), field.to_owned())
                .await?;
            println!("{}", res);
        }

        Commands::TransactionStatus { hash, starknet } => {
            let res = Cast::new(Url::parse(&starknet.rpc_url)?)
                .get_transaction_by_hash(hash.to_owned(), Some("status".to_string()))
                .await?;
            println!("{}", res);
        }

        Commands::TransactionReceipt {
            hash,
            field,
            starknet,
        } => {
            let res = Cast::new(Url::parse(&starknet.rpc_url)?)
                .get_transaction_receipt(hash.to_owned(), field.to_owned())
                .await?;
            println!("{}", res);
        }

        Commands::Block {
            id,
            full,
            field,
            starknet,
        } => {
            let block = Cast::new(Url::parse(starknet.rpc_url.as_str())?)
                .block(id.to_owned(), full.clone(), field.to_owned())
                .await?;

            println!("{}", block)
        }

        Commands::Age { id, starknet } => {
            let timestamp = Cast::new(Url::parse(starknet.rpc_url.as_str())?)
                .block(id.to_owned(), false, Some("timestamp".to_string()))
                .await?;

            println!("{}", timestamp);
        }

        Commands::CountTransactions { id, starknet } => {
            let total = Cast::new(Url::parse(&starknet.rpc_url)?)
                .get_block_transaction_count(id.to_owned())
                .await?;

            println!("{}", total);
        }

        Commands::Nonce {
            contract_address,
            starknet,
        } => {
            let nonce = Cast::new(Url::parse(&starknet.rpc_url)?)
                .get_nonce(contract_address.to_owned())
                .await?;
            println!("{}", nonce);
        }

        Commands::PendingTransactions { starknet } => {
            let transactions = Cast::new(Url::parse(&starknet.rpc_url)?)
                .pending_transactions()
                .await?;
            println!("{}", transactions);
        }

        Commands::Storage {
            contract_address,
            key,
            hash,
            number,
            starknet,
        } => {
            let block_id = if let Some(hash) = hash {
                BlockId::Hash(hash.to_owned())
            } else {
                BlockId::Number(number.unwrap())
            };

            let res = Cast::new(Url::parse(&starknet.rpc_url)?)
                .get_storage_at(contract_address.to_owned(), key.to_owned(), block_id)
                .await?;

            println!("{}", res);
        }

        Commands::Rpc(rpc_args) => {
            let res = rpc_args.to_owned().run().await?;
            println!("{}", res);
        }

        Commands::Call {
            function_name,
            abi,
            inputs,
            contract_address,
            block_id,
            starknet
        } => {
            let expected_params_count = utils::count_function_inputs_from_abi(abi, function_name)?;
            let inputs = inputs.to_owned();
            let len = inputs.len();

            if expected_params_count == len as u8 {
                let res = Cast::new(Url::parse(&starknet.rpc_url)?)
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

        Commands::StateUpdate { block_id, starknet } => {
            let url = Url::parse(&starknet.rpc_url)?;
            let res = Cast::new(url).get_state_update(block_id).await?;
            println!("{}", res);
        }

        _ => {
            println!("{:?}", cli.command);
        }
    }

    Ok(())
}
