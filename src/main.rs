mod cli;
mod utils;

use crate::cli::{
    commands::ecdsa::{self, EcdsaCommand},
    App, Commands,
};

use clap::Parser;
use eyre::Result;
use starknet_core::{
    types::FieldElement,
    utils::{cairo_short_string_to_felt, parse_cairo_short_string},
};

fn main() -> Result<()> {
    let cli = App::parse();

    match &cli.command {
        Commands::AddressZero => {
            println!("{}", utils::hex_encode(FieldElement::ZERO.to_bytes_be()));
        }

        Commands::DecToHex { dec } => {
            let felt = FieldElement::from_dec_str(dec)?;
            println!("{}", utils::hex_encode(felt.to_bytes_be()));
        }

        Commands::Ecdsa { commands } => match commands {
            ecdsa::EcdsaCommand::Sign(ecdsa::SignArgs {
                private_key,
                message,
            }) => {
                let private_key = private_key.to_owned().unwrap();
                let signature = EcdsaCommand::sign(&private_key, &message)?;
                println!("{} {}", signature.r, signature.s);
            }

            ecdsa::EcdsaCommand::Verify(ecdsa::VerifyArgs {
                public_key,
                message,
                signature_r,
                signature_s,
            }) => {
                let public_key = public_key.to_owned().unwrap();
                let is_valid =
                    EcdsaCommand::verify(&public_key, &message, &signature_r, &signature_s)?;
                println!("{}", is_valid);
            }
        },

        Commands::FromUtf8 { felt } => {
            let str = parse_cairo_short_string(&FieldElement::from_hex_be(felt)?)?;
            println!("{}", str);
        }

        Commands::HexToDec { hex } => {
            let felt = FieldElement::from_hex_be(hex)?;
            println!("{}", felt);
        }

        Commands::Keccak { data } => {
            let hash = utils::keccak(data)?;
            println!("{}", hash);
        }

        Commands::MaxSignedFelt => {
            println!("{}", utils::SIGNED_FELT_MAX);
        }

        Commands::MinSignedFelt => {
            println!("{}", utils::SIGNED_FELT_MIN)
        }

        Commands::StrToFelt { str } => {
            let felt = cairo_short_string_to_felt(str)?;
            println!("{}", utils::hex_encode(felt.to_bytes_be()));
        }

        Commands::MaxUnsignedFelt => {
            println!("{}", FieldElement::MAX);
        }

        Commands::Pedersen { x, y } => {
            let hash = utils::pedersen(x, y)?;
            println!("{}", hash);
        }

        #[allow(unreachable_patterns)]
        _ => {}
    }

    Ok(())
}
