use clap::{Args, Subcommand};
use eyre::Result;
use starknet_core::{
    crypto::{ecdsa_sign, ecdsa_verify, Signature},
    types::FieldElement,
};

#[derive(Subcommand, Debug)]
pub enum EcdsaCommand {
    #[clap(about = "Sign a message.")]
    Sign(SignArgs),

    #[clap(about = "Verify the signature of a message.")]
    Verify(VerifyArgs),
}

#[derive(Args, Debug)]
// #[clap(group(ArgGroup::new("signing_account").required(true).args(&["account", "private-key"])))]
pub struct SignArgs {
    #[clap(long)]
    #[clap(value_name = "PRIVATEKEY")]
    // #[clap(conflicts_with = "account-dir")]
    #[clap(help = "Specify a private key for signing.")]
    pub private_key: Option<String>,

    #[clap(value_name = "MESSAGE")]
    #[clap(help = "Message hash to be signed.")]
    pub message: String,
    //

    // #[clap(long)]
    // #[clap(value_name = "NAME")]
    // #[clap(help = "Use an account from the StarkNet keystore.")]
    // account: Option<String>,

    // #[clap(long)]
    // #[clap(value_name = "DIR")]
    // #[clap(requires = "account")]
    // #[clap(default_value = "~/.starknet_accounts")]
    // #[clap(help = "The directory containing the StarkNet keystore.")]
    // account_dir: String,
}

#[derive(Args, Debug)]
// #[clap(group(ArgGroup::new("verifying_account").required(true).args(&["account", "public-key"])))]
pub struct VerifyArgs {
    #[clap(long)]
    #[clap(value_name = "PUBLICKEY")]
    // #[clap(conflicts_with = "account-dir")]
    #[clap(help = "Specify a public key for verification.")]
    pub public_key: Option<String>,

    #[clap(value_name = "MESSAGE")]
    #[clap(help = "Message hash used in the signature.")]
    pub message: String,

    #[clap(help = "R value of the signature.")]
    pub signature_r: String,

    #[clap(help = "S value of the signature.")]
    pub signature_s: String,
    //

    // #[clap(long)]
    // #[clap(value_name = "ACCOUNTNAME")]
    // #[clap(help = "Use an account from the StarkNet keystore.")]
    // account: Option<String>,

    // #[clap(long)]
    // #[clap(value_name = "DIR")]
    // #[clap(requires = "account")]
    // #[clap(default_value = "~/.starknet_accounts")]
    // #[clap(help = "The directory containing the StarkNet keystore.")]
    // account_dir: String,
    // #[clap(long)]
    // #[clap(required = true)]
    // #[clap(number_of_values = 2)]
    // #[clap(help = "ECDSA signature, r and s.")]
    // #[clap(value_names = &["R", "S"])]
    // signature: Vec<String>,
}

impl EcdsaCommand {
    pub fn sign(private_key: &str, message_hash: &str) -> Result<Signature> {
        Ok(ecdsa_sign(
            &FieldElement::from_hex_be(private_key)?,
            &FieldElement::from_hex_be(message_hash)?,
        )?)
    }

    pub fn verify(
        public_key: &str,
        message_hash: &str,
        signature_r: &str,
        signature_s: &str,
    ) -> Result<bool> {
        Ok(ecdsa_verify(
            &FieldElement::from_hex_be(public_key)?,
            &FieldElement::from_hex_be(message_hash)?,
            &Signature {
                r: FieldElement::from_hex_be(signature_r)?,
                s: FieldElement::from_hex_be(signature_s)?,
            },
        )?)
    }
}
