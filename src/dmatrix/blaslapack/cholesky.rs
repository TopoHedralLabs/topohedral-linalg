//! Cholesky decomposition of a [`DMatrix`] via LAPACK `dpotrf`/`spotrf`.
//!
//! Provides the `cholesky()` method on square symmetric positive-definite [`DMatrix<T>`]
//! instances, computing the lower-triangular factor `L` such that `A = L L^T`. The factorisation
//! is delegated to LAPACK `Potrf`; failures, including non-positive-definite input, are reported
//! through a typed `Error`.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::{cholesky_raw, CholeskyRawError, LapackScalar};
use crate::dmatrix::DMatrix;
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: Error
/// Errors that can occur during Cholesky decomposition.
#[derive(Clone, Error, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The leading principal minor of this order was not positive definite.
    #[error("leading principal minor {minor} is not positive definite")]
    NotPositiveDefinite {
        /// One-based order of the failing leading principal minor.
        minor: usize,
    },
    /// The validated backend call unexpectedly rejected an argument.
    #[error("cholesky backend failed with info code {info}")]
    BackendFailure {
        /// Raw LAPACK diagnostic code.
        info: i32,
    },
}
//}}}
//{{{ struct: Return
/// Represents the Cholesky decomposition of a symmetric positive-definite matrix.
#[derive(Clone, Debug)]
pub struct Return<T> {
    /// Lower-triangular factor L such that A = L L^T.
    pub l: DMatrix<T>,
}
//}}}
//{{{ impl DMatrix<T>
impl<T> DMatrix<T>
where
    T: LapackScalar,
{
    /// Computes the Cholesky decomposition of the matrix.
    ///
    /// Factors `self` into `L` such that `self = L L^T`. Only the lower triangle of `self` is
    /// read by LAPACK.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NotPositiveDefinite`] when a leading principal minor is not positive
    /// definite, or [`Error::BackendFailure`] for an unexpected backend failure.
    /// # Panics
    ///
    /// Panics if the matrix is not square or its dimensions exceed the LAPACK integer range.
    pub fn cholesky(self) -> Result<Return<T>, Error> {
        let n = self.nrows;
        if n != self.ncols {
            panic!("matrix must be square for Cholesky decomposition");
        }
        let raw = cholesky_raw(self.data, n).map_err(|error| match error {
            CholeskyRawError::LapackError(info) if info > 0 => Error::NotPositiveDefinite {
                minor: info as usize,
            },
            CholeskyRawError::LapackError(info) => Error::BackendFailure { info },
        })?;
        Ok(Return {
            l: DMatrix {
                data: raw.l_data,
                nrows: n,
                ncols: n,
            },
        })
    }
}
//}}}
