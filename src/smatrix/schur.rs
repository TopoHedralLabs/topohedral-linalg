//! Schur decomposition of an [`SMatrix`] via LAPACK `dgees`/`sgees`.
//!
//! Provides the `schur()` method on [`SMatrix<T, N, M>`], computing A = Q T Q^H. The static
//! `Return<T, N, M>` struct carries Q and T as [`SMatrix`] instances with compile-time dimensions.
//! The implementation delegates to `Gees` and is the static counterpart of the [`DMatrix`](crate::dmatrix::DMatrix)
//! Schur decomposition.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::blaslapack::gees::{self, Gees};
use crate::common::{Field, One, Zero};
use crate::ops::schur::schur_raw;
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: Error
/// Errors that can occur during Schur decomposition.
#[derive(Error, Debug)]
pub enum Error
{
    /// Wraps a LAPACK `gees` error from the Schur factorisation routine.
    #[error("Error in schur(), exited with error:\n{0}")]
    GeesError(#[from] gees::Error),
}
//}}}
//{{{ struct: Return
/// Result of a Schur decomposition: orthogonal factor Q and quasi-upper-triangular Schur matrix T.
pub struct Return<T, const N: usize, const M: usize>
where
    [(); N * M]:,
    T: Field + Copy,
{
    /// Orthogonal (unitary) transformation matrix Q such that A = Q T Q^H.
    pub q: SMatrix<T, N, M>,
    /// Quasi-upper-triangular Schur matrix T.
    pub t: SMatrix<T, N, M>,
}
//}}}
//{{{ impl: SMatrix<T, N, M>
#[allow(private_bounds)]
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    [(); N * 5]:,
    [(); N * M]:,
    T: One + Zero + Gees + Field + Default + Copy,
{
    /// Computes the Schur decomposition A = Q T Q^H of the matrix.
    ///
    /// # Errors
    ///
    /// Returns an error if the LAPACK `gees` routine fails.
    pub fn schur(&self) -> Result<Return<T, N, M>, Error>
    {
        let raw = schur_raw(self.data.to_vec(), N, M)?;
        let q_arr: [T; N * M] = raw.q_data.try_into().unwrap_or_else(|_| unreachable!());
        let t_arr: [T; N * M] = raw.t_data.try_into().unwrap_or_else(|_| unreachable!());
        Ok(Return {
            q: SMatrix { data: q_arr, nrows: N, ncols: M },
            t: SMatrix { data: t_arr, nrows: N, ncols: M },
        })
    }
}
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{}
//}}}
