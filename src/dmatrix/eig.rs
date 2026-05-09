//! General (non-symmetric) eigendecomposition of a [`DMatrix`] via LAPACK `dgeev`/`sgeev`.
//!
//! Provides the `eig()` method on [`DMatrix<T>`], computing all eigenvalues and both the left
//! and right eigenvectors of a general square matrix. The computation is delegated to the
//! [`Geev`] LAPACK driver. Eigenvalues are returned as complex numbers even when the input is
//! real-valued; eigenvector matrices are stored column-major in the `Return<T>` struct. LAPACK
//! errors propagate as a typed `Error`.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::blaslapack::common::AsI32;
use crate::blaslapack::geev::{self, Geev};
use crate::common::{Complex, Field, One, Zero};
//}}}
//{{{ std imports
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
    GeevError(#[from] geev::Error),
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
        let mut a = self.clone();
        let mut vl = DMatrix::<T>::zeros(n, n);
        let mut vr = DMatrix::<T>::zeros(n, n);
        let nelem = self.nrows * self.ncols;
        let mut wr = vec![T::zero(); nelem];
        let mut wi = vec![T::zero(); nelem];

        // Query optimal workspace
        let mut work = vec![T::zero(); 1];
        T::geev(
            b'V',
            b'V',
            n as i32,
            &mut a.data,
            n as i32,
            &mut wr,
            &mut wi,
            &mut vl.data,
            n as i32,
            &mut vr.data,
            n as i32,
            &mut work,
            -1,
        )?;

        // Perform eigenvalue decomposition
        let lwork = work[0].as_i32();
        let mut work = vec![T::zero(); lwork as usize];
        T::geev(
            b'V',
            b'V',
            n as i32,
            &mut a.data,
            n as i32,
            &mut wr,
            &mut wi,
            &mut vl.data,
            n as i32,
            &mut vr.data,
            n as i32,
            &mut work,
            lwork,
        )?;

        let eigvals: Vec<Complex<T>> = wr
            .iter()
            .zip(wi.iter())
            .map(|(&wr_val, &wi_val)| Complex::new(wr_val, wi_val))
            .take(n)
            .collect();

        Ok(Return {
            left_eigvecs: vl,
            right_eigvecs: vr,
            eigvals,
        })
    }
}
//}}}
