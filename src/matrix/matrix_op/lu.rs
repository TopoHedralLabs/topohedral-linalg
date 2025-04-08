//! Provides LU decomposition of a matrix.
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

use rand::distributions::uniform::SampleUniform;

//{{{ crate imports
use crate::common::*;
use crate::matrix::smatrix::*;
//}}}
//{{{ std imports
use std::fmt;
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: LUError
/// Errors that can occur during LU decomposition.
///
/// The `LUError` enum represents the different types of errors that can occur during the LU decomposition
/// of a matrix. The `InvalidArgument` variant indicates that one of the arguments passed to the LU
/// decomposition function was invalid, while the `DiagonalZero` variant indicates that the diagonal
/// element of the matrix became zero during the decomposition, which is not allowed.
#[derive(Error, Debug)]
pub enum LUError {
    #[error("Error in LU, argument {0} is invalid")]
    InvalidArgument(i32), 
    #[error("Error in LU, diagonal element is zero")]
    DiagonalZero,
}
//}}}
//{{{ sturct: SLU
/// Represents the LU decomposition of a matrix.
///
/// The LU decomposition is a factorization of a matrix into the product of a lower triangular matrix
/// and an upper triangular matrix. This struct stores the L, U, and permutation matrices resulting
/// from the LU decomposition.
pub struct SLU<T, const N: usize, const M: usize>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display,
{
    pub l: SMatrix<T, N, M>,
    pub u: SMatrix<T, N, M>,
    pub p: SMatrix<T, N, M>,
}
//}}}
//{{{ trait: Getrf
/// Trait for types that support LU factorization.
trait Getrf: Copy
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
    ) -> i32;
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
    ) -> i32
    {
        let mut info = 0;
        unsafe {
            lapack::dgetrf(m, n, a, lda, ipiv, &mut info);
        }
        info
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
    ) -> i32
    {
        let mut info = 0;
        unsafe {
            lapack::sgetrf(m, n, a, lda, ipiv, &mut info);
        }
        info
    }
}
//}}}
//{{{ impl SMatrix<T, N, M> 
#[allow(private_bounds)]
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    [(); N * M]:,
    T: One + Zero + Getrf + Field + Default + Copy + fmt::Display + SampleUniform,
{
    pub fn lu(&self) -> Result<SLU<T, N, M>, LUError>
    {
        //{{{ com: call getrf and check for errors
        let mut a = self.clone();
        let mut ipiv = vec![0; N.min(M)];
        let info = T::getrf(N as i32, M as i32, &mut a.data, N as i32, &mut ipiv);
        if info > 0
        {
            return Err(LUError::InvalidArgument(info));
        }
        else if info < 0 
        {
            return Err(LUError::DiagonalZero);
        }
        //}}}
        //{{{ com:  Extract L and U matrices from the factorized matrix
        let mut l = SMatrix::<T, N, M>::default();
        let mut u = SMatrix::<T, N, M>::default();

        for i in 0..N
        {
            for j in 0..M
            {
                if i > j
                {
                    l.data[i + j * N] = a.data[i + j * N];
                }
                else if i == j
                {
                    l.data[i + j * N] = T::one();
                    u.data[i + j * N] = a.data[i + j * N];
                }
                else
                {
                    u.data[i + j * N] = a.data[i + j * N];
                }
            }
        }
        //}}}
        //{{{ com: Create permutation matrix from ipiv
        let mut p = SMatrix::<T, N, M>::identity();
        for (k, &pivot) in ipiv.iter().enumerate()
        {
            let pivot = (pivot - 1) as usize;
            if k != pivot
            {
                for j in 0..M
                {
                    p.data.swap(k + j * N, pivot + j * N);
                }
            }
        }
        //}}}
        Ok(SLU { l, u, p })
    }
}
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{
}
//}}}
