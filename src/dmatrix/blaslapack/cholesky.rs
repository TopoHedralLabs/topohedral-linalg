//! Cholesky decomposition of a [`DMatrix`] via LAPACK `dpotrf`/`spotrf`.
//!
//! Provides the `cholesky()` method on square symmetric positive-definite [`DMatrix<T>`]
//! instances, computing the lower-triangular factor `L` such that `A = L L^T`. The factorisation
//! is delegated to LAPACK `Potrf`; failures, including non-positive-definite input, are reported
//! through a typed `Error`.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::{cholesky_raw, CholeskyRawError, Potrf};
use crate::common::{Field, Zero};
use crate::dmatrix::DMatrix;
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
    #[error("Error in cholesky(), exited with error:\n{0}")]
    /// LAPACK `potrf` reported a failure, e.g. the matrix is not positive definite.
    PotrfError(#[from] CholeskyRawError),
}
//}}}
//{{{ struct: Return
/// Represents the Cholesky decomposition of a symmetric positive-definite matrix.
#[derive(Debug)]
pub struct Return<T>
where
    T: Field + Copy,
{
    /// Lower-triangular factor L such that A = L L^T.
    pub l: DMatrix<T>,
}
//}}}
//{{{ impl DMatrix<T>
#[allow(private_bounds)]
impl<T> DMatrix<T>
where
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
    pub fn cholesky(&self) -> Result<Return<T>, Error>
    {
        let n = self.nrows;
        if n != self.ncols
        {
            panic!("Matrix must be square for Cholesky decomposition");
        }
        let raw = cholesky_raw(self.data.clone(), n)?;
        Ok(Return {
            l: DMatrix {
                data: raw.l_data,
                nrows: n,
                ncols: n,
            },
        })
    }
}
//}}}
