use color_eyre::Result;
pub use probe_args::commands::utility::UtilityCommands;
use probe_ops as ops;

pub fn execute(command: UtilityCommands) -> Result<()> {
    match command {
        UtilityCommands::Index(args) => ops::utility::storage_address(args)?,
        _ => unimplemented!("This command is not implemented yet"),
    }

    Ok(())
}
