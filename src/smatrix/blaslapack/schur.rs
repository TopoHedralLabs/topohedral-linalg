//! Schur decomposition of an [`SMatrix`] via LAPACK `dgees`/`sgees`.
//!
//! Provides the `schur()` method on [`SMatrix<T, N, M>`], computing A = Q T Q^H. The static
//! `Return<T, N, M>` struct carries Q and T as [`SMatrix`] instances with compile-time dimensions.
//! The implementation delegates to `Gees` and is the static counterpart of the [`DMatrix`](crate::dmatrix::DMatrix)
//! Schur decomposition.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::{schur_raw, Gees, ShurRawError};
use crate::common::{Field, One, Zero};
use crate::smatrix::SMatrix;
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
    GeesError(#[from] ShurRawError),
}
//}}}
//{{{ struct: Return
/// Result of a Schur decomposition: orthogonal factor Q and quasi-upper-triangular Schur matrix T.
pub struct Return<T, const N: usize, const M: usize>
where
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
    T: One + Zero + Gees + Field + Default + Copy,
{
    /// Computes the Schur decomposition A = Q T Q^H of the matrix.
    ///
    /// # Errors
    ///
    /// Returns an error if the LAPACK `gees` routine fails.
    pub fn schur(&self) -> Result<Return<T, N, M>, Error>
    {
        let raw = schur_raw(self.as_slice().to_vec(), N, M)?;
        Ok(Return {
            q: SMatrix::from_col_vec(raw.q_data),
            t: SMatrix::from_col_vec(raw.t_data),
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
