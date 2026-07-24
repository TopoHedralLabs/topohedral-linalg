//! LU decomposition of an [`SMatrix`] via LAPACK `dgetrf`/`sgetrf`.
//!
//! Provides the `lu()` method on [`SMatrix<T, N, M>`], factoring the matrix into lower-triangular
//! L, upper-triangular U, and a row-permutation matrix P (as a static matrix) such that PA = LU.
//! The result is a const-generic `Return<T, N, M>` struct so that downstream code retains full
//! compile-time shape information for all three factor matrices.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::{lu_raw, LapackScalar, LuRawError};
use crate::smatrix::SMatrix;
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: Error
/// Errors that can occur during LU decomposition.
#[derive(Clone, Error, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// A zero pivot made the matrix singular.
    #[error("matrix is singular at pivot {pivot}")]
    Singular {
        /// One-based index of the zero pivot.
        pivot: usize,
    },
    /// The validated backend call unexpectedly rejected an argument.
    #[error("LU backend failed with info code {info}")]
    BackendFailure {
        /// Raw LAPACK diagnostic code.
        info: i32,
    },
}
//}}}
//{{{ struct: Return
/// Represents the LU decomposition of a matrix.
#[derive(Clone, Debug)]
pub struct Return<T, const N: usize, const M: usize> {
    /// Lower-triangular factor L with unit diagonal.
    pub l: SMatrix<T, N, M>,
    /// Upper-triangular factor U.
    pub u: SMatrix<T, N, M>,
    /// Row-permutation matrix P such that P A = L U.
    pub p: SMatrix<T, N, N>,
    /// Number of row swaps applied during pivoting.
    pub num_swaps: usize,
}
//}}}
//{{{ impl SMatrix<T, N, M>
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    T: LapackScalar,
{
    /// Computes the LU decomposition of the matrix with partial pivoting.
    ///
    /// Returns `(L, U, P, num_swaps)` such that `P * self = L * U`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Singular`] when a zero pivot is encountered, or
    /// [`Error::BackendFailure`] for an unexpected backend failure.
    /// # Panics
    ///
    /// Panics if either dimension exceeds the LAPACK integer range.
    pub fn lu(self) -> Result<Return<T, N, M>, Error> {
        let raw = lu_raw(self.into_iter().collect(), N, M).map_err(|error| match error {
            LuRawError::LapackError(info) if info > 0 => Error::Singular {
                pivot: info as usize,
            },
            LuRawError::LapackError(info) => Error::BackendFailure { info },
        })?;
        Ok(Return {
            l: SMatrix::from_col_vec(raw.l_data),
            u: SMatrix::from_col_vec(raw.u_data),
            p: SMatrix::from_col_vec(raw.p_data),
            num_swaps: raw.num_swaps,
        })
    }
}
//}}}
