use std::path::PathBuf;

use eyre::{Context, Result};

/// Canonicalizes a path and performs both tilde and environment expansions in the default system context.
pub fn canonicalize_path(path: &str) -> Result<PathBuf> {
    let expanded = shellexpand::full(path).context(format!("failed to expand path {path}"))?;
    let path = PathBuf::from(expanded.into_owned());
    Ok(dunce::canonicalize(path)?)
}
