//! LU decomposition of an [`SMatrix`] via LAPACK `dgetrf`/`sgetrf`.
//!
//! Provides the `lu()` method on [`SMatrix<T, N, M>`], factoring the matrix into lower-triangular
//! L, upper-triangular U, and a row-permutation matrix P (as a static matrix) such that PA = LU.
//! The result is a const-generic `Return<T, N, M>` struct so that downstream code retains full
//! compile-time shape information for all three factor matrices.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::blaslapack::getrf::lu_raw;
use crate::blaslapack::getrf::{self, Getrf};
use crate::common::{Field, One, Zero};
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
    /// Wraps a LAPACK `getrf` error returned by the underlying factorisation routine.
    #[error("Error in lu(), exited with error:\n{0}")]
    GetrfError(#[from] getrf::Error),
}
//}}}
//{{{ struct: Return
/// Represents the LU decomposition of a matrix.
pub struct Return<T, const N: usize, const M: usize>
where
    [(); N * M]:,
    T: Field + Copy,
{
    /// Lower-triangular factor L with unit diagonal.
    pub l: SMatrix<T, N, M>,
    /// Upper-triangular factor U.
    pub u: SMatrix<T, N, M>,
    /// Row-permutation matrix P such that P A = L U.
    pub p: SMatrix<T, N, M>,
    /// Number of row swaps applied during pivoting.
    pub num_swaps: usize,
}
//}}}
//{{{ impl SMatrix<T, N, M>
#[allow(private_bounds)]
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    [(); N * M]:,
    T: One + Zero + Getrf + Field + Copy,
{
    /// Computes the LU decomposition of the matrix with partial pivoting.
    ///
    /// Returns `(L, U, P, num_swaps)` such that `P * self = L * U`.
    ///
    /// # Errors
    ///
    /// Returns an error if the LAPACK `getrf` routine fails.
    pub fn lu(&self) -> Result<Return<T, N, M>, Error>
    {
        let raw = lu_raw(self.data.to_vec(), N, M)?;
        let l_arr: [T; N * M] = raw.l_data.try_into().unwrap_or_else(|_| unreachable!());
        let u_arr: [T; N * M] = raw.u_data.try_into().unwrap_or_else(|_| unreachable!());
        let p_arr: [T; N * M] = raw.p_data.try_into().unwrap_or_else(|_| unreachable!());
        Ok(Return {
            l: SMatrix {
                data: l_arr,
                nrows: N,
                ncols: M,
            },
            u: SMatrix {
                data: u_arr,
                nrows: N,
                ncols: M,
            },
            p: SMatrix {
                data: p_arr,
                nrows: N,
                ncols: M,
            },
            num_swaps: raw.num_swaps,
        })
    }
}
//}}}
