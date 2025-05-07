//! Short Description of module
//!
//! Longer description of module
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
    #[error("Error in orgqr, exited with code {0}")]
    LapackError(i32),
}

/// Trait for LAPACK's symmetric eigenvalue computation routine
#[allow(clippy::too_many_arguments)]
pub trait Syev: Copy
{
    fn syev(
        jobz: u8, // 'N' for eigenvalues only, 'V' for eigenvalues and eigenvectors
        uplo: u8, // 'U' for upper triangle, 'L' for lower triangle
        n: i32,
        a: &mut [Self], // On exit, contains eigenvectors if jobz = 'V'
        lda: i32,
        w: &mut [Self], // Contains eigenvalues on exit
        work: &mut [Self],
        lwork: i32,
    ) -> Result<(), Error>;
}

// Implementation for f64
impl Syev for f64
{
    #[inline]
    fn syev(
        jobz: u8,
        uplo: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        w: &mut [Self],
        work: &mut [Self],
        lwork: i32,
    ) -> Result<(), Error>
    {
        let mut info = 0;
        unsafe {
            lapack::dsyev(jobz, uplo, n, a, lda, w, work, lwork, &mut info);
        }
        if info != 0
        {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}

// Implementation for f32
impl Syev for f32
{
    #[inline]
    fn syev(
        jobz: u8,
        uplo: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        w: &mut [Self],
        work: &mut [Self],
        lwork: i32,
    ) -> Result<(), Error>
    {
        let mut info = 0;
        unsafe {
            lapack::ssyev(jobz, uplo, n, a, lda, w, work, lwork, &mut info);
        }
        if info != 0
        {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
