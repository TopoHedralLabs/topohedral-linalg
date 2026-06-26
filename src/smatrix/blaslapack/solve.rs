//! Linear system solver for [`SMatrix`] via LAPACK `dgesv`/`sgesv`.
//!
//! Provides the `solve` method on square [`SMatrix<T, N, N>`] instances, solving A X = B for X
//! given right-hand-side [`SMatrix<T, N, M>`] B. Dimension compatibility (the row count of B
//! must equal the column count of A) is enforced at compile time through const generics. The
//! implementation delegates to `Gesv` and returns the solution as an `SMatrix<T, N, M>`.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::{solve_raw, Gesv, SolveRawError};
use crate::common::Field;
use crate::smatrix::SMatrix;
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: Error
/// Errors that can occur when solving a linear system.
#[derive(Error, Debug)]
pub enum Error {
    /// Wraps a LAPACK `gesv` error from the linear solve routine.
    #[error("Error in solve(), exited with error:\n{0}")]
    GesvError(#[from] SolveRawError),
}
//}}}
//{{{ impl: SMatrix<T, N, M>
#[allow(private_bounds)]
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    T: Gesv + Field,
{
    /// Solves the linear system `self * X = b`, returning the solution matrix X.
    ///
    /// # Errors
    ///
    /// Returns an error if the LAPACK `gesv` routine fails (e.g., singular matrix).
    pub fn solve(
        &self,
        b: &SMatrix<T, N, M>,
    ) -> Result<SMatrix<T, N, M>, Error> {
        let data = solve_raw(self.as_slice().to_vec(), b.as_slice().to_vec(), N, M)?;
        Ok(SMatrix::from_col_vec(data))
    }
}
//}}}
