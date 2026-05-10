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
use super::DMatrix;
use crate::blaslapack::getrf::{self, Getrf};
use crate::common::{Field, One, Zero};
use crate::ops::lu::lu_raw;
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: Error
/// Errors that can occur during LU decomposition.
#[derive(Error, Debug)]
pub enum Error
{
    #[error("Error in lu(), exited with error:\n{0}")]
    /// LAPACK `getrf` reported a failure, e.g. a zero pivot was encountered.
    GetrfError(#[from] getrf::Error),
}
//}}}
//{{{ struct: Return
/// Represents the LU decomposition of a matrix.
#[derive(Debug)]
pub struct Return<T>
where
    T: Field + Copy,
{
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
#[allow(private_bounds)]
impl<T> DMatrix<T>
where
    T: One + Zero + Getrf + Field + Copy,
{
    /// Computes the LU decomposition of the matrix.
    ///
    /// Factors `self` into `P`, `L`, and `U` such that `PA = LU`, where `P` is a permutation
    /// matrix, `L` is lower-triangular with unit diagonal, and `U` is upper-triangular.
    ///
    /// # Errors
    ///
    /// Returns [`Error::GetrfError`] if the LAPACK `getrf` routine fails.
    pub fn lu(&self) -> Result<Return<T>, Error>
    {
        let n = self.nrows;
        let m = self.ncols;
        let raw = lu_raw(self.data.clone(), n, m)?;
        Ok(Return {
            l:         DMatrix { data: raw.l_data, nrows: n, ncols: m },
            u:         DMatrix { data: raw.u_data, nrows: n, ncols: m },
            p:         DMatrix { data: raw.p_data, nrows: n, ncols: m },
            num_swaps: raw.num_swaps,
        })
    }
}
//}}}
