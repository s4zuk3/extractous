// Expose the extract-rs rust core as `ecore`.
// We will use `ecore::Xxx` to represents all types from extract-rs rust core.
pub use ::extract_rs as ecore;
use pyo3::prelude::*;

//use pyo3::exceptions::PyTypeError;

// Modules
mod errors;
//pub use errors::*;
mod extractor;
pub use extractor::*;
mod config;
//pub use config::*;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _extractrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CharSet>()?;
    m.add_class::<StreamReader>()?;
    m.add_class::<Extractor>()?;
    Ok(())
}
