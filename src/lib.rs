//! # tomllm
//!
//! Structured TOML annotations for AI-augmented applications.
//!
//! `.tomllm` files are **valid TOML** extended with structured `#` comment conventions:
//! - Annotation prefixes (`@example:`, `@requires:`, `@deprecated:`, `@note:`, `@tribal:`)
//!   associate machine-readable metadata with the next key-value pair
//! - A tail-map block at the end of any file enables fast metadata extraction without
//!   parsing the full document
//! - `strip_for_pipeline()` removes all annotation comments, yielding clean TOML/JSON
//!   for downstream consumers
//!
//! ## Example
//! ```toml
//! # @example: uv pip install requests
//! # @requires: Python 3.9+
//! package_manager = "uv"
//!
//! # tomllm:map v1
//! # summary: Python toolchain configuration
//! # tags: python, packaging
//! # complexity: 2
//! ```

use thiserror::Error;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub mod loader;
pub mod map_block;
pub mod parser;
pub mod registry;
pub mod stripper;

#[cfg(feature = "python")]
pub mod python;

pub use loader::{load_any_typed, load_first, load_typed, resolve_path};
pub use map_block::{CognitiveTier, MapBlock};
pub use parser::TomllmDoc;
pub use registry::TomllmRegistry;
// 🤓 define_typed_registry! is #[macro_export] — already at crate root; no pub use needed

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// Python module entry point — called by maturin-generated C extension
#[cfg(feature = "python")]
#[pymodule]
fn tomllm_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    python::tomllm(m)
}

#[derive(Debug, Error)]
pub enum TomllmError {
    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Map block parse error: {0}")]
    MapBlock(String),
}

pub type Result<T> = std::result::Result<T, TomllmError>;
