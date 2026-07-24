//! General eigendecomposition of an [`SMatrix`] via LAPACK `dgeev`/`sgeev`.
//!
//! Provides the `eig()` method on square [`SMatrix<T, N, N>`] instances. Returns a const-generic
//! `Return<T, N>` containing left and right eigenvector matrices (`SMatrix<T, N, N>`) and
//! eigenvalues as a fixed-size array `[Complex<T>; N]`. Because the dimensions are compile-time
//! constants, no heap allocation is needed for the result matrices.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::{eig_raw, EigRawError, LapackScalar};
use crate::common::Complex;
use crate::smatrix::SMatrix;
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: Error
/// Errors that can occur during general eigendecomposition.
#[derive(Clone, Error, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The QR algorithm did not converge.
    #[error("eigendecomposition did not converge")]
    NoConvergence,
    /// The validated backend call unexpectedly rejected an argument.
    #[error("eigendecomposition backend failed with info code {info}")]
    BackendFailure {
        /// Raw LAPACK diagnostic code.
        info: i32,
    },
}
//}}}

//{{{ struct: Return
/// Represents the eigenvalue decomposition of a square matrix of size `N`.
#[derive(Clone, Debug)]
pub struct Return<T, const N: usize> {
    /// Matrix whose columns are the left eigenvectors.
    pub left_eigvecs: SMatrix<T, N, N>,
    /// Matrix whose columns are the right eigenvectors.
    pub right_eigvecs: SMatrix<T, N, N>,
    /// Complex eigenvalues in the order returned by LAPACK.
    pub eigvals: [Complex<T>; N],
}
//}}}

//{{{ impl: SMatrix<T, N, N>
impl<T, const N: usize> SMatrix<T, N, N>
where
    T: LapackScalar,
{
    /// Computes the general (non-symmetric) eigendecomposition of the square matrix.
    ///
    /// Returns left eigenvectors, right eigenvectors, and complex eigenvalues.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NoConvergence`] if the QR algorithm does not converge, or
    /// [`Error::BackendFailure`] for an unexpected backend failure.
    /// # Panics
    ///
    /// Panics if `N` exceeds the LAPACK integer range.
    pub fn eig(self) -> Result<Return<T, N>, Error> {
        let raw = eig_raw(self.into_iter().collect(), N).map_err(|error| match error {
            EigRawError::LapackError(info) if info > 0 => Error::NoConvergence,
            EigRawError::LapackError(info) => Error::BackendFailure { info },
        })?;
        let eigvals: [Complex<T>; N] = std::array::from_fn(|i| raw.eigvals[i]);
        Ok(Return {
            left_eigvecs: SMatrix::from_col_vec(raw.vl),
            right_eigvecs: SMatrix::from_col_vec(raw.vr),
            eigvals,
        })
    }
}
//}}}
