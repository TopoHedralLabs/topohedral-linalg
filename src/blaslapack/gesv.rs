//! LAPACK `dgesv`/`sgesv` wrapper for solving general linear systems.
//!
//! Provides the [`Gesv`] trait, wrapping the LAPACK `?gesv` routine that solves A X = B for X by
//! performing LU factorisation with partial pivoting in place. The coefficient matrix A and
//! right-hand-side B are overwritten on return; the solution X occupies the space originally held
//! by B. A typed error is returned when the matrix is singular. This is the LAPACK driver used by
//! both `DMatrix::solve()` and `SMatrix::solve()`.
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
/// Errors returned by the [`Gesv`] LAPACK wrapper.
#[derive(Error, Debug)]
pub enum Error {
    /// LAPACK returned a non-zero info code indicating a singular coefficient matrix.
    #[error("Error in gesv, exited with code {0}")]
    LapackError(i32),
}
//}}}

//{{{ trait: Gesv
/// Trait for types that support solving a general linear system A X = B via LU factorisation.
pub trait Gesv: Copy {
    /// Solves the linear system A X = B, overwriting A with its LU factorisation and B with X.
    fn gesv(
        n: i32,
        nrhs: i32,
        a: &mut [Self],
        lda: i32,
        ipiv: &mut [i32],
        b: &mut [Self],
        ldb: i32,
    ) -> Result<(), Error>;
}
//}}}

//{{{ impl: Gesv for f64
impl Gesv for f64 {
    #[inline]
    fn gesv(
        n: i32,
        nrhs: i32,
        a: &mut [Self],
        lda: i32,
        ipiv: &mut [i32],
        b: &mut [Self],
        ldb: i32,
    ) -> Result<(), Error> {
        let mut info = 0;
        unsafe {
            lapack::dgesv(n, nrhs, a, lda, ipiv, b, ldb, &mut info);
        }
        if info != 0 {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
//}}}

//{{{ impl: Gesv for f32
impl Gesv for f32 {
    #[inline]
    fn gesv(
        n: i32,
        nrhs: i32,
        a: &mut [Self],
        lda: i32,
        ipiv: &mut [i32],
        b: &mut [Self],
        ldb: i32,
    ) -> Result<(), Error> {
        let mut info = 0;
        unsafe {
            lapack::sgesv(n, nrhs, a, lda, ipiv, b, ldb, &mut info);
        }
        if info != 0 {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
//}}}

//{{{ fun: solve_raw
/// Shared GESV algorithm. Returns the solution data (overwrites the rhs).
pub(crate) fn solve_raw<T>(
    mut a_data: Vec<T>,
    mut b_data: Vec<T>,
    n: usize,
    nrhs: usize,
) -> Result<Vec<T>, Error>
where
    T: Gesv + crate::common::Field,
{
    let mut ipiv = vec![0; n];
    T::gesv(
        n as i32,
        nrhs as i32,
        &mut a_data,
        n as i32,
        &mut ipiv,
        &mut b_data,
        n as i32,
    )?;
    Ok(b_data)
}
//}}}
