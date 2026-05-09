//! QR decomposition of a [`DMatrix`] via LAPACK `dgeqrf`/`sgeqrf` and `dorgqr`/`sorgqr`.
//!
//! Provides the `qr()` method on [`DMatrix<T>`], computing the factorisation A = QR where Q is
//! an orthogonal matrix and R is upper-triangular. The implementation calls `Geqrf` to produce
//! the compact Householder representation and calls `Orgqr` to expand Q into an explicit
//! orthogonal matrix. An optimal BLAS workspace size is queried before the main computation.
//! Results are returned in a `Return<T>` struct; errors from either LAPACK call are aggregated
//! into a single typed `Error`.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::blaslapack::common::AsI32;
use crate::blaslapack::geqrf;
use crate::blaslapack::geqrf::Geqrf;
use crate::blaslapack::orgqr;
use crate::blaslapack::orgqr::Orgqr;
use crate::common::{Field, One, Zero};
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: Error
/// Errors that can occur during QR decomposition.
#[derive(Error, Debug)]
pub enum Error
{
    #[error("Error in qr(), exited with error:\n{0}")]
    /// LAPACK `geqrf` failed while computing the Householder QR factorisation.
    GetrfError(#[from] geqrf::Error),
    #[error("Error in qr(), exited with error:\n{0}")]
    /// LAPACK `orgqr` failed while expanding the Householder reflectors into an explicit Q matrix.
    OrgqrError(#[from] orgqr::Error),
}
//}}}
//{{{ struct: Return
/// Represents the QR decomposition of a matrix.
///
/// The decomposition satisfies `A = QR` where `Q` is an orthogonal matrix and `R` is
/// upper-triangular.
pub struct Return<T>
where
    T: Field + Copy,
{
    /// Orthogonal factor Q.
    pub q: DMatrix<T>,
    /// Upper-triangular factor R.
    pub r: DMatrix<T>,
}
//}}}
//{{{ impl: SMatrix<T, N, M>
impl<T> DMatrix<T>
where
    T: One + Zero + Geqrf + Orgqr + Field + Copy + AsI32,
{
    /// Computes the QR decomposition of the matrix.
    ///
    /// Factors `self` into `Q` and `R` such that `A = QR`, where `Q` is orthogonal and `R` is
    /// upper-triangular. An optimal BLAS workspace size is queried before the main computation.
    ///
    /// # Errors
    ///
    /// Returns [`Error::GetrfError`] if `geqrf` fails, or [`Error::OrgqrError`] if `orgqr` fails.
    pub fn qr(&self) -> Result<Return<T>, Error>
    {
        let n = self.nrows;
        let m = self.ncols;
        let mut a = self.clone();
        let k = self.nrows.min(self.ncols);
        let mut tau = vec![T::zero(); k];

        // Query optimal workspace
        let mut work = vec![T::zero(); 1];
        T::geqrf(
            n as i32,
            m as i32,
            &mut a.data,
            n as i32,
            &mut tau,
            &mut work,
            -1,
        )?;

        // Perform QR factorization
        let lwork = work[0].as_i32();
        let mut work = vec![T::zero(); lwork as usize];
        T::geqrf(
            n as i32,
            m as i32,
            &mut a.data,
            n as i32,
            &mut tau,
            &mut work,
            lwork,
        )?;

        // Extract R matrix (upper triangular part)
        let mut r = DMatrix::<T>::zeros(n, m);
        for i in 0..n
        {
            for j in i..m
            {
                r[(i, j)] = a[(i, j)];
            }
        }

        // Generate Q matrix
        T::orgqr(
            n as i32,
            n.min(m) as i32,
            k as i32,
            &mut a.data,
            n as i32,
            &tau,
            &mut work,
            lwork,
        )?;
        let q = a;
        Ok(Return { q, r })
    }
}
//}}}
