use std::sync::Arc;

use arrow::array::RecordBatch;
use pyo3::{
    types::{IntoPyDict, PyAnyMethods, PyDict, PyModule, PyTuple},
    Bound, IntoPyObject, Py, PyAny, PyResult, Python,
};
use pyo3_arrow::PyRecordBatch;

#[derive(Clone)]
pub struct AKShare(Arc<AKShareInner>);

struct AKShareInner {
    /// `akshare` module.
    akshare: Py<PyModule>,
    /// Python Arrow implementation。
    arrow: PyArrowModule,
}

impl AKShare {
    /// Initialize the interface.
    pub fn new() -> PyResult<Self> {
        Python::with_gil(|py| {
            let akshare = py.import("akshare")?.unbind();
            let arrow = PyArrowModule::new(py)?;
            let inner = AKShareInner { akshare, arrow };
            Ok(Self(Arc::new(inner)))
        })
    }

    /// General API call.
    pub fn call<'py, A>(
        &self,
        py: Python<'py>,
        name: &str,
        args: A,
        kwargs: Option<&Bound<'py, PyDict>>,
        preserve_index: bool,
    ) -> PyResult<RecordBatch>
    where
        A: IntoPyObject<'py, Target = PyTuple>,
    {
        let df = self.0.akshare.bind(py).getattr(name)?.call(args, kwargs)?;
        self.0
            .arrow
            .pandas_dataframe_to_rust(py, df, preserve_index)
    }
}

/// Wrap different impls supported by `pyo3-arrow`.
///
/// Currently only `pyarrow` is supported.
enum PyArrowModule {
    PyArrow(
        /// `pyarrow.RecordBatch.from_pandas` static method。
        Py<PyAny>,
    ),
}

impl PyArrowModule {
    fn new<'py>(py: Python<'py>) -> PyResult<Self> {
        let pyarrow = py.import("pyarrow")?;
        let record_batch_class = pyarrow.getattr("RecordBatch")?;
        let from_pandas_method = record_batch_class.getattr("from_pandas")?;
        Ok(Self::PyArrow(from_pandas_method.unbind()))
    }

    /// turn `pandas.DataFrame` into`arrow::RecordBatch`.
    fn pandas_dataframe_to_rust<'py>(
        &self,
        py: Python<'py>,
        df: Bound<'py, PyAny>,
        preserve_index: bool,
    ) -> PyResult<RecordBatch> {
        let kwargs = [("preserve_index", preserve_index)].into_py_dict(py)?;
        match self {
            PyArrowModule::PyArrow(method) => Ok(method
                .bind(py)
                .call((df,), Some(&kwargs))?
                .extract::<PyRecordBatch>()?
                .into_inner()),
        }
    }
}
