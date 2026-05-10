//! BLAS-accelerated matrix multiplication for [`DMatrix`].
//!
//! Implements the [`MatMul`] trait for `&DMatrix<T>` pairs using BLAS routines. The
//! implementation dispatches to `Gemm` for the general matrix–matrix case and to `Gemv` for
//! the special cases where one operand is a column or row vector, choosing the most efficient
//! BLAS Level-2 or Level-3 call automatically. All work is performed in column-major order to
//! match LAPACK conventions.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::dmatrix::DMatrix;
use crate::blaslapack::{Gemm, Gemv, matmul_dispatch};
use crate::common::{Field, MatMul, One, Zero};
use crate::smatrix::SMatrix;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl MatMul<&'a DMatrix<T>> for &'a DMatrix<T>
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
        matmul_dispatch(m, k, n, &self.data, &rhs.data, &mut result.data);
        result
    }
}
//}}}
//{{{ impl MatMul<&'a DMatrix<T>> for &'a mut DMatrix<T>
impl<'a, T> MatMul<&'a DMatrix<T>> for &'a mut DMatrix<T>
where
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
//{{{ impl MatMul<&'a mut DMatrix<T>> for &'a DMatrix<T>
impl<'a, T> MatMul<&'a mut DMatrix<T>> for &'a DMatrix<T>
where
    T: Gemm + Gemv + Field + Zero + One + Copy,
{
    type Output = DMatrix<T>;

    fn matmul(
        self,
        rhs: &'a mut DMatrix<T>,
    ) -> Self::Output
    {
        self.matmul(&(*rhs))
    }
}
//}}}
//{{{ impl MatMul<&'a mut DMatrix<T>> for &'a mut DMatrix<T>
impl<'a, T> MatMul<&'a mut DMatrix<T>> for &'a mut DMatrix<T>
where
    T: Gemm + Gemv + Field + Zero + One + Copy,
{
    type Output = DMatrix<T>;

    fn matmul(
        self,
        rhs: &'a mut DMatrix<T>,
    ) -> Self::Output
    {
        (&*self).matmul(&(*rhs))
    }
}
//}}}

//{{{ impl MatMul<DMatrix<T>> for &DMatrix<T>
impl<T> MatMul<DMatrix<T>> for &DMatrix<T>
where
    T: Gemm + Gemv + Field + Zero + One + Copy,
{
    type Output = DMatrix<T>;

    #[inline]
    fn matmul(
        self,
        rhs: DMatrix<T>,
    ) -> Self::Output
    {
        self.matmul(&rhs)
    }
}
//}}}
//{{{ impl MatMul<DMatrix<T>> for &mut DMatrix<T>
impl<T> MatMul<DMatrix<T>> for &mut DMatrix<T>
where
    T: Gemm + Gemv + Field + Zero + One + Copy,
{
    type Output = DMatrix<T>;

    #[inline]
    fn matmul(
        self,
        rhs: DMatrix<T>,
    ) -> Self::Output
    {
        (&*self).matmul(&rhs)
    }
}
//}}}

//{{{ impl MatMul<&'a SMatrix<T, K, N>> for &'a DMatrix<T>
impl<'a, T, const K: usize, const N: usize> MatMul<&'a SMatrix<T, K, N>> for &'a DMatrix<T>
where
    [(); K * N]:,
    T: Gemm + Gemv + Field + Zero + One + Copy,
{
    type Output = DMatrix<T>;

    fn matmul(
        self,
        rhs: &'a SMatrix<T, K, N>,
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
//{{{ impl MatMul<&'a SMatrix<T, K, N>> for &'a mut DMatrix<T>
impl<'a, T, const K: usize, const N: usize> MatMul<&'a SMatrix<T, K, N>> for &'a mut DMatrix<T>
where
    [(); K * N]:,
    T: Gemm + Gemv + Field + Zero + One + Copy,
{
    type Output = DMatrix<T>;

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
//{{{ impl MatMul<&'a mut SMatrix<T, K, N>> for &'a DMatrix<T>
impl<'a, T, const K: usize, const N: usize> MatMul<&'a mut SMatrix<T, K, N>> for &'a DMatrix<T>
where
    [(); K * N]:,
    T: Gemm + Gemv + Field + Zero + One + Copy,
{
    type Output = DMatrix<T>;

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
//{{{ impl MatMul<&'a mut SMatrix<T, K, N>> for &'a mut DMatrix<T>
impl<'a, T, const K: usize, const N: usize> MatMul<&'a mut SMatrix<T, K, N>> for &'a mut DMatrix<T>
where
    [(); K * N]:,
    T: Gemm + Gemv + Field + Zero + One + Copy,
{
    type Output = DMatrix<T>;

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
