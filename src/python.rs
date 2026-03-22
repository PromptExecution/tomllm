//! PyO3 Python bindings for tomllm
//!
//! Exposes TomllmDoc and MapBlock to Python via pyo3/maturin.
//! Build: `maturin build --features python`

use pyo3::prelude::*;

use crate::map_block::{CognitiveTier, MapBlock};
use crate::parser::TomllmDoc;

/// Python wrapper for MapBlock
#[pyclass(name = "MapBlock")]
#[derive(Clone)]
pub struct PyMapBlock {
    inner: MapBlock,
}

#[pymethods]
impl PyMapBlock {
    /// Parse a map block from arbitrary text (scans last 10 lines)
    #[staticmethod]
    pub fn from_text(text: &str) -> Option<Self> {
        MapBlock::scan_tail(text, 10).map(|inner| PyMapBlock { inner })
    }

    #[getter]
    pub fn summary(&self) -> Option<String> {
        self.inner.summary.clone()
    }

    #[getter]
    pub fn tags(&self) -> Vec<String> {
        self.inner.tags.clone()
    }

    #[getter]
    pub fn tier(&self) -> Option<String> {
        self.inner.tier.as_ref().map(|t| t.as_str().to_string())
    }

    #[getter]
    pub fn cmds(&self) -> Vec<String> {
        self.inner.cmds.clone()
    }

    #[getter]
    pub fn complexity(&self) -> Option<u8> {
        self.inner.complexity
    }

    pub fn __repr__(&self) -> String {
        format!(
            "MapBlock(summary={:?}, tier={:?}, tags={:?})",
            self.inner.summary,
            self.inner.tier.as_ref().map(|t| t.as_str()),
            self.inner.tags
        )
    }
}

/// Python wrapper for TomllmDoc
#[pyclass(name = "TomllmDoc")]
pub struct PyTomllmDoc {
    inner: TomllmDoc,
}

#[pymethods]
impl PyTomllmDoc {
    /// Parse a .tomllm string
    #[staticmethod]
    pub fn parse(text: &str) -> PyResult<Self> {
        TomllmDoc::parse(text)
            .map(|inner| PyTomllmDoc { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Return stripped TOML (no comments) as JSON string — for downstream pipelines
    pub fn strip_for_pipeline(&self) -> String {
        self.inner.strip_for_pipeline()
    }

    /// Return all annotation strings (flat list, all keys)
    pub fn annotations(&self) -> Vec<String> {
        self.inner
            .annotations
            .values()
            .flat_map(|v| v.iter().cloned())
            .collect()
    }

    /// Get annotations for a specific dot-path key
    pub fn get_annotations(&self, key_path: &str) -> Vec<String> {
        self.inner
            .get_annotations(key_path)
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    /// Cognitive tier as string: "sm0l", "ch0nky", or "frontier"
    pub fn cognitive_tier(&self) -> String {
        self.inner.cognitive_tier().as_str().to_string()
    }

    /// Summary from tail-map or [b00t].hint fallback
    pub fn summary(&self) -> Option<String> {
        self.inner.summary()
    }

    /// MapBlock if present
    pub fn map_block(&self) -> Option<PyMapBlock> {
        self.inner.map_block.clone().map(|inner| PyMapBlock { inner })
    }

    pub fn __repr__(&self) -> String {
        format!(
            "TomllmDoc(tier={}, summary={:?})",
            self.inner.cognitive_tier(),
            self.inner.summary()
        )
    }
}

/// Register the tomllm Python module
#[pymodule]
pub fn tomllm(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTomllmDoc>()?;
    m.add_class::<PyMapBlock>()?;
    Ok(())
}
