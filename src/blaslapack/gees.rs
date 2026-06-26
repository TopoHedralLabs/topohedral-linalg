//! LAPACK `dgees`/`sgees` wrapper for the Schur decomposition.
//!
//! Provides the [`Gees`] trait, wrapping the LAPACK `?gees` routine that computes the Schur form
//! of a real general matrix, optionally reordering the Schur form so that selected eigenvalues
//! appear first. The `job` flag controls whether Schur vectors (Q) are computed; `sort` enables
//! eigenvalue reordering. This is the LAPACK driver used by both `DMatrix::schur()` and
//! `SMatrix::schur()`.
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
/// Errors returned by the [`Gees`] LAPACK wrapper.
#[derive(Error, Debug)]
pub enum Error {
    /// LAPACK returned a non-zero info code indicating the Schur decomposition failed.
    #[error("Error in gees, exited with code {0}")]
    LapackError(i32),
}
//}}}

//{{{ trait: Gees
/// Trait for types that support the Schur decomposition of a real general matrix.
#[allow(clippy::too_many_arguments)]
pub trait Gees: Copy {
    /// Computes the Schur form of a real general matrix, optionally computing Schur vectors.
    fn gees(
        jobvs: u8,
        sort: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        sdim: &mut i32,
        wr: &mut [Self],
        wi: &mut [Self],
        vs: &mut [Self],
        ldvs: i32,
        work: &mut [Self],
        lwork: i32,
        bwork: &mut [i32],
    ) -> Result<(), Error>;
}
//}}}

//{{{ impl: Gees for f64
impl Gees for f64 {
    #[inline]
    fn gees(
        jobvs: u8,
        sort: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        sdim: &mut i32,
        wr: &mut [Self],
        wi: &mut [Self],
        vs: &mut [Self],
        ldvs: i32,
        work: &mut [Self],
        lwork: i32,
        bwork: &mut [i32],
    ) -> Result<(), Error> {
        let mut info = 0;
        unsafe {
            lapack::dgees(
                jobvs, sort, None, n, a, lda, sdim, wr, wi, vs, ldvs, work, lwork, bwork, &mut info,
            );
        }
        if info != 0 {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
//}}}

//{{{ impl: Gees for f32
impl Gees for f32 {
    #[inline]
    fn gees(
        jobvs: u8,
        sort: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        sdim: &mut i32,
        wr: &mut [Self],
        wi: &mut [Self],
        vs: &mut [Self],
        ldvs: i32,
        work: &mut [Self],
        lwork: i32,
        bwork: &mut [i32],
    ) -> Result<(), Error> {
        let mut info = 0;
        unsafe {
            lapack::sgees(
                jobvs, sort, None, n, a, lda, sdim, wr, wi, vs, ldvs, work, lwork, bwork, &mut info,
            );
        }
        if info != 0 {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
//}}}

//{{{ struct: SchurRaw
pub(crate) struct SchurRaw<T> {
    pub q_data: Vec<T>,
    pub t_data: Vec<T>,
}
//}}}
//{{{ fun: schur_raw
/// Shared GEES algorithm. Consumes the cloned matrix data; returns raw Q/T buffers.
pub(crate) fn schur_raw<T>(
    mut a_data: Vec<T>,
    n: usize,
    m: usize,
) -> Result<SchurRaw<T>, Error>
where
    T: Gees + crate::common::One + crate::common::Zero + crate::common::Field + Default + Copy,
{
    let mut vs = vec![T::zero(); n * m];
    let mut wr = vec![T::zero(); n];
    let mut wi = vec![T::zero(); n];
    let mut sdim = 0;
    let mut work = vec![T::zero(); n * 5];
    let lwork = (n * 5) as i32;
    let mut bwork = vec![0; n];
    T::gees(
        b'V',
        b'N',
        n as i32,
        &mut a_data,
        n as i32,
        &mut sdim,
        &mut wr,
        &mut wi,
        &mut vs,
        n as i32,
        &mut work,
        lwork,
        &mut bwork,
    )?;
    Ok(SchurRaw {
        q_data: vs,
        t_data: a_data,
    })
}
//}}}
