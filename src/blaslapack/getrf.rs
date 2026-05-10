//! LAPACK `dgetrf`/`sgetrf` wrapper for LU factorisation with partial pivoting.
//!
//! Provides the [`Getrf`] trait, wrapping the LAPACK `?getrf` routine that factors a general
//! m×n matrix A into P·L·U using partial pivoting with row interchanges. The pivot array is
//! returned alongside the factored storage; a typed error is produced when the matrix is singular.
//! Implementations for `f64` and `f32` call the Fortran LAPACK ABI via the `lapack` crate.
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
/// Errors returned by the [`Getrf`] LAPACK wrapper.
#[derive(Error, Debug)]
pub enum Error
{
    /// LAPACK returned a non-zero info code indicating a singular or invalid matrix.
    #[error("Error in getrf, exited with code {0}")]
    LapackError(i32),
}
//}}}

//{{{ trait: Getrf
/// Trait for types that support LU factorization.
pub trait Getrf: Copy
{
    /// Performs LU factorization of a general M-by-N matrix A using partial pivoting
    /// with row interchanges.
    ///
    /// The factorization has the form:
    /// A = P * L * U
    ///
    /// where P is a permutation matrix, L is lower triangular with unit diagonal
    /// elements, and U is upper triangular.
    fn getrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        ipiv: &mut [i32],
    ) -> Result<(), Error>;
}
//}}}
//{{{ impl: Getrf for f64
impl Getrf for f64
{
    #[inline]
    fn getrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        ipiv: &mut [i32],
    ) -> Result<(), Error>
    {
        let mut info = 0;
        unsafe {
            lapack::dgetrf(m, n, a, lda, ipiv, &mut info);
        }
        if info != 0
        {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
//}}}
//{{{ impl: Getrf for f32
impl Getrf for f32
{
    #[inline]
    fn getrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        ipiv: &mut [i32],
    ) -> Result<(), Error>
    {
        let mut info = 0;
        unsafe {
            lapack::sgetrf(m, n, a, lda, ipiv, &mut info);
        }
        if info != 0
        {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
//}}}

//{{{ struct: LuRaw
pub(crate) struct LuRaw<T>
{
    pub l_data:    Vec<T>,
    pub u_data:    Vec<T>,
    pub p_data:    Vec<T>,
    pub num_swaps: usize,
}
//}}}
//{{{ fun: lu_raw
/// Shared GETRF algorithm. Consumes the cloned matrix data; returns raw L/U/P buffers.
pub(crate) fn lu_raw<T>(
    mut a_data: Vec<T>,
    n: usize,
    m: usize,
) -> Result<LuRaw<T>, Error>
where
    T: Getrf + crate::common::One + crate::common::Zero + crate::common::Field + Copy,
{
    let mut ipiv = vec![0; n.min(m)];
    T::getrf(n as i32, m as i32, &mut a_data, n as i32, &mut ipiv)?;

    let mut l_data = vec![T::zero(); n * m];
    let mut u_data = vec![T::zero(); n * m];
    for i in 0..n
    {
        for j in 0..m
        {
            let idx = i + j * n;
            if i > j
            {
                l_data[idx] = a_data[idx];
            }
            else if i == j
            {
                l_data[idx] = T::one();
                u_data[idx] = a_data[idx];
            }
            else
            {
                u_data[idx] = a_data[idx];
            }
        }
    }

    let mut p_data = vec![T::zero(); n * m];
    for i in 0..n.min(m)
    {
        p_data[i + i * n] = T::one();
    }
    let mut num_swaps = 0;
    for (k, &pivot) in ipiv.iter().enumerate()
    {
        let pivot = (pivot - 1) as usize;
        if k != pivot
        {
            for j in 0..m
            {
                p_data.swap(k + j * n, pivot + j * n);
                num_swaps += 1;
            }
        }
    }
    Ok(LuRaw { l_data, u_data, p_data, num_swaps })
}
//}}}
