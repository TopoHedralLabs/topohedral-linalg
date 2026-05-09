//! Symmetric eigendecomposition of a [`DMatrix`] via LAPACK `dsyev`/`ssyev`.
//!
//! Provides the `symeig()` method on [`DMatrix<T>`], computing all eigenvalues and eigenvectors
//! of a real symmetric square matrix. The `Syev` LAPACK driver is used, which exploits symmetry
//! for a significantly more efficient computation than the general `eig` path. Eigenvalues are
//! returned as real scalars in ascending order in a `Vec<T>`; eigenvectors are stored column-major
//! in the `Return<T>` struct. LAPACK errors propagate as a typed `Error`.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
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

//{{{ enum: Error
/// Errors that can occur during symmetric eigendecomposition.
#[derive(Error, Debug)]
pub enum Error
{
    #[error("Error in symeig(), exited with error:\n{0}")]
    /// LAPACK `syev` failed to compute eigenvalues or eigenvectors.
    GeevError(#[from] syev::Error),
}
//}}}

//{{{ struct: Return
/// Represents the eigenvalue decomposition of a symmetric matrix.
///
/// For symmetric matrices, the eigenvalues are always real, and the eigenvectors
/// form an orthogonal basis. The decomposition is of the form `A = QDQ^T`,
/// where `Q` is the matrix of eigenvectors, and `D` is the diagonal matrix of eigenvalues.
#[derive(Debug)]
pub struct Return<T>
where
    T: Field + Default + Copy,
{
    /// Matrix of eigenvectors (columns are the eigenvectors)
    pub eigvecs: DMatrix<T>,

    /// Real eigenvalues
    pub eigvals: Vec<T>,
}
//}}}

//{{{ impl DMatrix<T>
#[allow(private_bounds)]
impl<T> DMatrix<T>
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
    pub fn symeig(&self) -> Result<Return<T>, Error>
    {
        let n = self.nrows;
        if n != self.ncols
        {
            panic!("Matrix must be square for eigenvalue decomposition");
        }

        let mut a = self.clone();
        let mut eigvals = vec![T::zero(); n];

        // Query optimal workspace
        let mut work = vec![T::zero(); 1];
        T::syev(
            b'V', // Compute both eigenvalues and eigenvectors
            b'L', // Use lower triangular part of the matrix
            n as i32,
            &mut a.data,
            n as i32,
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
            n as i32,
            &mut a.data,
            n as i32,
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
//}}}
