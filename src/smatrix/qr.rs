//! QR decomposition of an [`SMatrix`] via LAPACK `dgeqrf`/`sgeqrf` and `dorgqr`/`sorgqr`.
//!
//! Provides the `qr()` method on [`SMatrix<T, N, M>`], computing A = QR with Q orthogonal and R
//! upper-triangular. The static `Return<T, N, M>` struct carries Q and R as [`SMatrix`] values
//! with appropriate compile-time dimensions. The implementation mirrors its [`DMatrix`](crate::dmatrix::DMatrix) counterpart
//! but operates entirely on stack-allocated storage.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::common::AsI32;
use crate::blaslapack::geqrf::{self, qr_raw, Geqrf, QrRawError};
use crate::blaslapack::orgqr::{self, Orgqr};
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
pub enum Error
{
    /// Wraps a LAPACK `geqrf` error from the Householder factorisation step.
    #[error("Error in qr(), exited with error:\n{0}")]
    GetrfError(#[from] geqrf::Error),
    /// Wraps a LAPACK `orgqr` error from the Q matrix generation step.
    #[error("Error in qr(), exited with error:\n{0}")]
    OrgqrError(#[from] orgqr::Error),
}

impl From<QrRawError> for Error
{
    fn from(e: QrRawError) -> Self
    {
        match e
        {
            QrRawError::Geqrf(e) => Error::GetrfError(e),
            QrRawError::Orgqr(e) => Error::OrgqrError(e),
        }
    }
}
//}}}
//{{{ struct: Return
/// Result of a QR decomposition: orthogonal factor Q and upper-triangular factor R.
pub struct Return<T, const N: usize, const M: usize>
where
    [(); N * M]:,
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
    [(); N * M]:,
    T: One + Zero + Geqrf + Orgqr + Field + Copy + AsI32,
{
    /// Computes the QR decomposition of the matrix, returning Q (orthogonal) and R (upper-triangular).
    ///
    /// # Errors
    ///
    /// Returns an error if either the LAPACK `geqrf` or `orgqr` routine fails.
    pub fn qr(&self) -> Result<Return<T, N, M>, Error>
    {
        let raw = qr_raw(self.data.to_vec(), N, M)?;
        let q_arr: [T; N * M] = raw.q_data.try_into().unwrap_or_else(|_| unreachable!());
        let r_arr: [T; N * M] = raw.r_data.try_into().unwrap_or_else(|_| unreachable!());
        Ok(Return {
            q: SMatrix {
                data: q_arr,
                nrows: N,
                ncols: M,
            },
            r: SMatrix {
                data: r_arr,
                nrows: N,
                ncols: M,
            },
        })
    }
}
//}}}
