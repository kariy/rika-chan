use rika_args::commands::{App, Commands, EcdsaCommand};
use rika_old::{Rika, SimpleRika};

use chrono::{Local, TimeZone};
use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use eyre::{eyre, Result};
use starknet::core::types::EventFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = App::parse();

    match cli.command {
        Commands::DecToHex { dec, pad } => {
            println!("{}", SimpleRika::to_hex(&dec, pad));
        }

        Commands::Ecdsa { commands } => match commands {
            EcdsaCommand::Sign {
                message,
                private_key,
            } => {
                let signature = SimpleRika::ecdsa_sign(&private_key, &message)?;
                println!("{:#x} {:#x}", signature.r, signature.s);
            }

            EcdsaCommand::Verify {
                message,
                signature,
                verifying_key,
            } => {
                let is_valid = SimpleRika::ecdsa_verify(
                    &verifying_key,
                    &message,
                    &signature[0],
                    &signature[1],
                )?;
                println!("{is_valid}");
            }
        },

        Commands::FromAscii { ascii } => {
            println!("{}", SimpleRika::from_utf8(&ascii)?);
        }

        Commands::HexToDec { hex } => {
            println!("{}", SimpleRika::to_dec(&hex));
        }

        Commands::Keccak { data } => {
            println!("{}", SimpleRika::keccak(&data)?);
        }

        Commands::MaxSignedFelt => {
            println!("{}", SimpleRika::max_signed_felt());
        }

        Commands::MinSignedFelt => {
            println!("{}", SimpleRika::min_signed_felt())
        }

        Commands::ToAscii { short_str } => {
            println!("{}", SimpleRika::str_to_felt(&short_str)?);
        }

        Commands::MaxUnsignedFelt => {
            println!("{}", SimpleRika::max_felt());
        }

        Commands::Pedersen { elements } => {
            println!("{:#x}", SimpleRika::pedersen(&elements));
        }

        Commands::BlockNumber { starknet } => {
            let res = Rika::new_with_http(starknet.rpc_url).block_number().await?;
            println!("{res}");
        }

        Commands::ChainId { starknet } => {
            let chain_id = Rika::new_with_http(starknet.rpc_url).chain_id().await?;
            println!("{chain_id}");
        }

        Commands::Transaction {
            hash,
            field,
            to_json,
            starknet,
        } => {
            let res = Rika::new_with_http(starknet.rpc_url)
                .get_transaction_by_hash(hash, field, to_json)
                .await?;
            println!("{res}");
        }

        Commands::TransactionStatus { hash, starknet } => {
            let res = Rika::new_with_http(starknet.rpc_url)
                .get_transaction_status(hash)
                .await?;
            println!("{res}");
        }

        Commands::Receipt {
            hash,
            display,
            starknet,
        } => {
            // let res = Rika::new_with_http(starknet.rpc_url)
            //     .get_transaction_receipt(hash, field, to_json)
            //     .await?;
            // println!("{res}");

            todo!()
        }

        Commands::Block {
            id,
            full,
            to_json,
            field,
            starknet,
        } => {
            let block = Rika::new_with_http(starknet.rpc_url)
                .block(id, full, field, to_json)
                .await?;
            println!("{block}")
        }

        Commands::Age {
            block_id,
            human_readable,
            starknet,
        } => {
            let timestamp = Rika::new_with_http(starknet.rpc_url)
                .block(block_id, false, Some("timestamp".to_string()), false)
                .await?;

            if human_readable {
                let timestamp = Local
                    .timestamp_opt(timestamp.parse::<i64>().unwrap(), 0)
                    .unwrap()
                    .to_string();
                println!("{timestamp}")
            } else {
                println!("{timestamp}");
            }
        }

        Commands::TransactionCount { block_id, starknet } => {
            let total = Rika::new_with_http(starknet.rpc_url)
                .get_block_transaction_count(block_id)
                .await?;

            println!("{total}");
        }

        Commands::Nonce {
            contract_address,
            block_id,
            starknet,
        } => {
            let nonce = Rika::new_with_http(starknet.rpc_url)
                .get_nonce(contract_address, &block_id)
                .await?;
            println!("{nonce}");
        }

        Commands::Storage {
            contract_address,
            index,
            block_id,
            starknet,
        } => {
            let res = Rika::new_with_http(starknet.rpc_url)
                .get_storage_at(contract_address, index, &block_id)
                .await?;

            println!("{res}");
        }

        Commands::Rpc(rpc_args) => {
            let res = rpc_args.run().await?;
            println!("{res}");
        }

        Commands::Call {
            contract_address,
            function,
            input,
            block_id,
            starknet,
        } => {
            let res = Rika::new_with_http(starknet.rpc_url)
                .call(&contract_address, &function, &input, &block_id)
                .await?;

            println!("{res}");
        }

        Commands::StateUpdate { block_id, starknet } => {
            let res = Rika::new_with_http(starknet.rpc_url)
                .get_state_update(&block_id)
                .await?;
            println!("{res}");
        }

        Commands::Index {
            variable_name,
            keys,
        } => {
            let res = SimpleRika::get_storage_index(&variable_name, &keys)?;
            println!("{res:#x}");
        }

        Commands::ClassHash { contract } => {
            let res = SimpleRika::compute_contract_hash(contract)?;
            println!("{res:#x}");
        }

        Commands::CompiledClassHash { contract } => {
            let res = SimpleRika::compute_compiled_contract_hash(contract)?;
            println!("{res:#x}");
        }

        Commands::Class {
            hash,
            block_id,
            starknet,
        } => {
            let res = Rika::new_with_http(starknet.rpc_url)
                .get_class_code(hash, &block_id)
                .await?;
            println!("{res}");
        }

        Commands::Code {
            contract_address,
            block_id,
            starknet,
        } => {
            let res = Rika::new_with_http(starknet.rpc_url)
                .get_contract_code(contract_address, &block_id)
                .await?;
            println!("{res}");
        }

        Commands::ContractClass {
            contract_address,
            block_id,
            starknet,
        } => {
            let res = Rika::new_with_http(starknet.rpc_url)
                .get_contract_class(contract_address, &block_id)
                .await?;
            println!("{res}");
        }

        Commands::ComputeAddress {
            caller_address,
            salt,
            class_hash,
            calldata,
        } => {
            let res =
                SimpleRika::compute_contract_address(caller_address, salt, class_hash, &calldata);
            println!("{res}");
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
            let res = Rika::new_with_http(starknet.rpc_url)
                .get_events(
                    EventFilter {
                        address: from,
                        from_block,
                        to_block,
                        keys,
                    },
                    chunk_size,
                    continuation_token,
                )
                .await?;
            println!("{res}");
        }

        Commands::SplitU256 { value } => {
            let res = SimpleRika::split_u256(&value)?;
            println!("{} {}", res.0, res.1);
        }

        Commands::Account { commands } => {
            commands.run().await?;
        }

        Commands::Balance {
            address,
            block_id,
            starknet,
        } => {
            let res = Rika::new_with_http(starknet.rpc_url)
                .get_eth_balance(address, block_id)
                .await?;
            println!("{res}");
        }

        Commands::CallArray { calls } => {
            let arg = calls.join(" ");
            let vec = SimpleRika::generate_multicall_calldata(&arg)?
                .into_iter()
                .map(|e| format!("{e:#x}"))
                .collect::<Vec<String>>();

            println!("{}", vec.join(" "))
        }

        Commands::ShellCompletions { shell } => {
            let shell = shell
                .or_else(Shell::from_env)
                .ok_or_else(|| eyre!("unable to identify shell from environment variable"))?;
            generate(shell, &mut App::command(), "rika", &mut std::io::stdout());
        }

        Commands::Syncing { starknet } => {
            let res = Rika::new_with_http(starknet.rpc_url).syncing().await?;
            println!("{res}");
        }
    }

    Ok(())
}
