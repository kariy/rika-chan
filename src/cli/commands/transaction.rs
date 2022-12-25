use std::path::PathBuf;

use clap::Parser;
use starknet::core::types::FieldElement;

use super::opts::TransactionOptions;

#[derive(Debug, Parser)]
pub struct InvokeTxArgs {
    #[clap(short, long)]
    pub sender_address: FieldElement,

    #[clap(short = 'd', long)]
    pub calldata: Vec<FieldElement>,

    #[clap(flatten)]
    #[clap(next_help_heading = "TRANSACTION OPTIONS")]
    pub transaction: TransactionOptions,
}

#[derive(Debug, Parser)]
pub struct DeclareTxArgs {
    #[clap(short, long)]
    #[clap(help = "The file of the contract to be declared")]
    pub contract: PathBuf,

    #[clap(short, long)]
    pub sender_address: FieldElement,

    #[clap(flatten)]
    #[clap(next_help_heading = "TRANSACTION OPTIONS")]
    pub transaction: TransactionOptions,
}

#[derive(Debug, Parser)]
pub struct DeployTxArgs {
    #[clap(short, long)]
    #[clap(help = "The file of the contract to be deployed")]
    pub contract: PathBuf,

    #[clap(short = 's', long = "salt")]
    pub contract_address_salt: FieldElement,

    #[clap(short = 'd', long)]
    pub constructor_calldata: Vec<FieldElement>,

    #[clap(flatten)]
    #[clap(next_help_heading = "TRANSACTION OPTIONS")]
    pub transaction: TransactionOptions,
}

#[derive(Debug, Parser)]
pub struct DeployAccountTxArgs {
    #[clap(short, long)]
    pub class_hash: FieldElement,

    #[clap(short = 's', long)]
    pub contract_address_salt: FieldElement,

    #[clap(short = 'd', long)]
    pub constructor_calldata: Vec<FieldElement>,

    #[clap(flatten)]
    #[clap(next_help_heading = "TRANSACTION OPTIONS")]
    pub transaction: TransactionOptions,
}
