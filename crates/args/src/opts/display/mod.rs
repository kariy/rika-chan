mod json;

use clap::Args;
use eyre::Result;

pub use self::json::JsonDisplay;
use crate::fmt::Pretty;

#[derive(Debug, Args)]
pub struct DisplayOptions<T>
where
    T: Args + RawDisplay,
{
    #[command(flatten)]
    pub raw_format: T,
}

impl<T> DisplayOptions<T>
where
    T: RawDisplay + Args,
{
    pub fn print(&self, value: <T as RawDisplay>::Value) -> Result<()>
    where
        <T as RawDisplay>::Value: Pretty,
    {
        if self.raw_format.is_raw() {
            println!("{}", self.raw_format.display_raw(value)?);
        } else {
            println!("{}", value.prettify());
        }
        Ok(())
    }
}

/// Trait for displaying a value in its intended raw format.
pub trait RawDisplay {
    /// The type of the value to be displayed.
    type Value;

    /// Returns the [Display](std::fmt::Display) implementation for the raw value.
    fn display_raw(&self, value: Self::Value) -> Result<impl std::fmt::Display>;

    fn is_raw(&self) -> bool {
        false
    }
}
