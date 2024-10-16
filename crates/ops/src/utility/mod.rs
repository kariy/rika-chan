use color_eyre::Result;
use rika_args::commands::utility::IndexArgs;
use starknet::core::utils::get_storage_var_address;

pub fn storage_address(args: IndexArgs) -> Result<()> {
    let IndexArgs { var_name, keys } = args;
    println!("{var_name}");
    let a = keys
        .iter()
        .map(|k| format!("{k:#x}"))
        .collect::<Vec<String>>()
        .join(" ");
    println!("{a}");
    let address = get_storage_var_address(&var_name, &keys)?;
    println!("{address:#x}");
    Ok(())
}
