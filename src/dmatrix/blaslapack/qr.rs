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
use crate::blaslapack::{qr_raw, AsI32, Geqrf, Orgqr, QrRawError};
use crate::common::{Field, One, Zero};
use crate::dmatrix::DMatrix;
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
    GetrfError(#[from] QrRawError),
}
//}}}
//{{{ struct: Return
/// Represents the QR decomposition of a matrix.
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
//{{{ impl DMatrix<T>
impl<T> DMatrix<T>
where
    T: One + Zero + Geqrf + Orgqr + Field + Copy + AsI32,
{
    /// Computes the QR decomposition of the matrix.
    ///
    /// Factors `self` into `Q` and `R` such that `A = QR`, where `Q` is orthogonal and `R` is
    /// upper-triangular.
    ///
    /// # Errors
    ///
    /// Returns [`Error::GetrfError`] if `geqrf` fails, or [`Error::OrgqrError`] if `orgqr` fails.
    pub fn qr(&self) -> Result<Return<T>, Error>
    {
        let n = self.nrows;
        let m = self.ncols;
        let raw = qr_raw(self.data.clone(), n, m)?;
        Ok(Return {
            q: DMatrix {
                data: raw.q_data,
                nrows: n,
                ncols: m,
            },
            r: DMatrix {
                data: raw.r_data,
                nrows: n,
                ncols: m,
            },
        })
    }
}
//}}}
