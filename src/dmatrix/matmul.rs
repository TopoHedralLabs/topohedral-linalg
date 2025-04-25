//! Matrix multiplication for dense, dynamically allocated matrices.
//!
//! This module provides functionality for performing matrix multiplication
//! on dense matrices (`DMatrix`).
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::blaslapack::common::AsI32;
use crate::blaslapack::gemm::Gemm;
use crate::blaslapack::gemv::Gemv;
use crate::common::{Field, One, Zero};
//}}}
//{{{ std imports
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

//{{{ impl MatMul for DMatrix
impl<'a, T> MatMul<&'a DMatrix<T>> for &'a DMatrix<T>
where
    T: Gemm + Gemv + Field + Zero + One + Copy,
{
    type Output = DMatrix<T>;

    fn matmul(
        self,
        rhs: &'a DMatrix<T>,
    ) -> Self::Output
    {
        let m = self.nrows;
        let k = self.ncols;
        let n = rhs.ncols;

        assert_eq!(
            k, rhs.nrows,
            "Matrix dimensions are incompatible for multiplication: {}x{} and {}x{}",
            self.nrows, self.ncols, rhs.nrows, rhs.ncols
        );

        let mut result = DMatrix::<T>::zeros(m, n);

        if n == 1
        {
            // Vector-matrix multiplication
            T::gemv(
                cblas::Transpose::None,
                m as i32,
                k as i32,
                T::one(),
                &self.data,
                m as i32,
                &rhs.data,
                1,
                T::zero(),
                &mut result.data,
                1,
            );
        }
        else if m == 1
        {
            // Matrix-vector multiplication
            T::gemv(
                cblas::Transpose::Ordinary,
                k as i32,
                n as i32,
                T::one(),
                &rhs.data,
                k as i32,
                &self.data,
                1,
                T::zero(),
                &mut result.data,
                1,
            );
        }
        else
        {
            // General matrix-matrix multiplication
            T::gemm(
                cblas::Transpose::None, // transa: transpose left matrix
                cblas::Transpose::None, // transb: transpose right matrix
                m as i32,               // m: rows of result/left matrix
                n as i32,               // n: columns of result/right matrix
                k as i32,               // k: columns of left/rows of right
                T::one(),               // alpha: scaling factor for multiplication
                &self.data,             // a: left matrix data
                m as i32,               // lda: leading dimension of left matrix
                &rhs.data,              // b: right matrix data
                k as i32,               // ldb: leading dimension of right matrix
                T::zero(),              // beta: scaling factor for result matrix
                &mut result.data,       // c: result matrix data
                m as i32,               // ldc: leading dimension of result matrix
            );
        }

        result
    }
}
//}}}
