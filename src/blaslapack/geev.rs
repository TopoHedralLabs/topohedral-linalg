//! LAPACK `dgeev`/`sgeev` wrapper for general (non-symmetric) eigendecomposition.
//!
//! Provides the [`Geev`] trait, wrapping the LAPACK `?geev` routine that computes all eigenvalues
//! and, optionally, the left and right eigenvectors of a general real matrix. Eigenvalues are
//! returned as separate real and imaginary part arrays to match the LAPACK interface; callers in
//! [`eig`] combine them into `Complex<T>` values. A workspace query is supported to obtain the
//! optimal workspace size before the main computation.
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
    #[error("Error in geev, exited with code {0}")]
    LapackError(i32),
}

#[allow(clippy::too_many_arguments)]
pub trait Geev: Copy
{
    fn geev(
        jobvl: u8,
        jobvr: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        wr: &mut [Self],
        wi: &mut [Self],
        vl: &mut [Self],
        ldvl: i32,
        vr: &mut [Self],
        ldvr: i32,
        work: &mut [Self],
        lwork: i32,
    ) -> Result<(), Error>;
}

impl Geev for f64
{
    #[inline]
    fn geev(
        jobvl: u8,
        jobvr: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        wr: &mut [Self],
        wi: &mut [Self],
        vl: &mut [Self],
        ldvl: i32,
        vr: &mut [Self],
        ldvr: i32,
        work: &mut [Self],
        lwork: i32,
    ) -> Result<(), Error>
    {
        let mut info = 0;
        unsafe {
            lapack::dgeev(
                jobvl, jobvr, n, a, lda, wr, wi, vl, ldvl, vr, ldvr, work, lwork, &mut info,
            );
        }

        if info != 0
        {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}

impl Geev for f32
{
    #[inline]
    fn geev(
        jobvl: u8,
        jobvr: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        wr: &mut [Self],
        wi: &mut [Self],
        vl: &mut [Self],
        ldvl: i32,
        vr: &mut [Self],
        ldvr: i32,
        work: &mut [Self],
        lwork: i32,
    ) -> Result<(), Error>
    {
        let mut info = 0;
        unsafe {
            lapack::sgeev(
                jobvl, jobvr, n, a, lda, wr, wi, vl, ldvl, vr, ldvr, work, lwork, &mut info,
            );
        }
        if info != 0
        {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
