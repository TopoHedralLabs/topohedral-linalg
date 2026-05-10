//! General (non-symmetric) eigendecomposition of a [`DMatrix`] via LAPACK `dgeev`/`sgeev`.
//!
//! Provides the `eig()` method on [`DMatrix<T>`], computing all eigenvalues and both the left
//! and right eigenvectors of a general square matrix. The computation is delegated to the
//! `Geev` LAPACK driver. Eigenvalues are returned as complex numbers even when the input is
//! real-valued; eigenvector matrices are stored column-major in the `Return<T>` struct. LAPACK
//! errors propagate as a typed `Error`.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::dmatrix::DMatrix;
use crate::blaslapack::AsI32;
use crate::blaslapack::{eig_raw, Geev, EigRawError};
use crate::common::{Complex, Field, One, Zero};
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
    #[error("Error in eig(), exited with error:\n{0}")]
    /// LAPACK `geev` failed to compute eigenvalues or eigenvectors.
    GeevError(#[from] EigRawError),
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
#[derive(Debug)]
pub struct Return<T>
where
    T: Field + Default + Copy,
{
    /// Matrix whose columns are the left eigenvectors of A.
    pub left_eigvecs: DMatrix<T>,
    /// Matrix whose columns are the right eigenvectors of A.
    pub right_eigvecs: DMatrix<T>,
    /// Eigenvalues as complex numbers (real part from `wr`, imaginary part from `wi`).
    pub eigvals: Vec<Complex<T>>,
}
//}}}

//{{{ impl DMatrix<T>
#[allow(private_bounds)]
impl<T> DMatrix<T>
where
    T: One + Zero + Geev + Field + Default + Copy + AsI32,
{
    /// Computes the eigendecomposition of a general square matrix.
    ///
    /// Returns all eigenvalues and both left and right eigenvectors of `self`. Eigenvalues are
    /// represented as complex numbers even when the input matrix is real-valued.
    ///
    /// # Errors
    ///
    /// Returns [`Error::GeevError`] if the LAPACK `geev` routine fails.
    pub fn eig(&self) -> Result<Return<T>, Error>
    {
        let n = self.nrows;
        let raw = eig_raw(self.data.clone(), n)?;
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
