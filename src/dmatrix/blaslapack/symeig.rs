//! Symmetric eigendecomposition of a [`DMatrix`] via LAPACK `dsyev`/`ssyev`.
//!
//! Provides the `symeig()` method on [`DMatrix<T>`], computing all eigenvalues and eigenvectors
//! of a real symmetric square matrix. The `Syev` LAPACK driver is used, which exploits symmetry
//! for a significantly more efficient computation than the general `eig` path. Eigenvalues are
//! returned as real scalars in ascending order in a `Vec<T>`; eigenvectors are stored column-major
//! in the `Return<T>` struct. LAPACK errors propagate as a typed `Error`.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::{symeig_raw, LapackScalar, SymEigRawError};
use crate::dmatrix::DMatrix;
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
///
/// For symmetric matrices, the eigenvalues are always real, and the eigenvectors
/// form an orthogonal basis. The decomposition is of the form `A = QDQ^T`,
/// where `Q` is the matrix of eigenvectors, and `D` is the diagonal matrix of eigenvalues.
#[derive(Clone, Debug)]
pub struct Return<T> {
    /// Matrix of eigenvectors (columns are the eigenvectors)
    pub eigvecs: DMatrix<T>,

    /// Real eigenvalues
    pub eigvals: Vec<T>,
}
//}}}

//{{{ impl DMatrix<T>
impl<T> DMatrix<T>
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
    /// Panics if the matrix is not square or its dimensions exceed the LAPACK integer range.
    pub fn symeig(self) -> Result<Return<T>, Error> {
        let n = self.nrows;
        if n != self.ncols {
            panic!("matrix must be square for eigenvalue decomposition");
        }
        let raw = symeig_raw(self.data, n).map_err(|error| match error {
            SymEigRawError::LapackError(info) if info > 0 => Error::NoConvergence,
            SymEigRawError::LapackError(info) => Error::BackendFailure { info },
        })?;
        Ok(Return {
            eigvecs: DMatrix {
                data: raw.eigvecs_data,
                nrows: n,
                ncols: n,
            },
            eigvals: raw.eigvals,
        })
    }
}
//}}}
