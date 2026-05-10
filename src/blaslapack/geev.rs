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

//{{{ enum: Error
/// Errors returned by the [`Geev`] LAPACK wrapper.
#[derive(Error, Debug)]
pub enum Error
{
    /// LAPACK returned a non-zero info code indicating a failure in the QR algorithm.
    #[error("Error in geev, exited with code {0}")]
    LapackError(i32),
}
//}}}

//{{{ trait: Geev
/// Trait for types that support general (non-symmetric) eigendecomposition.
#[allow(clippy::too_many_arguments)]
pub trait Geev: Copy
{
    /// Computes all eigenvalues and optionally left/right eigenvectors of a general real matrix.
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
//}}}

//{{{ impl: Geev for f64
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
//}}}

//{{{ impl: Geev for f32
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
//}}}

//{{{ struct: EigRaw
pub(crate) struct EigRaw<T>
{
    pub vl:      Vec<T>,
    pub vr:      Vec<T>,
    pub eigvals: Vec<crate::common::Complex<T>>,
}
//}}}
//{{{ fun: eig_raw
/// Shared GEEV algorithm. Consumes the cloned matrix data; returns raw eigenvector/eigenvalue buffers.
pub(crate) fn eig_raw<T>(
    mut a_data: Vec<T>,
    n: usize,
) -> Result<EigRaw<T>, Error>
where
    T: Geev
        + crate::common::One
        + crate::common::Zero
        + crate::common::Field
        + Default
        + Copy
        + super::common::AsI32,
{
    let mut vl = vec![T::zero(); n * n];
    let mut vr = vec![T::zero(); n * n];
    let mut wr = vec![T::zero(); n];
    let mut wi = vec![T::zero(); n];

    let mut work = vec![T::zero(); 1];
    T::geev(b'V', b'V', n as i32, &mut a_data, n as i32, &mut wr, &mut wi, &mut vl, n as i32, &mut vr, n as i32, &mut work, -1)?;

    let lwork = work[0].as_i32();
    let mut work = vec![T::zero(); lwork as usize];
    T::geev(b'V', b'V', n as i32, &mut a_data, n as i32, &mut wr, &mut wi, &mut vl, n as i32, &mut vr, n as i32, &mut work, lwork)?;

    let eigvals: Vec<crate::common::Complex<T>> = wr.iter().zip(wi.iter()).take(n).map(|(&r, &i)| crate::common::Complex::new(r, i)).collect();
    Ok(EigRaw { vl, vr, eigvals })
}
//}}}
