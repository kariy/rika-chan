mod cast;
mod commands;

use crate::cast::{Cast, SimpleCast};
use crate::commands::{App, Commands};

use clap::Parser;
use eyre::Result;

fn main() -> Result<()> {
    let cli = App::parse();

    match &cli.command {
        Commands::AddressZero => {
            println!("{}", SimpleCast::address_zero());
        }

        Commands::DecToHex { dec } => {
            println!("{}", SimpleCast::to_hex(dec)?);
        }

        Commands::Ecdsa { commands } => match commands {
            commands::EcdsaCommand::Sign {
                private_key,
                message,
            } => {
                let private_key = private_key.to_owned().unwrap();
                let signature = SimpleCast::ecdsa_sign(&private_key, &message)?;
                println!("{} {}", signature.r, signature.s);
            }

            commands::EcdsaCommand::Verify {
                public_key,
                message,
                signature_r,
                signature_s,
            } => {
                let public_key = public_key.to_owned().unwrap();
                let is_valid =
                    SimpleCast::ecdsa_verify(&public_key, &message, &signature_r, &signature_s)?;
                println!("{}", is_valid);
            }
        },

        Commands::FromUtf8 { felt } => {
            println!("{}", SimpleCast::from_utf8(felt)?);
        }

        Commands::HexToDec { hex } => {
            println!("{}", SimpleCast::to_dec(hex)?);
        }

        Commands::Keccak { data } => {
            println!("{}", SimpleCast::keccak(data)?);
        }

        Commands::MaxSignedFelt => {
            println!("{}", SimpleCast::max_signed_felt());
        }

        Commands::MinSignedFelt => {
            println!("{}", SimpleCast::min_signed_felt())
        }

        Commands::StrToFelt { str } => {
            println!("{}", SimpleCast::str_to_felt(str)?);
        }

        Commands::MaxUnsignedFelt => {
            println!("{}", SimpleCast::max_felt());
        }

        Commands::Pedersen { x, y } => {
            println!("{}", SimpleCast::pedersen(x, y)?);
        }

        #[allow(unreachable_patterns)]
        _ => {}
    }

    Ok(())
}
