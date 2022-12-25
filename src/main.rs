mod cast;
mod cli;

use crate::cast::{Probe, SimpleProbe};
use crate::cli::commands::{App, Commands, EcdsaCommand};

use clap::Parser;
use eyre::Result;
use starknet::core::utils::get_selector_from_name;
use starknet::providers::jsonrpc::models::{EventFilter, FunctionCall};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = App::parse();

    match &cli.command {
        Commands::DecToHex { dec } => {
            println!("{}", SimpleProbe::to_hex(dec));
        }

        Commands::Ecdsa { commands } => match commands {
            EcdsaCommand::Sign {
                message,
                private_key,
            } => {
                let key = private_key.to_owned();
                let signature = SimpleProbe::ecdsa_sign(&key, &message)?;
                println!("{:#x} {:#x}", signature.r, signature.s);
            }

            EcdsaCommand::Verify {
                message,
                signature,
                verifying_key,
            } => {
                let key = verifying_key.to_owned();
                let is_valid =
                    SimpleProbe::ecdsa_verify(&key, &message, &signature[0], &signature[1])?;
                println!("{}", is_valid);
            }
        },

        Commands::FromAscii { ascii } => {
            println!("{}", SimpleProbe::from_utf8(ascii)?);
        }

        Commands::HexToDec { hex } => {
            println!("{}", SimpleProbe::to_dec(hex));
        }

        Commands::Keccak { data } => {
            println!("{}", SimpleProbe::keccak(data)?);
        }

        Commands::MaxSignedFelt => {
            println!("{}", SimpleProbe::max_signed_felt());
        }

        Commands::MinSignedFelt => {
            println!("{}", SimpleProbe::min_signed_felt())
        }

        Commands::ToAscii { short_str } => {
            println!("{}", SimpleProbe::str_to_felt(short_str)?);
        }

        Commands::MaxUnsignedFelt => {
            println!("{}", SimpleProbe::max_felt());
        }

        Commands::Pedersen { x, y } => {
            println!("{}", SimpleProbe::pedersen(x, y)?);
        }

        Commands::BlockNumber { starknet } => {
            let res = Probe::new(starknet.rpc_url.to_owned())
                .block_number()
                .await?;
            println!("{:?}", res);
        }

        Commands::ChainId { starknet } => {
            let chain_id = Probe::new(starknet.rpc_url.to_owned()).chain_id().await?;
            println!("{}", chain_id);
        }

        Commands::Transaction {
            hash,
            field,
            starknet,
        } => {
            let res = Probe::new(starknet.rpc_url.to_owned())
                .get_transaction_by_hash(hash.to_owned(), field.to_owned())
                .await?;
            println!("{}", res);
        }

        Commands::TransactionStatus { hash, starknet } => {
            let res = Probe::new(starknet.rpc_url.to_owned())
                .get_transaction_receipt(hash.to_owned(), Some("status".to_string()))
                .await?;
            println!("{}", res);
        }

        Commands::TransactionReceipt {
            hash,
            field,
            starknet,
        } => {
            let res = Probe::new(starknet.rpc_url.to_owned())
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
            let block = Probe::new(starknet.rpc_url.to_owned())
                .block(id.to_owned(), full.clone(), field.to_owned())
                .await?;

            println!("{}", block)
        }

        Commands::Age { block_id, starknet } => {
            let timestamp = Probe::new(starknet.rpc_url.to_owned())
                .block(block_id.to_owned(), false, Some("timestamp".to_string()))
                .await?;

            println!("{}", timestamp);
        }

        Commands::CountTransactions { block_id, starknet } => {
            let total = Probe::new(starknet.rpc_url.to_owned())
                .get_block_transaction_count(block_id.to_owned())
                .await?;

            println!("{}", total);
        }

        Commands::Nonce {
            contract_address,
            block_id,
            starknet,
        } => {
            let nonce = Probe::new(starknet.rpc_url.to_owned())
                .get_nonce(contract_address.to_owned(), block_id)
                .await?;
            println!("{}", nonce);
        }

        Commands::PendingTransactions { starknet } => {
            let transactions = Probe::new(starknet.rpc_url.to_owned())
                .pending_transactions()
                .await?;
            println!("{}", transactions);
        }

        Commands::Storage {
            contract_address,
            index,
            block_id,
            starknet,
        } => {
            let res = Probe::new(starknet.rpc_url.to_owned())
                .get_storage_at(contract_address.to_owned(), index.to_owned(), block_id)
                .await?;

            println!("{}", res);
        }

        Commands::Rpc(rpc_args) => {
            let res = rpc_args.to_owned().run().await?;
            println!("{}", res);
        }

        Commands::Call {
            contract_address,
            function,
            inputs,
            abi,
            block_id,
            starknet,
        } => {
            let res = Probe::new(starknet.rpc_url.to_owned())
                .call(contract_address, function, inputs, block_id, abi)
                .await?;

            println!("{}", res);
        }

        Commands::StateUpdate { block_id, starknet } => {
            let res = Probe::new(starknet.rpc_url.to_owned())
                .get_state_update(block_id)
                .await?;
            println!("{}", res);
        }

        Commands::Index {
            variable_name,
            keys,
        } => {
            let res = SimpleProbe::get_storage_index(variable_name, keys)?;
            println!("{:#x}", res);
        }

        Commands::ContractHash { contract } => {
            let res = SimpleProbe::compute_contract_hash(contract)?;
            println!("{:#x}", res);
        }

        Commands::Estimate {
            contract_address,
            function_name,
            calldata,
            block_id,
            starknet,
        } => {
            let res = Probe::new(starknet.rpc_url.to_owned())
                .estimate_fee(
                    FunctionCall {
                        contract_address: contract_address.to_owned(),
                        calldata: calldata.to_owned(),
                        entry_point_selector: get_selector_from_name(function_name)?,
                    },
                    block_id,
                )
                .await?;
            println!("{}", res);
        }

        Commands::Class {
            hash,
            block_id,
            starknet,
        } => {
            let res = Probe::new(starknet.rpc_url.to_owned())
                .get_class_code(hash.to_owned(), block_id)
                .await?;
            println!("{}", res);
        }

        Commands::Code {
            contract_address,
            block_id,
            starknet,
        } => {
            let res = Probe::new(starknet.rpc_url.to_owned())
                .get_contract_code(contract_address.to_owned(), block_id)
                .await?;
            println!("{}", res);
        }

        Commands::ContractClass {
            contract_address,
            block_id,
            starknet,
        } => {
            let res = Probe::new(starknet.rpc_url.to_owned())
                .get_contract_class(contract_address.to_owned(), block_id)
                .await?;
            println!("{}", res);
        }

        Commands::ComputeAddress {
            caller_address,
            salt,
            class_hash,
            calldata,
        } => {
            let res = SimpleProbe::compute_contract_address(
                *caller_address,
                *salt,
                *class_hash,
                calldata,
            );
            println!("{}", res);
        }

        Commands::Events {
            chunk_size,
            continuation_token,
            from,
            keys,
            from_block,
            to_block,
            starknet,
        } => {
            let res = Probe::new(starknet.rpc_url.to_owned())
                .get_events(
                    EventFilter {
                        address: *from,
                        from_block: from_block.to_owned(),
                        to_block: to_block.to_owned(),
                        keys: keys.to_owned(),
                    },
                    *chunk_size,
                    continuation_token.to_owned(),
                )
                .await?;
            println!("{}", res);
        }

        Commands::SplitU256 { value } => {
            let res = SimpleProbe::split_u256(value)?;
            println!("{} {}", res.0, res.1);
        }
    }

    Ok(())
}
