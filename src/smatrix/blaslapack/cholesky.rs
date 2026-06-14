//! Cholesky decomposition of an [`SMatrix`] via LAPACK `dpotrf`/`spotrf`.
//!
//! Provides the `cholesky()` method on square symmetric positive-definite [`SMatrix<T, N, N>`]
//! instances. Returns a const-generic `Return<T, N>` containing the lower-triangular factor `L`
//! such that `A = L L^T`.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::{cholesky_raw, CholeskyRawError, Potrf};
use crate::common::{Field, Zero};
use crate::smatrix::SMatrix;
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: Error
/// Errors that can occur during Cholesky decomposition.
#[derive(Error, Debug)]
pub enum Error
{
    /// Wraps a LAPACK `potrf` error from the Cholesky factorisation routine.
    #[error("Error in cholesky(), exited with error:\n{0}")]
    PotrfError(#[from] CholeskyRawError),
}
//}}}
//{{{ struct: Return
/// Represents the Cholesky decomposition of a symmetric positive-definite matrix.
#[derive(Debug)]
pub struct Return<T, const N: usize>
where
    [(); N * N]:,
    T: Field + Copy,
{
    /// Lower-triangular factor L such that A = L L^T.
    pub l: SMatrix<T, N, N>,
}
//}}}
//{{{ impl: SMatrix<T, N, N>
#[allow(private_bounds)]
impl<T, const N: usize> SMatrix<T, N, N>
where
    [(); N * N]:,
    T: Zero + Potrf + Field + Copy,
{
    /// Computes the Cholesky decomposition of the matrix.
    ///
    /// Factors `self` into `L` such that `self = L L^T`. Only the lower triangle of `self` is
    /// read by LAPACK.
    ///
    /// # Errors
    ///
    /// Returns [`Error::PotrfError`] if LAPACK `potrf` fails, including when the matrix is not
    /// positive definite.
    pub fn cholesky(&self) -> Result<Return<T, N>, Error>
    {
        let raw = cholesky_raw(self.data.to_vec(), N)?;
        let l_arr: [T; N * N] = raw.l_data.try_into().unwrap_or_else(|_| unreachable!());
        Ok(Return {
            l: SMatrix {
                data: l_arr,
                nrows: N,
                ncols: N,
            },
        })
    }
}
//}}}
