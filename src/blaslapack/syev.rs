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

//{{{ struct: SymEigRaw
pub(crate) struct SymEigRaw<T>
{
    pub eigvecs_data: Vec<T>,
    pub eigvals:      Vec<T>,
}
//}}}
//{{{ fun: symeig_raw
/// Shared SYEV algorithm. Consumes the cloned matrix data; returns raw eigenvec/eigenval buffers.
pub(crate) fn symeig_raw<T>(
    mut a_data: Vec<T>,
    n: usize,
) -> Result<SymEigRaw<T>, Error>
where
    T: Syev
        + crate::common::One
        + crate::common::Zero
        + crate::common::Field
        + Default
        + Copy
        + super::common::AsI32,
{
    let mut eigvals = vec![T::zero(); n];

    let mut work = vec![T::zero(); 1];
    T::syev(b'V', b'L', n as i32, &mut a_data, n as i32, &mut eigvals, &mut work, -1)?;

    let lwork = work[0].as_i32();
    let mut work = vec![T::zero(); lwork as usize];
    T::syev(b'V', b'L', n as i32, &mut a_data, n as i32, &mut eigvals, &mut work, lwork)?;

    Ok(SymEigRaw { eigvecs_data: a_data, eigvals })
}
//}}}
