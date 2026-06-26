//! Symmetric eigendecomposition of an [`SMatrix`] via LAPACK `dsyev`/`ssyev`.
//!
//! Provides the `symeig()` method on square symmetric [`SMatrix<T, N, N>`] instances. Returns a
//! const-generic `Return<T, N>` with the eigenvector matrix (`SMatrix<T, N, N>`) and eigenvalues
//! as a fixed-size array `[T; N]` in ascending order. Exploits symmetry via the `Syev` driver
//! for a more efficient computation than the general `eig` path.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::AsI32;
use crate::blaslapack::{symeig_raw, Syev, SymEigRawError};
use crate::common::{Field, One, Zero};
use crate::smatrix::SMatrix;
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: Error
/// Errors that can occur during symmetric eigendecomposition.
#[derive(Error, Debug)]
pub enum Error {
    /// Wraps a LAPACK `syev` error from the symmetric eigenvalue routine.
    #[error("Error in symeig(), exited with error:\n{0}")]
    SyevError(#[from] SymEigRawError),
}
//}}}

//{{{ struct: Return
/// Represents the eigenvalue decomposition of a symmetric matrix.
#[derive(Debug)]
pub struct Return<T, const N: usize>
where
    T: Field + Default + Copy,
{
    /// Matrix of eigenvectors (columns are the eigenvectors)
    pub eigvecs: SMatrix<T, N, N>,

    /// Real eigenvalues
    pub eigvals: [T; N],
}
//}}}

//{{{ impl: SMatrix<T, N, N>
#[allow(private_bounds)]
impl<T, const N: usize> SMatrix<T, N, N>
where
    T: One + Zero + Syev + Field + Default + Copy + AsI32,
{
    /// Computes the eigendecomposition of a symmetric matrix.
    ///
    /// Returns a tuple containing the eigenvectors and eigenvalues.
    /// The eigenvalues are sorted in ascending order.
    ///
    /// # Errors
    ///
    /// Returns an error if the LAPACK routine fails.
    pub fn symeig(&self) -> Result<Return<T, N>, Error> {
        let raw = symeig_raw(self.as_slice().to_vec(), N)?;
        let eigvals: [T; N] = raw.eigvals.try_into().unwrap_or_else(|_| unreachable!());
        Ok(Return {
            eigvecs: SMatrix::from_col_vec(raw.eigvecs_data),
            eigvals,
        })
    }
}
//}}}
