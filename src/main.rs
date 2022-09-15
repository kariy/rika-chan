#![allow(warnings)]
mod cli;

use clap::Parser;
use cli::{App, Commands};
use hex::{FromHex, ToHex};
use starknet_core::{
    types::FieldElement,
    utils::{
        cairo_short_string_to_felt, get_selector_from_name, parse_cairo_short_string,
        starknet_keccak,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = App::parse();

    // println!("commands : {:?}", &cli.command);

    match &cli.command {
        Commands::AddressZero => {
            println!(
                "0x{}",
                hex::encode(starknet_core::types::FieldElement::ZERO.to_bytes_be())
            );
        }

        Commands::DecToHex {
            dec,
            padding,
            upper,
        } => {
            let felt_bytes = FieldElement::from_dec_str(dec)?.to_bytes_be();
            let hex_str = if *upper {
                hex::encode_upper(felt_bytes)
            } else {
                hex::encode(felt_bytes)
            };

            if *padding {
                println!("0x{}", hex_str.strip_prefix("0").unwrap());
            } else {
                let mut start: usize = hex_str.len() - 1;
                for (idx, char) in hex_str.chars().enumerate() {
                    if char != '0' {
                        start = idx;
                        break;
                    }
                }
                let unpadded_hex = hex_str.get(start..).unwrap();
                println!("0x{}", unpadded_hex);
            }
        }

        Commands::FeltToStr { felt } => {
            let str = parse_cairo_short_string(&FieldElement::from_dec_str(felt)?)?;
            println!("{}", str);
        }

        Commands::HexToDec { hex } => {
            let felt = FieldElement::from_hex_be(hex)?;
            println!("{}", felt.to_string());
        }

        Commands::Keccak { data } => {
            let bytes = data.as_bytes();
            let hash = starknet_keccak(bytes);
            println!("{}", hash);
        }

        Commands::Selector { function_name } => {
            let selector = get_selector_from_name(function_name)?;
            println!("{}", selector);
        }

        Commands::StrToFelt { str } => {
            let felt = cairo_short_string_to_felt(str)?;
            println!("{}", felt.to_string());
        }

        Commands::UnsignedFeltMax => {
            println!("{}", FieldElement::MAX);
        }

        _ => {}
    }

    Ok(())
}
