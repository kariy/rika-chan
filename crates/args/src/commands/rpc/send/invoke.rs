use clap::Args;
use color_eyre::eyre::bail;
use color_eyre::Result;
use starknet::accounts::{Account, Call};
use starknet::core::types::{FieldElement, InvokeTransactionResult};

use crate::opts::account::WalletOptions;
use crate::opts::starknet::StarknetOptions;
use crate::opts::transaction::TransactionOptions;
use crate::parser::selector_parser;

#[derive(Debug, Args)]
pub struct InvokeArgs {
    #[arg(value_name = "CONTRACT_ADDRESS")]
    pub to: FieldElement,

    #[arg(value_name = "SELECTOR")]
    #[arg(help = "The function of that contract that you want to call.")]
    #[arg(long_help = "The function of that contract that you want to call. Can be the actual \
                       function name or the function selector. E.g., 'foo' or '0x12345678'.")]
    #[arg(value_parser(selector_parser))]
    pub selector: FieldElement,

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
        let InvokeArgs { to, selector, calldata, starknet, wallet, transaction } = self;

        let Some(wallet) = wallet.build_wallet()? else { bail!("missing wallet") };

        let account = wallet.account(starknet.provider()).await?;

        let mut tx = account.execute(vec![Call { to, selector, calldata }]);

        if let Some(nonce) = transaction.nonce {
            tx = tx.nonce(nonce);
        }

        if let Some(max_fee) = transaction.max_fee {
            tx = tx.max_fee(max_fee);
        }

        Ok(tx.send().await?)
    }
}
