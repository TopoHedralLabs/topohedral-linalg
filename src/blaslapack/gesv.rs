//! LAPACK `dgesv`/`sgesv` wrapper for solving general linear systems.
//!
//! Provides the [`Gesv`] trait, wrapping the LAPACK `?gesv` routine that solves A X = B for X by
//! performing LU factorisation with partial pivoting in place. The coefficient matrix A and
//! right-hand-side B are overwritten on return; the solution X occupies the space originally held
//! by B. A typed error is returned when the matrix is singular. This is the LAPACK driver used by
//! both `DMatrix::solve()` and `SMatrix::solve()`.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
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
    #[error("Error in gesv, exited with code {0}")]
    LapackError(i32),
}

pub trait Gesv: Copy
{
    fn gesv(
        n: i32,
        nrhs: i32,
        a: &mut [Self],
        lda: i32,
        ipiv: &mut [i32],
        b: &mut [Self],
        ldb: i32,
    ) -> Result<(), Error>;
}

impl Gesv for f64
{
    #[inline]
    fn gesv(
        n: i32,
        nrhs: i32,
        a: &mut [Self],
        lda: i32,
        ipiv: &mut [i32],
        b: &mut [Self],
        ldb: i32,
    ) -> Result<(), Error>
    {
        let mut info = 0;
        unsafe {
            lapack::dgesv(n, nrhs, a, lda, ipiv, b, ldb, &mut info);
        }
        if info != 0
        {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}

impl Gesv for f32
{
    #[inline]
    fn gesv(
        n: i32,
        nrhs: i32,
        a: &mut [Self],
        lda: i32,
        ipiv: &mut [i32],
        b: &mut [Self],
        ldb: i32,
    ) -> Result<(), Error>
    {
        let mut info = 0;
        unsafe {
            lapack::sgesv(n, nrhs, a, lda, ipiv, b, ldb, &mut info);
        }
        if info != 0
        {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
