use crate::opts::transaction::{DeclareTxArgs, DeployAccountTxArgs, DeployTxArgs, InvokeTxArgs};

use std::fs;

use clap::Subcommand;
use eyre::Result;
use starknet::providers::jsonrpc::models::{
    BroadcastedDeclareTransaction, BroadcastedDeployAccountTransaction,
    BroadcastedDeployTransaction, BroadcastedInvokeTransaction, BroadcastedInvokeTransactionV1,
    BroadcastedTransaction, ContractClass,
};

#[derive(Debug, Subcommand)]
pub enum EstimateFeeCommands {
    Invoke(InvokeTxArgs),
    Deploy(DeployTxArgs),
    Declare(DeclareTxArgs),
    DeployAccount(DeployAccountTxArgs),
}

impl EstimateFeeCommands {
    pub fn prepare_transaction(self) -> Result<BroadcastedTransaction> {
        let tx = match self {
            Self::Invoke(InvokeTxArgs {
                sender_address,
                calldata,
                transaction,
            }) => BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction::V1(
                BroadcastedInvokeTransactionV1 {
                    calldata,
                    sender_address,
                    max_fee: transaction.max_fee.expect("missing max fee"),
                    nonce: transaction.nonce.expect("missing nonce"),
                    signature: transaction.signature.expect("missing signature"),
                },
            )),

            Self::DeployAccount(DeployAccountTxArgs {
                class_hash,
                contract_address_salt,
                constructor_calldata,
                transaction,
            }) => BroadcastedTransaction::DeployAccount(BroadcastedDeployAccountTransaction {
                class_hash,
                constructor_calldata,
                contract_address_salt,
                max_fee: transaction.max_fee.expect("missing max fee"),
                nonce: transaction.nonce.expect("missing nonce"),
                signature: transaction.signature.expect("missing signature"),
                version: 1,
            }),

            Self::Declare(DeclareTxArgs {
                contract,
                sender_address,
                transaction,
            }) => {
                let res = fs::read_to_string(contract)?;
                let contract_class: ContractClass = serde_json::from_str(&res)?;

                BroadcastedTransaction::Declare(BroadcastedDeclareTransaction {
                    contract_class,
                    sender_address,
                    max_fee: transaction.max_fee.expect("missing max fee"),
                    nonce: transaction.nonce.expect("missing nonce"),
                    signature: transaction.signature.expect("missing signature"),
                    version: 1,
                })
            }

            Self::Deploy(DeployTxArgs {
                contract,
                contract_address_salt,
                constructor_calldata,
                ..
            }) => {
                let res = fs::read_to_string(contract)?;
                let contract_class: ContractClass = serde_json::from_str(&res)?;

                BroadcastedTransaction::Deploy(BroadcastedDeployTransaction {
                    contract_class,
                    constructor_calldata,
                    contract_address_salt,
                    version: 0,
                })
            }
        };

        Ok(tx)
    }
}
