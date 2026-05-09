//! LAPACK `dorgqr`/`sorgqr` wrapper for explicit Q reconstruction from a QR factorisation.
//!
//! Provides the [`Orgqr`] trait, wrapping the LAPACK `?orgqr` routine that expands the compact
//! Householder representation produced by [`Geqrf`] into an explicit orthogonal matrix Q.
//! Parameters k (the number of reflectors), tau, and the workspace follow the LAPACK convention.
//! This is the second of the two LAPACK calls in the QR decomposition path.
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
/// Errors returned by the [`Orgqr`] LAPACK wrapper.
#[derive(Error, Debug)]
pub enum Error
{
    /// LAPACK returned a non-zero info code indicating an invalid argument.
    #[error("Error in orgqr, exited with code {0}")]
    LapackError(i32),
}
//}}}

//{{{ trait: Orqr
/// Trait for types that support explicit Q reconstruction from a QR factorisation.
#[allow(clippy::too_many_arguments)]
pub trait Orgqr: Copy
{
    /// Expands the compact Householder representation from [`Geqrf`] into an explicit orthogonal matrix Q.
    fn orgqr(
        m: i32,
        n: i32,
        k: i32,
        a: &mut [Self],
        lda: i32,
        tau: &[Self],
        work: &mut [Self],
        lwork: i32,
    ) -> Result<(), Error>;
}
//}}}
//{{{ impl: Orqr for f64
impl Orgqr for f64
{
    #[inline]
    fn orgqr(
        m: i32,
        n: i32,
        k: i32,
        a: &mut [Self],
        lda: i32,
        tau: &[Self],
        work: &mut [Self],
        lwork: i32,
    ) -> Result<(), Error>
    {
        let mut info = 0;
        unsafe {
            lapack::dorgqr(m, n, k, a, lda, tau, work, lwork, &mut info);
        }
        if info != 0
        {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
//}}}
//{{{ impl: Orqr for f32
impl Orgqr for f32
{
    #[inline]
    fn orgqr(
        m: i32,
        n: i32,
        k: i32,
        a: &mut [Self],
        lda: i32,
        tau: &[Self],
        work: &mut [Self],
        lwork: i32,
    ) -> Result<(), Error>
    {
        let mut info = 0;
        unsafe {
            lapack::sorgqr(m, n, k, a, lda, tau, work, lwork, &mut info);
        }
        if info != 0
        {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
//}}}
