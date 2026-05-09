//! LAPACK `dgetrf`/`sgetrf` wrapper for LU factorisation with partial pivoting.
//!
//! Provides the [`Getrf`] trait, wrapping the LAPACK `?getrf` routine that factors a general
//! m×n matrix A into P·L·U using partial pivoting with row interchanges. The pivot array is
//! returned alongside the factored storage; a typed error is produced when the matrix is singular.
//! Implementations for `f64` and `f32` call the Fortran LAPACK ABI via the `lapack` crate.
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
    #[error("Error in getrf, exited with code {0}")]
    LapackError(i32),
}

//{{{ trait: Getrf
/// Trait for types that support LU factorization.
pub trait Getrf: Copy
{
    /// Performs LU factorization of a general M-by-N matrix A using partial pivoting
    /// with row interchanges.
    ///
    /// The factorization has the form:
    /// A = P * L * U
    ///
    /// where P is a permutation matrix, L is lower triangular with unit diagonal
    /// elements, and U is upper triangular.
    fn getrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        ipiv: &mut [i32],
    ) -> Result<(), Error>;
}
//}}}
//{{{ impl: Getrf for f64
impl Getrf for f64
{
    #[inline]
    fn getrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        ipiv: &mut [i32],
    ) -> Result<(), Error>
    {
        let mut info = 0;
        unsafe {
            lapack::dgetrf(m, n, a, lda, ipiv, &mut info);
        }
        if info != 0
        {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
//}}}
//{{{ impl: Getrf for f32
impl Getrf for f32
{
    #[inline]
    fn getrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        ipiv: &mut [i32],
    ) -> Result<(), Error>
    {
        let mut info = 0;
        unsafe {
            lapack::sgetrf(m, n, a, lda, ipiv, &mut info);
        }
        if info != 0
        {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
//}}}
