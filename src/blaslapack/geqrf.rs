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

//{{{ struct: QrRaw
pub(crate) struct QrRaw<T>
{
    pub q_data: Vec<T>,
    pub r_data: Vec<T>,
}
//}}}
//{{{ enum: QrRawError
#[derive(Debug)]
pub(crate) enum QrRawError
{
    Geqrf(Error),
    Orgqr(super::orgqr::Error),
}

impl From<Error> for QrRawError
{
    fn from(e: Error) -> Self
    {
        QrRawError::Geqrf(e)
    }
}

impl From<super::orgqr::Error> for QrRawError
{
    fn from(e: super::orgqr::Error) -> Self
    {
        QrRawError::Orgqr(e)
    }
}
//}}}
//{{{ fun: qr_raw
/// Shared GEQRF + ORGQR algorithm. Consumes the cloned matrix data; returns raw Q/R buffers.
pub(crate) fn qr_raw<T>(
    mut a_data: Vec<T>,
    n: usize,
    m: usize,
) -> Result<QrRaw<T>, QrRawError>
where
    T: Geqrf
        + super::orgqr::Orgqr
        + crate::common::One
        + crate::common::Zero
        + crate::common::Field
        + Copy
        + super::common::AsI32,
{
    let k = n.min(m);
    let mut tau = vec![T::zero(); k];

    let mut work = vec![T::zero(); 1];
    T::geqrf(
        n as i32,
        m as i32,
        &mut a_data,
        n as i32,
        &mut tau,
        &mut work,
        -1,
    )?;

    let lwork = work[0].as_i32();
    let mut work = vec![T::zero(); lwork as usize];
    T::geqrf(
        n as i32,
        m as i32,
        &mut a_data,
        n as i32,
        &mut tau,
        &mut work,
        lwork,
    )?;

    let mut r_data = vec![T::zero(); n * m];
    for i in 0..n
    {
        for j in i..m
        {
            r_data[i + j * n] = a_data[i + j * n];
        }
    }

    T::orgqr(
        n as i32,
        n.min(m) as i32,
        k as i32,
        &mut a_data,
        n as i32,
        &tau,
        &mut work,
        lwork,
    )?;
    Ok(QrRaw {
        q_data: a_data,
        r_data,
    })
}
//}}}
