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
use crate::blaslapack::{qr_raw, LapackScalar, QrRawError};
use crate::dmatrix::DMatrix;
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
/// Represents the QR decomposition of a matrix.
#[derive(Clone, Debug)]
pub struct Return<T> {
    /// Orthogonal factor Q.
    pub q: DMatrix<T>,
    /// Upper-triangular factor R.
    pub r: DMatrix<T>,
}
//}}}
//{{{ impl DMatrix<T>
impl<T> DMatrix<T>
where
    T: LapackScalar,
{
    /// Computes the QR decomposition of the matrix.
    ///
    /// Factors `self` into `Q` and `R` such that `A = QR`, where `Q` is orthogonal and `R` is
    /// upper-triangular.
    ///
    /// # Errors
    ///
    /// Returns [`Error::GeqrfFailure`] or [`Error::OrgqrFailure`] if a validated backend call
    /// unexpectedly rejects an argument.
    ///
    /// # Panics
    ///
    /// Panics if either dimension exceeds the LAPACK integer range.
    pub fn qr(self) -> Result<Return<T>, Error> {
        let n = self.nrows;
        let m = self.ncols;
        let raw = qr_raw(self.data, n, m).map_err(|error| match error {
            QrRawError::Geqrf(error) => Error::GeqrfFailure { info: error.info() },
            QrRawError::Orgqr(error) => Error::OrgqrFailure { info: error.info() },
        })?;
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
