//! QR decomposition of an [`SMatrix`] via LAPACK `dgeqrf`/`sgeqrf` and `dorgqr`/`sorgqr`.
//!
//! Provides the `qr()` method on [`SMatrix<T, N, M>`], computing A = QR with Q orthogonal and R
//! upper-triangular. The static `Return<T, N, M>` struct carries Q and R as [`SMatrix`] values
//! with appropriate compile-time dimensions. The implementation mirrors its [`DMatrix`](crate::dmatrix::DMatrix) counterpart
//! but operates entirely on stack-allocated storage.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::{qr_raw, LapackScalar, QrRawError};
use crate::smatrix::SMatrix;
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: Error
/// Errors that can occur during QR decomposition.
#[derive(Clone, Error, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The validated `geqrf` backend call unexpectedly rejected an argument.
    #[error("geqrf backend failed with info code {info}")]
    GeqrfFailure {
        /// Raw LAPACK diagnostic code.
        info: i32,
    },
    /// The validated `orgqr` backend call unexpectedly rejected an argument.
    #[error("orgqr backend failed with info code {info}")]
    OrgqrFailure {
        /// Raw LAPACK diagnostic code.
        info: i32,
    },
}
//}}}
//{{{ struct: Return
/// Result of a QR decomposition: orthogonal factor Q and upper-triangular factor R.
#[derive(Clone, Debug)]
pub struct Return<T, const N: usize, const M: usize> {
    /// Orthogonal factor Q.
    pub q: SMatrix<T, N, M>,
    /// Upper-triangular factor R.
    pub r: SMatrix<T, N, M>,
}
//}}}
//{{{ impl: SMatrix<T, N, M>
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    T: LapackScalar,
{
    /// Computes the QR decomposition of the matrix, returning Q (orthogonal) and R (upper-triangular).
    ///
    /// # Errors
    ///
    /// Returns [`Error::GeqrfFailure`] or [`Error::OrgqrFailure`] if a validated backend call
    /// unexpectedly rejects an argument.
    /// # Panics
    ///
    /// Panics if either dimension exceeds the LAPACK integer range.
    pub fn qr(self) -> Result<Return<T, N, M>, Error> {
        let raw = qr_raw(self.into_iter().collect(), N, M).map_err(|error| match error {
            QrRawError::Geqrf(error) => Error::GeqrfFailure { info: error.info() },
            QrRawError::Orgqr(error) => Error::OrgqrFailure { info: error.info() },
        })?;
        Ok(Return {
            q: SMatrix::from_col_vec(raw.q_data),
            r: SMatrix::from_col_vec(raw.r_data),
        })
    }
}
//}}}
