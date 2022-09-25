mod cast;
mod commands;

use crate::cast::{Cast, SimpleCast};
use crate::commands::{App, Commands};

use clap::Parser;
use eyre::Result;
use reqwest::Url;
use starknet::providers::jsonrpc::models::{BlockId, BlockTag};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = App::parse();

    match &cli.command {
        Commands::AddressZero => {
            println!("{}", SimpleCast::address_zero());
        }

        Commands::DecToHex { dec } => {
            println!("{}", SimpleCast::to_hex(dec)?);
        }

        Commands::Ecdsa { commands } => match commands {
            commands::EcdsaCommand::Sign {
                private_key,
                message,
            } => {
                let private_key = private_key.to_owned().unwrap();
                let signature = SimpleCast::ecdsa_sign(&private_key, &message)?;
                println!("{} {}", signature.r, signature.s);
            }

            commands::EcdsaCommand::Verify {
                public_key,
                message,
                signature_r,
                signature_s,
            } => {
                let public_key = public_key.to_owned().unwrap();
                let is_valid =
                    SimpleCast::ecdsa_verify(&public_key, &message, &signature_r, &signature_s)?;
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
            hash,
            number,
            full,
            field,
            rpc_url,
        } => {
            let id = if let Some(hash) = hash {
                BlockId::Hash(hash.to_owned())
            } else if number.is_some() {
                BlockId::Number(number.unwrap())
            } else {
                BlockId::Tag(BlockTag::Latest)
            };

            let block = Cast::new(Url::parse(rpc_url.as_str())?)
                .block(id, full.clone(), field.to_owned())
                .await?;

            println!("{}", block)
        }

        Commands::Age {
            hash,
            number,
            rpc_url,
        } => {
            let id = if let Some(hash) = hash {
                BlockId::Hash(hash.to_owned())
            } else if number.is_some() {
                BlockId::Number(number.unwrap())
            } else {
                BlockId::Tag(BlockTag::Latest)
            };

            let timestamp = Cast::new(Url::parse(rpc_url.as_str())?)
                .block(id, false, Some("timestamp".to_string()))
                .await?;

            println!("{}", timestamp);
        }

        Commands::CountTransactions {
            hash,
            number,
            rpc_url,
        } => {
            let id = if let Some(hash) = hash {
                BlockId::Hash(hash.to_owned())
            } else if number.is_some() {
                BlockId::Number(number.unwrap())
            } else {
                BlockId::Tag(BlockTag::Latest)
            };

            let total = Cast::new(Url::parse(&rpc_url)?)
                .get_block_transaction_count(id)
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
    }

    Ok(())
}
