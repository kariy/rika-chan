use eyre::Result;
use rika_args::commands::utility::IndexArgs;
use starknet::core::utils::get_storage_var_address;

pub fn storage_address(args: IndexArgs) -> Result<()> {
    let IndexArgs { var_name, keys } = args;
    let address = get_storage_var_address(&var_name, &keys)?;
    println!("{address:#x}");
    Ok(())
}
