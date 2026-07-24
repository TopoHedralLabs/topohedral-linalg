//! Linear system solver for [`DMatrix`] via LAPACK `dgesv`/`sgesv`.
//!
//! Provides the `solve` method on [`DMatrix<T>`], solving the linear system A X = B for X given
//! coefficient matrix A and right-hand-side matrix B. The system is solved using the `Gesv`
//! LAPACK driver, which performs LU factorisation with partial pivoting in place. The solution
//! matrix X is returned on success; failures produce a typed `Error`.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::{solve_raw, LapackScalar, SolveRawError};
use crate::dmatrix::DMatrix;
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: Error
/// Errors that can occur when solving a linear system.
#[derive(Clone, Error, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// A zero pivot made the coefficient matrix singular.
    #[error("coefficient matrix is singular at pivot {pivot}")]
    Singular {
        /// One-based index of the zero pivot.
        pivot: usize,
    },
    /// The validated backend call unexpectedly rejected an argument.
    #[error("linear solve backend failed with info code {info}")]
    BackendFailure {
        /// Raw LAPACK diagnostic code.
        info: i32,
    },
}
//}}}

//{{{ impl DMatrix<T>
impl<T> DMatrix<T>
where
    T: LapackScalar,
{
    /// Solves the linear system `A X = B` for `X`.
    ///
    /// Uses LAPACK `gesv`, which performs LU factorisation with partial pivoting on `self` in
    /// order to compute the solution matrix `X`.  `self` must be square and non-singular.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Singular`] when the coefficient matrix has a zero pivot, or
    /// [`Error::BackendFailure`] for an unexpected backend failure.
    /// # Panics
    ///
    /// Panics if `self` is not square, if `b` has a different row count, or if a dimension
    /// exceeds the LAPACK integer range.
    pub fn solve(
        self,
        b: DMatrix<T>,
    ) -> Result<DMatrix<T>, Error> {
        let n = self.nrows;
        assert_eq!(n, self.ncols, "coefficient matrix must be square");
        assert_eq!(
            b.nrows, n,
            "right-hand-side row count must match the coefficient matrix"
        );
        let nrhs = b.ncols;
        let data = solve_raw(self.data, b.data, n, nrhs).map_err(|error| match error {
            SolveRawError::LapackError(info) if info > 0 => Error::Singular {
                pivot: info as usize,
            },
            SolveRawError::LapackError(info) => Error::BackendFailure { info },
        })?;
        Ok(DMatrix {
            data,
            nrows: n,
            ncols: nrhs,
        })
    }
}
//}}}
