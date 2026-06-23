//! General eigendecomposition of an [`SMatrix`] via LAPACK `dgeev`/`sgeev`.
//!
//! Provides the `eig()` method on square [`SMatrix<T, N, N>`] instances. Returns a const-generic
//! `Return<T, N>` containing left and right eigenvector matrices (`SMatrix<T, N, N>`) and
//! eigenvalues as a fixed-size array `[Complex<T>; N]`. Because the dimensions are compile-time
//! constants, no heap allocation is needed for the result matrices.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::AsI32;
use crate::blaslapack::{eig_raw, EigRawError, Geev};
use crate::common::{Complex, Field, One, Zero};
use crate::smatrix::SMatrix;
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: Error
/// Errors that can occur during general eigendecomposition.
#[derive(Error, Debug)]
pub enum Error
{
    /// Wraps a LAPACK `geev` error from the eigenvalue computation routine.
    #[error("Error in eig(), exited with error:\n{0}")]
    GeevError(#[from] EigRawError),
}
//}}}

//{{{ struct: Return
/// Represents the eigenvalue decomposition of a square matrix of size `N`.
#[derive(Debug)]
pub struct Return<T, const N: usize>
where
    T: Field + Default + Copy,
{
    /// Matrix whose columns are the left eigenvectors.
    pub left_eigvecs: SMatrix<T, N, N>,
    /// Matrix whose columns are the right eigenvectors.
    pub right_eigvecs: SMatrix<T, N, N>,
    /// Complex eigenvalues in the order returned by LAPACK.
    pub eigvals: [Complex<T>; N],
}
//}}}

//{{{ impl: SMatrix<T, N, N>
#[allow(private_bounds)]
impl<T, const N: usize> SMatrix<T, N, N>
where
    T: One + Zero + Geev + Field + Default + Copy + AsI32,
{
    /// Computes the general (non-symmetric) eigendecomposition of the square matrix.
    ///
    /// Returns left eigenvectors, right eigenvectors, and complex eigenvalues.
    ///
    /// # Errors
    ///
    /// Returns an error if the LAPACK `geev` routine fails.
    pub fn eig(&self) -> Result<Return<T, N>, Error>
    {
        let raw = eig_raw(self.as_slice().to_vec(), N)?;
        let eigvals: [Complex<T>; N] = std::array::from_fn(|i| raw.eigvals[i]);
        Ok(Return {
            left_eigvecs: SMatrix::from_col_vec(raw.vl),
            right_eigvecs: SMatrix::from_col_vec(raw.vr),
            eigvals,
        })
    }
}
//}}}
