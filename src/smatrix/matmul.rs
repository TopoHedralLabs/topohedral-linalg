//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::blaslapack::common::AsI32;
use crate::blaslapack::gemm::Gemm;
use crate::blaslapack::gemv::Gemv;
use crate::common::{Complex, Field, One, Zero};
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ trait: MatMul
/// Trait which provides the matrix multiplication operation `matmul`.
pub trait MatMul<Rhs = Self>
{
    type Output;

    /// Performs a matrix multiplication operation.
    fn matmul(
        self,
        rhs: Rhs,
    ) -> Self::Output;
}

//}}}
//{{{ trait: MatMul for SMatrix
impl<'a, T, const N: usize, const M: usize, const K: usize> MatMul<&'a SMatrix<T, K, N>>
    for &'a SMatrix<T, M, K>
where
    [(); M * K]:,
    [(); K * N]:,
    [(); M * N]:,
    T: Gemm + Gemv + Field + Zero + One + Copy,
{
    type Output = SMatrix<T, M, N>;

    fn matmul(
        self,
        rhs: &'a SMatrix<T, K, N>,
    ) -> Self::Output
    {
        let mut result = SMatrix::<T, M, N>::zeros();

        if N == 1
        {
            T::gemv(
                cblas::Transpose::None,
                M as i32,
                K as i32,
                T::one(),
                &self.data,
                M as i32,
                &rhs.data,
                1,
                T::zero(),
                &mut result.data,
                1,
            );
        }
        else if M == 1
        {
            T::gemv(
                cblas::Transpose::Ordinary,
                K as i32,
                N as i32,
                T::one(),
                &rhs.data,
                K as i32,
                &self.data,
                1,
                T::zero(),
                &mut result.data,
                1,
            );
        }
        else
        {
            T::gemm(
                cblas::Transpose::None, // transa: transpose left matrix
                cblas::Transpose::None, // transb: transpose right matrix
                M as i32,               // m: rows of result/left matrix
                N as i32,               // n: columns of result/right matrix
                K as i32,               // k: columns of left/rows of right
                T::one(),               // alpha: scaling factor for multiplication
                &self.data,             // a: left matrix data
                M as i32,               // lda: leading dimension of left matrix
                &rhs.data,              // b: right matrix data
                K as i32,               // ldb: leading dimension of right matrix
                T::zero(),              // beta: scaling factor for result matrix
                &mut result.data,       // c: result matrix data
                M as i32,               // ldc: leading dimension of result matrix
            );
        }

        result
    }
}

//}}}
