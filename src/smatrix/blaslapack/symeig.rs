//! Symmetric eigendecomposition of an [`SMatrix`] via LAPACK `dsyev`/`ssyev`.
//!
//! Provides the `symeig()` method on square symmetric [`SMatrix<T, N, N>`] instances. Returns a
//! const-generic `Return<T, N>` with the eigenvector matrix (`SMatrix<T, N, N>`) and eigenvalues
//! as a fixed-size array `[T; N]` in ascending order. Exploits symmetry via the `Syev` driver
//! for a more efficient computation than the general `eig` path.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::{symeig_raw, LapackScalar, SymEigRawError};
use crate::smatrix::SMatrix;
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: Error
/// Errors that can occur during symmetric eigendecomposition.
#[derive(Clone, Error, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The eigensolver did not converge.
    #[error("symmetric eigendecomposition did not converge")]
    NoConvergence,
    /// The validated backend call unexpectedly rejected an argument.
    #[error("symmetric eigendecomposition backend failed with info code {info}")]
    BackendFailure {
        /// Raw LAPACK diagnostic code.
        info: i32,
    },
}
//}}}

//{{{ struct: Return
/// Represents the eigenvalue decomposition of a symmetric matrix.
#[derive(Clone, Debug)]
pub struct Return<T, const N: usize> {
    /// Matrix of eigenvectors (columns are the eigenvectors)
    pub eigvecs: SMatrix<T, N, N>,

    /// Real eigenvalues
    pub eigvals: [T; N],
}
//}}}

//{{{ impl: SMatrix<T, N, N>
impl<T, const N: usize> SMatrix<T, N, N>
where
    T: LapackScalar,
{
    /// Computes the eigendecomposition of a symmetric matrix.
    ///
    /// Returns a tuple containing the eigenvectors and eigenvalues.
    /// The eigenvalues are sorted in ascending order.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NoConvergence`] if the algorithm does not converge, or
    /// [`Error::BackendFailure`] for an unexpected backend failure.
    /// # Panics
    ///
    /// Panics if `N` exceeds the LAPACK integer range.
    pub fn symeig(self) -> Result<Return<T, N>, Error> {
        let raw = symeig_raw(self.into_iter().collect(), N).map_err(|error| match error {
            SymEigRawError::LapackError(info) if info > 0 => Error::NoConvergence,
            SymEigRawError::LapackError(info) => Error::BackendFailure { info },
        })?;
        let eigvals: [T; N] = raw.eigvals.try_into().unwrap_or_else(|_| unreachable!());
        Ok(Return {
            eigvecs: SMatrix::from_col_vec(raw.eigvecs_data),
            eigvals,
        })
    }
}
//}}}
