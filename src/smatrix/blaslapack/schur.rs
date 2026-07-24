//! Schur decomposition of an [`SMatrix`] via LAPACK `dgees`/`sgees`.
//!
//! Provides the `schur()` method on [`SMatrix<T, N, N>`], computing A = Q T Q^H. The static
//! `Return<T, N>` struct carries Q and T as square [`SMatrix`] instances.
//! The implementation delegates to `Gees` and is the static counterpart of the [`DMatrix`](crate::dmatrix::DMatrix)
//! Schur decomposition.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::{schur_raw, LapackScalar, ShurRawError};
use crate::smatrix::SMatrix;
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
/// Result of a Schur decomposition: orthogonal factor Q and quasi-upper-triangular Schur matrix T.
#[derive(Clone, Debug)]
pub struct Return<T, const N: usize> {
    /// Orthogonal (unitary) transformation matrix Q such that A = Q T Q^H.
    pub q: SMatrix<T, N, N>,
    /// Quasi-upper-triangular Schur matrix T.
    pub t: SMatrix<T, N, N>,
}
//}}}
//{{{ impl: SMatrix<T, N, N>
impl<T, const N: usize> SMatrix<T, N, N>
where
    T: LapackScalar,
{
    /// Computes the Schur decomposition A = Q T Q^H of the matrix.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NoConvergence`] if the QR algorithm does not converge, or
    /// [`Error::BackendFailure`] for an unexpected backend failure.
    /// # Panics
    ///
    /// Panics if `N` exceeds the LAPACK integer range.
    pub fn schur(self) -> Result<Return<T, N>, Error> {
        let raw = schur_raw(self.into_iter().collect(), N).map_err(|error| match error {
            ShurRawError::LapackError(info) if info > 0 => Error::NoConvergence,
            ShurRawError::LapackError(info) => Error::BackendFailure { info },
        })?;
        Ok(Return {
            q: SMatrix::from_col_vec(raw.q_data),
            t: SMatrix::from_col_vec(raw.t_data),
        })
    }
}
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests {}
//}}}
