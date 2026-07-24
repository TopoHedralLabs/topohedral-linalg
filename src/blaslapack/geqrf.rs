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
#[derive(Clone, Error, Debug, PartialEq, Eq)]
pub enum Error {
    /// LAPACK returned a non-zero info code indicating an invalid argument.
    #[error("geqrf failed with info code {0}")]
    LapackError(i32),
}

impl Error {
    pub(crate) fn info(&self) -> i32 {
        match self {
            Self::LapackError(info) => *info,
        }
    }
}
//}}}

//{{{ trait: Geqrf
/// Trait for types that support QR factorisation via Householder reflectors.
pub trait Geqrf: Copy {
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
impl Geqrf for f64 {
    #[inline]
    fn geqrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        tau: &mut [Self],
        work: &mut [Self],
        lwork: i32,
    ) -> Result<(), Error> {
        let mut info = 0;
        unsafe {
            lapack::dgeqrf(m, n, a, lda, tau, work, lwork, &mut info);
        }
        if info != 0 {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
//}}}
//{{{ impl: Geqrf for f32
impl Geqrf for f32 {
    #[inline]
    fn geqrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        tau: &mut [Self],
        work: &mut [Self],
        lwork: i32,
    ) -> Result<(), Error> {
        let mut info = 0;
        unsafe {
            lapack::sgeqrf(m, n, a, lda, tau, work, lwork, &mut info);
        }
        if info != 0 {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
//}}}

//{{{ struct: QrRaw
/// Raw buffers produced by the shared QR factorisation.
pub(crate) struct QrRaw<T> {
    /// Orthogonal factor in column-major order.
    pub q_data: Vec<T>,
    /// Upper-triangular factor in column-major order.
    pub r_data: Vec<T>,
}
//}}}
//{{{ enum: QrRawError
#[derive(Clone, Error, Debug, PartialEq, Eq)]
pub enum QrRawError {
    /// Householder factorisation failed.
    #[error(transparent)]
    Geqrf(#[from] Error),
    /// Explicit Q construction failed.
    #[error(transparent)]
    Orgqr(#[from] super::orgqr::Error),
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
    let n_i32 = super::common::blas_dim("matrix row count", n);
    let m_i32 = super::common::blas_dim("matrix column count", m);
    let k_i32 = super::common::blas_dim("QR reflector count", k);
    super::common::assert_matrix_len("matrix", a_data.len(), n, m);
    let mut tau = vec![T::zero(); k];

    let mut work = vec![T::zero(); 1];
    T::geqrf(n_i32, m_i32, &mut a_data, n_i32, &mut tau, &mut work, -1)?;

    let (workspace_len, lwork) = super::common::workspace_len(&work[0]);
    let mut work = vec![T::zero(); workspace_len];
    T::geqrf(n_i32, m_i32, &mut a_data, n_i32, &mut tau, &mut work, lwork)?;

    let mut r_data = vec![T::zero(); super::common::matrix_len("matrix", n, m)];
    for i in 0..n {
        for j in i..m {
            r_data[i + j * n] = a_data[i + j * n];
        }
    }

    T::orgqr(
        n_i32,
        k_i32,
        k_i32,
        &mut a_data,
        n_i32,
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
