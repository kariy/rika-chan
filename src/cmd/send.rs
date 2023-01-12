use crate::cmd::account::simple_account::Account;
use crate::opts::account::WalletOptions;
use crate::opts::starknet::StarkNetOptions;
use crate::opts::transaction::TransactionOptions;

use clap::Parser;
use eyre::{bail, eyre, Result};
use starknet::accounts::Call;
use starknet::core::types::FieldElement;
use starknet::core::utils::get_selector_from_name;
use starknet::providers::jsonrpc::models::{BroadcastedTransaction, InvokeTransactionResult};
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};

#[derive(Debug, Parser)]
pub struct InvokeArgs {
    #[clap(long)]
    #[clap(value_name = "CONTRACT_ADDRESS")]
    pub to: FieldElement,

    #[clap(long)]
    #[clap(value_name = "FUNCTION_NAME")]
    pub function: String,

    #[clap(long)]
    #[clap(value_delimiter = ',')]
    #[clap(help = "Comma seperated values e.g., 0x12345,0x69420,...")]
    pub calldata: Vec<FieldElement>,

    #[clap(flatten)]
    #[clap(next_help_heading = "STARKNET OPTIONS")]
    pub starknet: StarkNetOptions,

    #[clap(flatten)]
    #[clap(next_help_heading = "Wallet OPTIONS")]
    pub wallet: WalletOptions,

    #[clap(flatten)]
    #[clap(next_help_heading = "TRANSACTION OPTIONS")]
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

        let Some(mut account) = wallet.build_wallet()? else {
            bail!("missing wallet")
        };

        account
            .provider
            .get_or_insert(JsonRpcClient::new(HttpTransport::new(starknet.rpc_url)));

        let call = Call {
            to,
            selector: get_selector_from_name(&function)?,
            calldata,
        };

        let nonce = match transaction.nonce {
            Some(nonce) => nonce,
            None => account.get_nonce().await?,
        };

        let max_fee = match transaction.max_fee {
            Some(ref fee) => fee.to_owned(),
            None => {
                let request = account
                    .prepare_invoke_transaction(&[call.clone()], nonce, FieldElement::ZERO)
                    .await?;

                account
                    .get_max_fee(&BroadcastedTransaction::Invoke(request))
                    .await
                    .map(FieldElement::from)?
            }
        };

        let request = account
            .prepare_invoke_transaction(&[call], nonce, max_fee)
            .await?;

        account
            .send_invoke_transaction(&request)
            .await
            .map_err(|e| eyre!(e))
    }
}
