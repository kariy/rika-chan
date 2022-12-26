use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct AccountOptions {
    #[clap(long)]
    #[clap(value_name = "PRIVATE_KEY")]
    #[clap(help_heading = "WALLET OPTIONS - RAW")]
    #[clap(help = "The raw private key associated with the account contract.")]
    pub private_key: Option<String>,

    #[clap(long)]
    #[clap(value_name = "ACCOUNT_ADDRESS")]
    #[clap(help_heading = "WALLET OPTIONS - RAW")]
    #[clap(help = "Account contract to initiate the transaction from.")]
    pub account_address: Option<String>,
}
