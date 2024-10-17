mod json;

use clap::Args;
use color_eyre::Result;
use probe_fmt::Pretty;

pub use self::json::JsonDisplay;

#[derive(Debug, Args)]
pub struct DisplayOptions<T = NullDisplayOptions>
where
    T: Args,
{
    #[command(flatten)]
    pub raw_format: T,
}

impl DisplayOptions {
    pub fn print(&self, value: impl std::fmt::Display) -> Result<()> {
        println!("{value}");
        Ok(())
    }
}

impl<T> DisplayOptions<T>
where
    T: Args + RawDisplay,
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

#[derive(Debug, Args)]
pub struct NullDisplayOptions;
