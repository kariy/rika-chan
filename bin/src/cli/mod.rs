#[cfg(feature = "rpc")]
mod rpc;
mod utils;

use clap::Parser;
use color_eyre::Result;
use probe_args::commands::utility;

#[derive(Parser, Debug)]
#[command(name = "probe", version, about, long_about = None)]
pub enum Cli {
    #[command(flatten)]
    Utilities(utility::UtilityCommands),

    #[cfg(feature = "rpc")]
    #[command(flatten)]
    Rpc(rpc::RpcCommands),
}

impl Cli {
    pub fn execute(self) -> Result<()> {
        match self {
            Cli::Utilities(cmd) => utils::execute(cmd)?,
            #[cfg(feature = "rpc")]
            Cli::Rpc(rpc) => rpc::execute(rpc)?,
        }
        Ok(())
    }
}
