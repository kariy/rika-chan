use std::marker::PhantomData;

use clap::Args;
use color_eyre::{
    eyre::{ensure, eyre},
    Result,
};
use serde::Serialize;
use serde_json::Value;

use super::RawDisplay;

#[derive(Debug, Args)]
pub struct JsonDisplay<T: Serialize> {
    #[arg(short, long)]
    #[arg(help = "Display the output in its raw JSON format")]
    json: bool,

    #[arg(long)]
    #[arg(help = "Display only the specified field")]
    pub field: Option<String>,

    #[arg(skip)]
    _phantom: PhantomData<T>,
}

#[derive(Debug, thiserror::Error)]
#[error("Field '{field}' doesn't exist.\nAvailable fields: {}", available_fields.join(", "))]
pub struct FieldNotFoundError {
    /// The field that was not found.
    field: String,
    /// The available fields in the object.
    available_fields: Vec<String>,
}

impl<T: Serialize> JsonDisplay<T> {
    fn display_json(&self, value: T) -> Result<String> {
        if let Some(ref field) = self.field {
            let value = serde_json::to_value(&value)?;

            ensure!(
                value.is_object(),
                "Unable to extract field '{field}'. Value is not an object."
            );

            // TODO: allow specifying nested fields using dot notation (e.g. "block.number") in the cli
            match value.get(field) {
                Some(field) => Ok(colored_json::to_colored_json_auto(field)?),

                None => {
                    return Err(eyre!(FieldNotFoundError {
                        field: field.clone(),
                        available_fields: keys(&value)
                    }));
                }
            }
        } else {
            Ok(colored_json::to_colored_json_auto(&value)?)
        }
    }
}

impl<T: Serialize> RawDisplay for JsonDisplay<T> {
    type Value = T;

    fn display_raw(&self, value: Self::Value) -> Result<impl std::fmt::Display> {
        #[cfg(windows)]
        let _ = colored_json::enable_ansi_support();
        Ok(self.display_json(value)?)
    }

    fn is_raw(&self) -> bool {
        self.json || self.field.is_some()
    }
}

fn keys(value: &Value) -> Vec<String> {
    value
        .as_object()
        .map(|obj| obj.keys().map(|s| s.to_string()).collect::<Vec<_>>())
        .unwrap_or_default()
}
