//! LAPACK `dsyev`/`ssyev` wrapper for symmetric (real) eigendecomposition.
//!
//! Provides the [`Syev`] trait, wrapping the LAPACK `?syev` routine for computing eigenvalues and
//! eigenvectors of a real symmetric matrix. The `jobz` parameter controls whether eigenvectors are
//! computed; `uplo` selects which triangle is used. Eigenvalues are written to a separate array in
//! ascending order; eigenvectors (if requested) overwrite the input. This is the LAPACK driver used
//! by both `DMatrix::symeig()` and `SMatrix::symeig()`.
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
/// Errors returned by the [`Syev`] LAPACK wrapper.
#[derive(Error, Debug)]
pub enum Error
{
    /// LAPACK returned a non-zero info code indicating the algorithm failed to converge.
    #[error("Error in orgqr, exited with code {0}")]
    LapackError(i32),
}
//}}}

//{{{ trait: Syev
/// Trait for LAPACK's symmetric eigenvalue computation routine
#[allow(clippy::too_many_arguments)]
pub trait Syev: Copy
{
    /// Computes eigenvalues and optionally eigenvectors of a real symmetric matrix.
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
//}}}

//{{{ impl: Syev for f64
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
//}}}

//{{{ impl: Syev for f32
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
//}}}
