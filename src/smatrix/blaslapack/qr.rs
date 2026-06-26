//! QR decomposition of an [`SMatrix`] via LAPACK `dgeqrf`/`sgeqrf` and `dorgqr`/`sorgqr`.
//!
//! Provides the `qr()` method on [`SMatrix<T, N, M>`], computing A = QR with Q orthogonal and R
//! upper-triangular. The static `Return<T, N, M>` struct carries Q and R as [`SMatrix`] values
//! with appropriate compile-time dimensions. The implementation mirrors its [`DMatrix`](crate::dmatrix::DMatrix) counterpart
//! but operates entirely on stack-allocated storage.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::AsI32;
use crate::blaslapack::{qr_raw, Geqrf, Orgqr, QrRawError};
use crate::common::{Field, One, Zero};
use crate::smatrix::SMatrix;
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: Error
/// Errors that can occur during QR decomposition.
#[derive(Error, Debug)]
pub enum Error {
    /// Wraps a LAPACK `geqrf`/`orgqr` error from the QR factorisation steps.
    #[error("Error in qr(), exited with error:\n{0}")]
    GetrfError(#[from] QrRawError),
}
//}}}
//{{{ struct: Return
/// Result of a QR decomposition: orthogonal factor Q and upper-triangular factor R.
pub struct Return<T, const N: usize, const M: usize>
where
    T: Field + Copy,
{
    /// Orthogonal factor Q.
    pub q: SMatrix<T, N, M>,
    /// Upper-triangular factor R.
    pub r: SMatrix<T, N, M>,
}
//}}}
//{{{ impl: SMatrix<T, N, M>
#[allow(private_bounds)]
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    T: One + Zero + Geqrf + Orgqr + Field + Copy + AsI32,
{
    /// Computes the QR decomposition of the matrix, returning Q (orthogonal) and R (upper-triangular).
    ///
    /// # Errors
    ///
    /// Returns an error if either the LAPACK `geqrf` or `orgqr` routine fails.
    pub fn qr(&self) -> Result<Return<T, N, M>, Error> {
        let raw = qr_raw(self.as_slice().to_vec(), N, M)?;
        Ok(Return {
            q: SMatrix::from_col_vec(raw.q_data),
            r: SMatrix::from_col_vec(raw.r_data),
        })
    }
}
//}}}
