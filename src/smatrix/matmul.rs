//! BLAS-accelerated matrix multiplication for [`SMatrix`].
//!
//! Implements the [`MatMul`] trait for `&SMatrix<T, M, K>` × `&SMatrix<T, K, N>` pairs using
//! BLAS routines, where the inner dimension K must match at compile time. The implementation
//! dispatches to `Gemm` for the general matrix–matrix case and to `Gemv` for the special
//! cases where one operand is effectively a vector (M = 1 or N = 1), choosing the most efficient
//! BLAS Level-2 or Level-3 call. All arithmetic is performed in column-major order.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::blaslapack::gemm::Gemm;
use crate::blaslapack::gemv::Gemv;
use crate::common::{Field, MatMul, One, Zero};
use crate::dmatrix::DMatrix;
use crate::ops::matmul::matmul_dispatch;
//}}}
//--------------------------------------------------------------------------------------------------

// SMatrix to SMatrix borrowed rhs impls
//{{{ trait: MatMul<&'a SMatrix<T, K, N>> for &'a SMatrix<T, M, K>
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
        matmul_dispatch(M, K, N, &self.data, &rhs.data, &mut result.data);
        result
    }
}

//}}}
//{{{ trait: MatMul<&'a SMatrix<T, K, N>> for &'a mut SMatrix<T, M, K>
impl<'a, T, const N: usize, const M: usize, const K: usize> MatMul<&'a SMatrix<T, K, N>>
    for &'a mut SMatrix<T, M, K>
where
    [(); M * K]:,
    [(); K * N]:,
    [(); M * N]:,
    T: Gemm + Gemv + Field + Zero + One + Copy,
{
    type Output = SMatrix<T, M, N>;

    #[inline]
    fn matmul(
        self,
        rhs: &'a SMatrix<T, K, N>,
    ) -> Self::Output
    {
        (&*self).matmul(rhs)
    }
}
//}}}
//{{{ trait: MatMul<&'a mut SMatrix<T, K, N>> for &'a SMatrix<T, M, K>
impl<'a, T, const N: usize, const M: usize, const K: usize> MatMul<&'a mut SMatrix<T, K, N>>
    for &'a SMatrix<T, M, K>
where
    [(); M * K]:,
    [(); K * N]:,
    [(); M * N]:,
    T: Gemm + Gemv + Field + Zero + One + Copy,
{
    type Output = SMatrix<T, M, N>;

    #[inline]
    fn matmul(
        self,
        rhs: &'a mut SMatrix<T, K, N>,
    ) -> Self::Output
    {
        self.matmul(&*rhs)
    }
}
//}}}
//{{{ trait: MatMul<&'a mut SMatrix<T, K, N>> for &'a mut SMatrix<T, M, K>
impl<'a, T, const N: usize, const M: usize, const K: usize> MatMul<&'a mut SMatrix<T, K, N>>
    for &'a mut SMatrix<T, M, K>
where
    [(); M * K]:,
    [(); K * N]:,
    [(); M * N]:,
    T: Gemm + Gemv + Field + Zero + One + Copy,
{
    type Output = SMatrix<T, M, N>;

    #[inline]
    fn matmul(
        self,
        rhs: &'a mut SMatrix<T, K, N>,
    ) -> Self::Output
    {
        (&*self).matmul(&*rhs)
    }
}
//}}}

// SMatrix to SMatrix owned rhs impls
//{{{ trait: MatMul<SMatrix<T, K, N>> for &SMatrix<T, M, K>
impl<T, const N: usize, const M: usize, const K: usize> MatMul<SMatrix<T, K, N>>
    for &SMatrix<T, M, K>
where
    [(); M * K]:,
    [(); K * N]:,
    [(); M * N]:,
    T: Gemm + Gemv + Field + Zero + One + Copy,
{
    type Output = SMatrix<T, M, N>;

    #[inline]
    fn matmul(
        self,
        rhs: SMatrix<T, K, N>,
    ) -> Self::Output
    {
        self.matmul(&rhs)
    }
}
//}}}
//{{{ trait: MatMul<SMatrix<T, K, N>> for &mut SMatrix<T, M, K>
impl<T, const N: usize, const M: usize, const K: usize> MatMul<SMatrix<T, K, N>>
    for &mut SMatrix<T, M, K>
where
    [(); M * K]:,
    [(); K * N]:,
    [(); M * N]:,
    T: Gemm + Gemv + Field + Zero + One + Copy,
{
    type Output = SMatrix<T, M, N>;

    #[inline]
    fn matmul(
        self,
        rhs: SMatrix<T, K, N>,
    ) -> Self::Output
    {
        (&*self).matmul(&rhs)
    }
}
//}}}

// SMatrix to DMatrix borrowed rhs impls
//{{{ trait: MatMul<&'a DMatrix<T>> for &'a SMatrix<T, M, K>
impl<'a, T, const M: usize, const K: usize> MatMul<&'a DMatrix<T>> for &'a SMatrix<T, M, K>
where
    [(); M * K]:,
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
        matmul_dispatch(m, k, n, &self.data, &rhs.data, &mut result.data);
        result
    }
}
//}}}
//{{{ trait: MatMul<&'a DMatrix<T>> for &'a mut SMatrix<T, M, K>
impl<'a, T, const M: usize, const K: usize> MatMul<&'a DMatrix<T>> for &'a mut SMatrix<T, M, K>
where
    [(); M * K]:,
    T: Gemm + Gemv + Field + Zero + One + Copy,
{
    type Output = DMatrix<T>;

    #[inline]
    fn matmul(
        self,
        rhs: &'a DMatrix<T>,
    ) -> Self::Output
    {
        (&*self).matmul(rhs)
    }
}
//}}}
//{{{ trait: MatMul<&'a mut DMatrix<T>> for &'a SMatrix<T, M, K>
impl<'a, T, const M: usize, const K: usize> MatMul<&'a mut DMatrix<T>> for &'a SMatrix<T, M, K>
where
    [(); M * K]:,
    T: Gemm + Gemv + Field + Zero + One + Copy,
{
    type Output = DMatrix<T>;

    #[inline]
    fn matmul(
        self,
        rhs: &'a mut DMatrix<T>,
    ) -> Self::Output
    {
        self.matmul(&*rhs)
    }
}
//}}}
//{{{ trait: MatMul<&'a mut DMatrix<T>> for &'a mut SMatrix<T, M, K>
impl<'a, T, const M: usize, const K: usize> MatMul<&'a mut DMatrix<T>> for &'a mut SMatrix<T, M, K>
where
    [(); M * K]:,
    T: Gemm + Gemv + Field + Zero + One + Copy,
{
    type Output = DMatrix<T>;

    #[inline]
    fn matmul(
        self,
        rhs: &'a mut DMatrix<T>,
    ) -> Self::Output
    {
        (&*self).matmul(&*rhs)
    }
}
//}}}
