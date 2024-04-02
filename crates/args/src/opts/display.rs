use clap::Args;
use eyre::{ContextCompat, Result};
use serde::Serialize;

use crate::fmt::Pretty;

#[derive(Debug, Args)]
pub struct DisplayOptions {
    #[arg(short, long)]
    #[arg(help = "Display the output in JSON format")]
    pub json: bool,

    #[arg(long)]
    #[arg(help = "Display only the specified field")]
    pub field: Option<String>,
}

impl DisplayOptions {
    pub fn display<T>(&self, value: T) -> Result<()>
    where
        T: Serialize + Pretty,
    {
        #[cfg(windows)]
        let _ = colored_json::enable_ansi_support();

        if self.json {
            if let Some(ref field) = self.field {
                self.display_json(
                    serde_json::to_value(&value)?
                        .get(field)
                        .context(format!("no such field exist: {field}"))?,
                )
            } else {
                self.display_json(&serde_json::to_value(&value)?)
            }
        } else {
            println!("{}", value.prettify());
            Ok(())
        }
    }

    fn display_json(&self, value: &serde_json::Value) -> Result<()> {
        println!("{}", colored_json::to_colored_json_auto(&value)?);
        Ok(())
    }
}
