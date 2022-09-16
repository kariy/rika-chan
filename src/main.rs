mod cli;
mod utils;

use crate::cli::{App, Commands, EcdsaCommand};
use crate::utils::Misc;

use clap::Parser;
use eyre::Result;
use starknet_core::{
    crypto::{ecdsa_sign, ecdsa_verify, Signature},
    types::FieldElement,
    utils::{cairo_short_string_to_felt, get_selector_from_name, parse_cairo_short_string},
};

fn main() -> Result<()> {
    let cli = App::parse();

    println!("commands : {:?}", &cli.command);

    match &cli.command {
        Commands::AddressZero => {
            println!(
                "{}",
                Misc::hex_encode(starknet_core::types::FieldElement::ZERO.to_bytes_be())
            );
        }

        Commands::DecToHex { dec } => {
            let felt = FieldElement::from_dec_str(dec)?;
            println!("{}", Misc::hex_encode(felt.to_bytes_be()));
        }

        Commands::Ecdsa { commands } => match commands {
            EcdsaCommand::Sign {
                private_key,
                message,
            } => {
                let private_key = private_key.clone().expect("MISSING PRIVATE KEY");
                let signature = ecdsa_sign(
                    &FieldElement::from_hex_be(&private_key)?,
                    &FieldElement::from_hex_be(message)?,
                )?;
                println!("{} {}", signature.r, signature.s);
            }

            EcdsaCommand::Verify {
                public_key,
                message,
                signature_r,
                signature_s,
            } => {
                let public_key = public_key.clone().expect("MISSING PUBLIC KEY");
                let is_valid = ecdsa_verify(
                    &FieldElement::from_hex_be(&public_key)?,
                    &FieldElement::from_hex_be(&message)?,
                    &Signature {
                        r: FieldElement::from_hex_be(&signature_r)?,
                        s: FieldElement::from_hex_be(&signature_s)?,
                    },
                )?;
                println!("{}", is_valid);
            }
        },

        Commands::FeltToStr { felt } => {
            let str = parse_cairo_short_string(&FieldElement::from_dec_str(felt)?)?;
            println!("{}", str);
        }

        Commands::HexToDec { hex } => {
            let felt = FieldElement::from_hex_be(hex)?;
            println!("{}", felt);
        }

        Commands::Keccak { data } => {
            let hash = Misc::keccak(data)?;
            println!("{}", hash);
        }

        Commands::Selector { function_name } => {
            let selector = get_selector_from_name(function_name)?;
            println!("{}", selector);
        }

        Commands::MaxSignedFelt => {
            println!("{}", utils::SIGNED_FELT_MAX);
        }

        Commands::MinSignedFelt => {
            println!("{}", utils::SIGNED_FELT_MIN)
        }

        Commands::StrToFelt { str } => {
            let felt = cairo_short_string_to_felt(str)?;
            println!("{}", felt);
        }

        Commands::MaxUnsignedFelt => {
            println!("{}", FieldElement::MAX);
        }

        Commands::Pedersen { x, y } => {
            let hash = Misc::pedersen(x, y)?;
            println!("{}", hash);
        }
    }

    Ok(())
}
