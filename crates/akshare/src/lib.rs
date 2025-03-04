//! # Example
//!
//! ```
//! let ak = AKShare::new()?;
//! let df = ak.stock_szse_summary().date("2025-01-01").call()?;
//! dbg!(df);
//!
//! ```

pub use crate::base::*;

mod base;
mod ops;
