//! Eigenvalue decomposition for symmetric matrices.
//!
//! This module provides functionality to compute eigenvalues and eigenvectors
//! of symmetric matrices, which is more efficient than the general eigenvalue
//! decomposition for asymmetric matrices.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::blaslapack::common::AsI32;
use crate::blaslapack::syev::{self, Syev};
use crate::common::{Field, One, Zero};
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

#[derive(Error, Debug)]
pub enum Error
{
    #[error("Error in symeig(), exited with error:\n{0}")]
    SyevError(#[from] syev::Error),
}

//{{{ struct: Return
/// Represents the eigenvalue decomposition of a symmetric matrix.
///
/// For symmetric matrices, the eigenvalues are always real, and the eigenvectors
/// form an orthogonal basis. The decomposition is of the form `A = QDQ^T`,
/// where `Q` is the matrix of eigenvectors, and `D` is the diagonal matrix of eigenvalues.
#[derive(Debug)]
pub struct Return<T, const N: usize>
where
    [(); N * N]:,
    T: Field + Default + Copy,
{
    /// Matrix of eigenvectors (columns are the eigenvectors)
    pub eigvecs: SMatrix<T, N, N>,

    /// Real eigenvalues
    pub eigvals: [T; N],
}
//}}}

#[allow(private_bounds)]
impl<T, const N: usize> SMatrix<T, N, N>
where
    [(); N * N]:,
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
    pub fn symeig(&self) -> Result<Return<T, N>, Error>
    {
        // SMatrix is always square, so no need to check for squareness

        let mut a = *self;
        let mut eigvals = [T::zero(); N];

        // Query optimal workspace
        let mut work = vec![T::zero(); 1];
        T::syev(
            b'V', // Compute both eigenvalues and eigenvectors
            b'L', // Use lower triangular part of the matrix
            N as i32,
            &mut a.data,
            N as i32,
            &mut eigvals,
            &mut work,
            -1, // Workspace query
        )?;

        // Perform eigenvalue decomposition
        let lwork = work[0].as_i32();
        let mut work = vec![T::zero(); lwork as usize];
        T::syev(
            b'V',
            b'L',
            N as i32,
            &mut a.data,
            N as i32,
            &mut eigvals,
            &mut work,
            lwork,
        )?;

        Ok(Return {
            eigvecs: a,
            eigvals,
        })
    }
}
