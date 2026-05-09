//! LAPACK `dgeqrf`/`sgeqrf` wrapper for QR factorisation via Householder reflectors.
//!
//! Provides the [`Geqrf`] trait, wrapping the LAPACK `?geqrf` routine. On return, the upper
//! triangle of the input contains R and the elementary reflectors encoding Q are stored in the
//! lower triangle together with the `tau` array. A workspace query (lwork = −1) is supported to
//! obtain the optimal workspace size before the main computation. Implementations for `f64` and
//! `f32`.
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
/// Errors returned by the [`Geqrf`] LAPACK wrapper.
#[derive(Error, Debug)]
pub enum Error
{
    /// LAPACK returned a non-zero info code indicating an invalid argument.
    #[error("Error in geqrf, exited with code {0}")]
    LapackError(i32),
}
//}}}

//{{{ trait: Geqrf
/// Trait for types that support QR factorisation via Householder reflectors.
pub trait Geqrf: Copy
{
    /// Computes the QR factorisation of an M-by-N matrix A, storing the result in-place.
    fn geqrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        tau: &mut [Self],
        work: &mut [Self],
        lwork: i32,
    ) -> Result<(), Error>;
}
//}}}
//{{{ impl: Geqrf for f64
impl Geqrf for f64
{
    #[inline]
    fn geqrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        tau: &mut [Self],
        work: &mut [Self],
        lwork: i32,
    ) -> Result<(), Error>
    {
        let mut info = 0;
        unsafe {
            lapack::dgeqrf(m, n, a, lda, tau, work, lwork, &mut info);
        }
        if info != 0
        {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
//}}}
//{{{ impl: Geqrf for f32
impl Geqrf for f32
{
    #[inline]
    fn geqrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        tau: &mut [Self],
        work: &mut [Self],
        lwork: i32,
    ) -> Result<(), Error>
    {
        let mut info = 0;
        unsafe {
            lapack::sgeqrf(m, n, a, lda, tau, work, lwork, &mut info);
        }
        if info != 0
        {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
//}}}
