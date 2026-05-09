//! Linear system solver for [`DMatrix`] via LAPACK `dgesv`/`sgesv`.
//!
//! Provides the `solve` method on [`DMatrix<T>`], solving the linear system A X = B for X given
//! coefficient matrix A and right-hand-side matrix B. The system is solved using the [`Gesv`]
//! LAPACK driver, which performs LU factorisation with partial pivoting in place. The solution
//! matrix X is returned on success; failures produce a typed `Error`.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
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
impl<T> DMatrix<T>
where
    T: Gesv + Field,
{
    pub fn solve(
        &self,
        b: &DMatrix<T>,
    ) -> Result<DMatrix<T>, Error>
    {
        let n = self.nrows;
        let m = self.ncols;
        let mut a = self.clone();
        let mut x = b.clone();
        let mut ipiv = vec![0; n];
        T::gesv(
            n as i32,
            m as i32,
            &mut a.data,
            n as i32,
            &mut ipiv,
            &mut x.data,
            m as i32,
        )?;
        Ok(x)
    }
}
