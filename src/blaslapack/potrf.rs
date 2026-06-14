//! LAPACK `dpotrf`/`spotrf` wrapper for Cholesky factorisation.
//!
//! Provides the [`Potrf`] trait, wrapping the LAPACK `?potrf` routine that factors a symmetric
//! positive-definite matrix A into either `A = L L^T` or `A = U^T U`. The shared raw helper used
//! by the public matrix APIs computes the lower-triangular factor and zeros the unused upper
//! triangle before returning it.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: Error
/// Errors returned by the [`Potrf`] LAPACK wrapper.
#[derive(Error, Debug)]
pub enum Error
{
    /// LAPACK returned a non-zero info code indicating an invalid argument or a non-positive pivot.
    #[error("Error in potrf, exited with code {0}")]
    LapackError(i32),
}
//}}}

//{{{ trait: Potrf
/// Trait for types that support Cholesky factorisation.
pub trait Potrf: Copy
{
    /// Computes the Cholesky factorisation of a symmetric positive-definite matrix.
    fn potrf(
        uplo: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
    ) -> Result<(), Error>;
}
//}}}
//{{{ impl: Potrf for f64
impl Potrf for f64
{
    #[inline]
    fn potrf(
        uplo: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
    ) -> Result<(), Error>
    {
        let mut info = 0;
        unsafe {
            lapack::dpotrf(uplo, n, a, lda, &mut info);
        }
        if info != 0
        {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
//}}}
//{{{ impl: Potrf for f32
impl Potrf for f32
{
    #[inline]
    fn potrf(
        uplo: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
    ) -> Result<(), Error>
    {
        let mut info = 0;
        unsafe {
            lapack::spotrf(uplo, n, a, lda, &mut info);
        }
        if info != 0
        {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
//}}}

//{{{ struct: CholeskyRaw
pub(crate) struct CholeskyRaw<T>
{
    pub l_data: Vec<T>,
}
//}}}
//{{{ fun: cholesky_raw
/// Shared POTRF algorithm. Consumes cloned matrix data and returns the lower-triangular factor.
pub(crate) fn cholesky_raw<T>(
    mut a_data: Vec<T>,
    n: usize,
) -> Result<CholeskyRaw<T>, Error>
where
    T: Potrf + crate::common::Zero + crate::common::Field + Copy,
{
    T::potrf(b'L', n as i32, &mut a_data, n as i32)?;

    for j in 0..n
    {
        for i in 0..j
        {
            a_data[i + j * n] = T::zero();
        }
    }

    Ok(CholeskyRaw { l_data: a_data })
}
//}}}
