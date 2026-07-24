//! General (non-symmetric) eigendecomposition of a [`DMatrix`] via LAPACK `dgeev`/`sgeev`.
//!
//! Provides the `eig()` method on [`DMatrix<T>`], computing all eigenvalues and both the left
//! and right eigenvectors of a general square matrix. The computation is delegated to the
//! `Geev` LAPACK driver. Eigenvalues are returned as complex numbers even when the input is
//! real-valued; eigenvector matrices are stored column-major in the `Return<T>` struct. LAPACK
//! errors propagate as a typed `Error`.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::{eig_raw, EigRawError, LapackScalar};
use crate::common::Complex;
use crate::dmatrix::DMatrix;
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
///
/// The eigenvalue decomposition of a matrix `A` is a factorization of the form `A = PDP^-1`,
/// where `P` is the matrix of right eigenvectors, `D` is the diagonal matrix of eigenvalues,
/// and `P^-1` is the matrix of left eigenvectors.
///
/// This struct contains the left and right eigenvectors, as well as the eigenvalues, of the
/// decomposition.
#[derive(Clone, Debug)]
pub struct Return<T> {
    /// Matrix whose columns are the left eigenvectors of A.
    pub left_eigvecs: DMatrix<T>,
    /// Matrix whose columns are the right eigenvectors of A.
    pub right_eigvecs: DMatrix<T>,
    /// Eigenvalues as complex numbers (real part from `wr`, imaginary part from `wi`).
    pub eigvals: Vec<Complex<T>>,
}
//}}}

//{{{ impl DMatrix<T>
impl<T> DMatrix<T>
where
    T: LapackScalar,
{
    /// Computes the eigendecomposition of a general square matrix.
    ///
    /// Returns all eigenvalues and both left and right eigenvectors of `self`. Eigenvalues are
    /// represented as complex numbers even when the input matrix is real-valued.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NoConvergence`] if the QR algorithm does not converge, or
    /// [`Error::BackendFailure`] for an unexpected backend failure.
    /// # Panics
    ///
    /// Panics if the matrix is not square or its dimensions exceed the LAPACK integer range.
    pub fn eig(self) -> Result<Return<T>, Error> {
        let n = self.nrows;
        assert_eq!(
            n, self.ncols,
            "matrix must be square for eigendecomposition"
        );
        let raw = eig_raw(self.data, n).map_err(|error| match error {
            EigRawError::LapackError(info) if info > 0 => Error::NoConvergence,
            EigRawError::LapackError(info) => Error::BackendFailure { info },
        })?;
        Ok(Return {
            left_eigvecs: DMatrix {
                data: raw.vl,
                nrows: n,
                ncols: n,
            },
            right_eigvecs: DMatrix {
                data: raw.vr,
                nrows: n,
                ncols: n,
            },
            eigvals: raw.eigvals,
        })
    }
}
//}}}
