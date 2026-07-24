//! LU decomposition of a [`DMatrix`] via LAPACK `dgetrf`/`sgetrf`.
//!
//! Provides the `lu()` method on [`DMatrix<T>`], factoring the matrix into lower-triangular L,
//! upper-triangular U, and a row-permutation matrix P such that PA = LU. The factorisation is
//! computed by the `Getrf` LAPACK routine using partial pivoting. Results are returned as a
//! `Return<T>` struct containing the three factor matrices and the number of row swaps, which
//! determines the sign of the determinant. Factorisation failures are reported through a typed
//! `LUError`.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::{lu_raw, LapackScalar, LuRawError};
use crate::dmatrix::DMatrix;
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
pub struct Return<T> {
    /// Lower-triangular factor L with unit diagonal.
    pub l: DMatrix<T>,
    /// Upper-triangular factor U.
    pub u: DMatrix<T>,
    /// Row-permutation matrix P such that PA = LU.
    pub p: DMatrix<T>,
    /// Number of row interchanges performed; determines the sign of the determinant.
    pub num_swaps: usize,
}
//}}}
//{{{ impl DMatrix<T>
impl<T> DMatrix<T>
where
    T: LapackScalar,
{
    /// Computes the LU decomposition of the matrix.
    ///
    /// Factors `self` into `P`, `L`, and `U` such that `PA = LU`, where `P` is a permutation
    /// matrix, `L` is lower-triangular with unit diagonal, and `U` is upper-triangular.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Singular`] when a zero pivot is encountered, or
    /// [`Error::BackendFailure`] for an unexpected backend failure.
    /// # Panics
    ///
    /// Panics if either dimension exceeds the LAPACK integer range.
    pub fn lu(self) -> Result<Return<T>, Error> {
        let n = self.nrows;
        let m = self.ncols;
        let raw = lu_raw(self.data, n, m).map_err(|error| match error {
            LuRawError::LapackError(info) if info > 0 => Error::Singular {
                pivot: info as usize,
            },
            LuRawError::LapackError(info) => Error::BackendFailure { info },
        })?;
        Ok(Return {
            l: DMatrix {
                data: raw.l_data,
                nrows: n,
                ncols: m,
            },
            u: DMatrix {
                data: raw.u_data,
                nrows: n,
                ncols: m,
            },
            p: DMatrix {
                data: raw.p_data,
                nrows: n,
                ncols: n,
            },
            num_swaps: raw.num_swaps,
        })
    }
}
//}}}
