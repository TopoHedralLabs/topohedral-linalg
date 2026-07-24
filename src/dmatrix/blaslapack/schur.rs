//! Schur decomposition of a [`DMatrix`] via LAPACK `dgees`/`sgees`.
//!
//! Provides the `schur()` method on [`DMatrix<T>`], computing the Schur decomposition A = Q T Q^H
//! where Q is orthogonal and T is quasi-upper-triangular (block upper-triangular with 1×1 and 2×2
//! diagonal blocks for real inputs). The factorisation is computed by the `Gees` LAPACK driver.
//! Results are returned in a `Return<T>` struct containing Q and T; LAPACK errors propagate as a
//! typed `Error`.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::{schur_raw, LapackScalar, ShurRawError};
use crate::dmatrix::DMatrix;
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: Error
/// Errors that can occur during Schur decomposition.
#[derive(Clone, Error, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The QR algorithm did not converge.
    #[error("Schur decomposition did not converge")]
    NoConvergence,
    /// The validated backend call unexpectedly rejected an argument.
    #[error("Schur backend failed with info code {info}")]
    BackendFailure {
        /// Raw LAPACK diagnostic code.
        info: i32,
    },
}
//}}}

//{{{ struct: Return
/// Represents the Schur decomposition of a square matrix.
///
/// The decomposition satisfies `A = Q T Q^H`, where `Q` is orthogonal and `T` is
/// quasi-upper-triangular (block upper-triangular with 1×1 and 2×2 diagonal blocks for real inputs).
#[derive(Clone, Debug)]
pub struct Return<T> {
    /// Orthogonal Schur vector matrix Q.
    pub q: DMatrix<T>,
    /// Quasi-upper-triangular Schur form T.
    pub t: DMatrix<T>,
}
//}}}

//{{{ impl DMatrix<T>
impl<T> DMatrix<T>
where
    T: LapackScalar,
{
    /// Computes the Schur decomposition of the matrix.
    ///
    /// Factors `self` into `Q` and `T` such that `A = Q T Q^H`, where `Q` is orthogonal and `T`
    /// is quasi-upper-triangular.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NoConvergence`] if the QR algorithm does not converge, or
    /// [`Error::BackendFailure`] for an unexpected backend failure.
    /// # Panics
    ///
    /// Panics if the matrix is not square or its dimensions exceed the LAPACK integer range.
    pub fn schur(self) -> Result<Return<T>, Error> {
        let n = self.nrows;
        assert_eq!(
            n, self.ncols,
            "matrix must be square for Schur decomposition"
        );
        let raw = schur_raw(self.data, n).map_err(|error| match error {
            ShurRawError::LapackError(info) if info > 0 => Error::NoConvergence,
            ShurRawError::LapackError(info) => Error::BackendFailure { info },
        })?;
        Ok(Return {
            q: DMatrix {
                data: raw.q_data,
                nrows: n,
                ncols: n,
            },
            t: DMatrix {
                data: raw.t_data,
                nrows: n,
                ncols: n,
            },
        })
    }
}
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests {}
//}}}
