//! Linear system solver for [`SMatrix`] via LAPACK `dgesv`/`sgesv`.
//!
//! Provides the `solve` method on square [`SMatrix<T, N, N>`] instances, solving A X = B for X
//! given right-hand-side [`SMatrix<T, N, M>`] B. Dimension compatibility (the row count of B
//! must equal the column count of A) is enforced at compile time through const generics. The
//! implementation delegates to [`Gesv`] and returns the solution as an `SMatrix<T, N, M>`.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::blaslapack::gesv;
use crate::blaslapack::gesv::Gesv;
use crate::common::Field;
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

#[derive(Error, Debug)]
pub enum Error
{
    #[error("Error in solve(), exited with error:\n{0}")]
    GesvError(#[from] gesv::Error),
}

#[allow(private_bounds)]
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Gesv + Field,
{
    pub fn solve(
        &self,
        b: &SMatrix<T, N, M>,
    ) -> Result<SMatrix<T, N, M>, Error>
    {
        let mut a = *self;
        let mut x = *b;
        let mut ipiv = vec![0; N];
        T::gesv(
            N as i32,
            M as i32,
            &mut a.data,
            N as i32,
            &mut ipiv,
            &mut x.data,
            N as i32,
        )?;
        Ok(x)
    }
}
