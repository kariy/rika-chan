use clap::Args;
use eyre::{ensure, eyre, Result};
use serde::Serialize;
use serde_json::Value;

use crate::fmt::Pretty;

#[derive(Debug, thiserror::Error)]
#[error("Field not found: {field}.\nAvailable fields: {}", available_fields.join(", "))]
pub struct FieldNotFoundError {
    /// The field that was not found.
    field: String,
    /// The available fields in the object.
    available_fields: Vec<String>,
}

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

        if self.json || self.field.is_some() {
            self.display_json(value)?;
        } else {
            println!("{}", value.prettify());
        }

        Ok(())
    }

    fn display_json<T: Serialize>(&self, value: T) -> Result<()> {
        if let Some(ref field) = self.field {
            let value = serde_json::to_value(&value)?;

            ensure!(
                value.is_object(),
                "Unable to extract field '{field}'. Value is not an object."
            );

            match value.get(field) {
                Some(field) => {
                    println!("{}", colored_json::to_colored_json_auto(field)?);
                }

                None => {
                    return Err(eyre!(FieldNotFoundError {
                        field: field.clone(),
                        available_fields: keys(&value)
                    }));
                }
            }
        } else {
            println!("{}", colored_json::to_colored_json_auto(&value)?);
        }

        Ok(())
    }
}

fn keys(value: &Value) -> Vec<String> {
    value
        .as_object()
        .map(|obj| obj.keys().map(|s| s.to_string()).collect::<Vec<_>>())
        .unwrap_or_default()
}
