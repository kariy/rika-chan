use std::path::PathBuf;
use std::sync::Arc;

use crate::opts::account::utils::read_json_file;
use crate::opts::account::WalletOptions;
use crate::opts::starknet::StarknetOptions;
use crate::opts::transaction::TransactionOptions;
use crate::probe::SimpleProbe;

use clap::Args;
use eyre::{bail, Result};
use starknet::accounts::{Account, Call};
use starknet::core::types::contract::legacy::LegacyContractClass;
use starknet::core::types::contract::SierraClass;
use starknet::core::types::InvokeTransactionResult;
use starknet::core::types::{DeclareTransactionResult, FieldElement};
use starknet::core::utils::get_selector_from_name;

#[derive(Debug, Args)]
pub struct InvokeArgs {
    #[arg(long)]
    #[arg(value_name = "CONTRACT_ADDRESS")]
    pub to: FieldElement,

    #[arg(long)]
    #[arg(value_name = "FUNCTION_NAME")]
    pub function: String,

    #[arg(long)]
    #[arg(value_delimiter = ',')]
    #[arg(help = "Comma seperated values e.g., 0x12345,0x69420,...")]
    pub calldata: Vec<FieldElement>,

    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    pub starknet: StarknetOptions,

    #[command(flatten)]
    #[command(next_help_heading = "Wallet options")]
    pub wallet: WalletOptions,

    #[command(flatten)]
    #[command(next_help_heading = "Transaction options")]
    pub transaction: TransactionOptions,
}

impl InvokeArgs {
    pub async fn run(self) -> Result<InvokeTransactionResult> {
        let InvokeArgs {
            to,
            function,
            calldata,
            starknet,
            wallet,
            transaction,
        } = self;

        let Some(wallet) = wallet.build_wallet()? else {
            bail!("missing wallet")
        };

        let account = wallet.account(starknet.provider()).await?;

        let mut tx = account.execute(vec![Call {
            to,
            selector: get_selector_from_name(&function)?,
            calldata,
        }]);

        if let Some(nonce) = transaction.nonce {
            tx = tx.nonce(nonce);
        }

        if let Some(max_fee) = transaction.max_fee {
            tx = tx.max_fee(max_fee);
        }

        tx.send().await.map_err(|e| e.into())
    }
}

#[derive(Debug, Args)]
pub struct DeclareArgs {
    #[arg(short = 'C', long)]
    #[arg(help = "The path to the contract artifact file")]
    pub contract_path: PathBuf,

    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    pub starknet: StarknetOptions,

    #[command(flatten)]
    #[command(next_help_heading = "Wallet options")]
    pub wallet: WalletOptions,

    #[command(flatten)]
    #[command(next_help_heading = "Transaction options")]
    pub transaction: TransactionOptions,
}

impl DeclareArgs {
    pub async fn run(self) -> Result<DeclareTransactionResult> {
        let DeclareArgs {
            contract_path,
            starknet,
            wallet,
            transaction,
        } = self;

        let Some(wallet) = wallet.build_wallet()? else {
            bail!("missing wallet")
        };

        let account = wallet.account(starknet.provider()).await?;
        let contract: SierraClass = read_json_file(&contract_path)?;
        let compiled_class_hash = SimpleProbe::compute_compiled_contract_hash(contract_path)?;

        let mut tx = account.declare(Arc::new(contract.flatten()?), compiled_class_hash);

        if let Some(nonce) = transaction.nonce {
            tx = tx.nonce(nonce);
        }

        if let Some(max_fee) = transaction.max_fee {
            tx = tx.max_fee(max_fee);
        }

        tx.send().await.map_err(|e| e.into())
    }
}

#[derive(Debug, Args)]
pub struct LegacyDeclareArgs {
    #[arg(short = 'C', long)]
    #[arg(help = "The path to the contract artifact file")]
    pub contract_path: PathBuf,

    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    pub starknet: StarknetOptions,

    #[command(flatten)]
    #[command(next_help_heading = "Wallet options")]
    pub wallet: WalletOptions,

    #[command(flatten)]
    #[command(next_help_heading = "Transaction options")]
    pub transaction: TransactionOptions,
}

impl LegacyDeclareArgs {
    pub async fn run(self) -> Result<DeclareTransactionResult> {
        let LegacyDeclareArgs {
            contract_path,
            starknet,
            wallet,
            transaction,
        } = self;

        let Some(wallet) = wallet.build_wallet()? else {
            bail!("missing wallet")
        };

        let account = wallet.account(starknet.provider()).await?;
        let contract: LegacyContractClass = read_json_file(&contract_path)?;

        let mut tx = account.declare_legacy(Arc::new(contract));

        if let Some(nonce) = transaction.nonce {
            tx = tx.nonce(nonce);
        }

        if let Some(max_fee) = transaction.max_fee {
            tx = tx.max_fee(max_fee);
        }

        tx.send().await.map_err(|e| e.into())
    }
}
