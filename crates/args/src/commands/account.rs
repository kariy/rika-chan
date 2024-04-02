use clap::Parser;

use crate::account::WalletCommands;

#[derive(Debug, Parser)]
pub struct AccountArgs {
    #[command(subcommand)]
    commands: WalletCommands,
}
