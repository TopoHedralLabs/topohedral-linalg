//! Linear system solver for [`SMatrix`] via LAPACK `dgesv`/`sgesv`.
//!
//! Provides the `solve` method on square [`SMatrix<T, N, N>`] instances, solving A X = B for X
//! given right-hand-side [`SMatrix<T, N, M>`] B. Dimension compatibility (the row count of B
//! must equal the column count of A) is enforced at compile time through const generics. The
//! implementation delegates to `Gesv` and returns the solution as an `SMatrix<T, N, M>`.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::{solve_raw, LapackScalar, SolveRawError};
use crate::smatrix::SMatrix;
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
//{{{ impl: SMatrix<T, N, N>
impl<T, const N: usize> SMatrix<T, N, N>
where
    T: LapackScalar,
{
    /// Solves the linear system `self * X = b`, returning the solution matrix X.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Singular`] when the coefficient matrix has a zero pivot, or
    /// [`Error::BackendFailure`] for an unexpected backend failure.
    /// # Panics
    ///
    /// Panics if `N` or `R` exceeds the LAPACK integer range.
    pub fn solve<const R: usize>(
        self,
        b: SMatrix<T, N, R>,
    ) -> Result<SMatrix<T, N, R>, Error> {
        let data = solve_raw(self.into_iter().collect(), b.into_iter().collect(), N, R).map_err(
            |error| match error {
                SolveRawError::LapackError(info) if info > 0 => Error::Singular {
                    pivot: info as usize,
                },
                SolveRawError::LapackError(info) => Error::BackendFailure { info },
            },
        )?;
        Ok(SMatrix::from_col_vec(data))
    }
}
//}}}
