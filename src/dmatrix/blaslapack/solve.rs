//! Linear system solver for [`DMatrix`] via LAPACK `dgesv`/`sgesv`.
//!
//! Provides the `solve` method on [`DMatrix<T>`], solving the linear system A X = B for X given
//! coefficient matrix A and right-hand-side matrix B. The system is solved using the `Gesv`
//! LAPACK driver, which performs LU factorisation with partial pivoting in place. The solution
//! matrix X is returned on success; failures produce a typed `Error`.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::dmatrix::DMatrix;
use crate::blaslapack::{solve_raw, SolveRawError, Gesv};
use crate::common::Field;
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: Error
/// Errors that can occur when solving a linear system.
#[derive(Error, Debug)]
pub enum Error
{
    #[error("Error in solve(), exited with error:\n{0}")]
    /// LAPACK `gesv` failed, e.g. because the coefficient matrix is singular.
    GesvError(#[from] SolveRawError),
}
//}}}

//{{{ impl DMatrix<T>
#[allow(private_bounds)]
impl<T> DMatrix<T>
where
    T: Gesv + Field,
{
    /// Solves the linear system `A X = B` for `X`.
    ///
    /// Uses LAPACK `gesv`, which performs LU factorisation with partial pivoting on `self` in
    /// order to compute the solution matrix `X`.  `self` must be square and non-singular.
    ///
    /// # Errors
    ///
    /// Returns [`Error::GesvError`] if the LAPACK `gesv` routine fails.
    pub fn solve(
        &self,
        b: &DMatrix<T>,
    ) -> Result<DMatrix<T>, Error>
    {
        let n = self.nrows;
        let nrhs = b.ncols;
        let data = solve_raw(self.data.clone(), b.data.clone(), n, nrhs)?;
        Ok(DMatrix {
            data,
            nrows: n,
            ncols: nrhs,
        })
    }
}
//}}}
