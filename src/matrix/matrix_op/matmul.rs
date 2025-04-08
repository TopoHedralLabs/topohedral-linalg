//! Short Description of module
//!
//! Longer description of module

//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::matrix::smatrix::*;
use crate::{apply_for_all_integer_types, common::*};

//}}}
//{{{ std imports
use std::fmt;

//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ trait: Gemm
/// Trait which signifies matrices of type can perform a general matrix multiplication (GEMM)
/// operation.

trait Gemm: Copy
{
    /// Performs a general matrix multiplication (GEMM) operation.
    ///
    /// This function computes the matrix-matrix product of two matrices `a` and `b`
    /// and adds the result to the matrix `c`, using the formula:
    ///
    /// $$
    /// \mathbf{C} = \alpha * op(\mathbf{A}) * op(\mathbf{B}) + \beta * \mathbf{C}
    /// $$
    ///
    /// where `op(x)` is either `x` or `x^T`, depending on the values of `tr1` and `tr2`.
    ///
    /// The function takes the following parameters:
    /// - `tr1`: the transpose operation to apply to matrix `a`
    /// - `tr2`: the transpose operation to apply to matrix `b`
    /// - `m`: the number of rows of the resulting matrix `c`
    /// - `n`: the number of columns of the resulting matrix `c`
    /// - `k`: the number of columns of matrix `a` (or rows of matrix `b`)
    /// - `alpha`: the scalar factor applied to the product of `a` and `b`
    /// - `a`: the first input matrix
    /// - `lda`: the leading dimension of matrix `a`
    /// - `b`: the second input matrix
    /// - `ldb`: the leading dimension of matrix `b`
    /// - `beta`: the scalar factor applied to matrix `c`
    /// - `c`: the output matrix
    /// - `ldc`: the leading dimension of matrix `c`

    fn gemm(
        tr1: cblas::Transpose,
        tr2: cblas::Transpose,
        m: i32,
        n: i32,
        k: i32,
        alpha: Self,
        a: &[Self],
        lda: i32,
        b: &[Self],
        ldb: i32,
        beta: Self,
        c: &mut [Self],
        ldc: i32,
    );
}

//}}}
//{{{ impl: Gemm for f64
impl Gemm for f64
{
    #[inline]

    fn gemm(
        tr1: cblas::Transpose,
        tr2: cblas::Transpose,
        m: i32,
        n: i32,
        k: i32,
        alpha: Self,
        a: &[Self],
        lda: i32,
        b: &[Self],
        ldb: i32,
        beta: Self,
        c: &mut [Self],
        ldc: i32,
    )
    {

        unsafe {

            cblas::dgemm(
                cblas::Layout::ColumnMajor,
                tr1,
                tr2,
                m,
                n,
                k,
                alpha,
                a,
                lda,
                b,
                ldb,
                beta,
                c,
                ldc,
            )
        }
    }
}

//}}}
//{{{ impl: Gemm for f32
impl Gemm for f32
{
    #[inline]

    fn gemm(
        tr1: cblas::Transpose,
        tr2: cblas::Transpose,
        m: i32,
        n: i32,
        k: i32,
        alpha: Self,
        a: &[Self],
        lda: i32,
        b: &[Self],
        ldb: i32,
        beta: Self,
        c: &mut [Self],
        ldc: i32,
    )
    {

        unsafe {

            cblas::sgemm(
                cblas::Layout::ColumnMajor,
                tr1,
                tr2,
                m,
                n,
                k,
                alpha,
                a,
                lda,
                b,
                ldb,
                beta,
                c,
                ldc,
            )
        }
    }
}

//}}}
//{{{ impl: Gemm for all integer types
macro_rules! impl_naive_gemm {
    ($t:ty) => {
        impl Gemm for $t
        {
            #[inline]

            fn gemm(
                tr1: cblas::Transpose,
                tr2: cblas::Transpose,
                m: i32,
                n: i32,
                k: i32,
                alpha: Self,
                a: &[Self],
                lda: i32,
                b: &[Self],
                ldb: i32,
                beta: Self,
                c: &mut [Self],
                ldc: i32,
            )
            {

                let get_a = |i, j| a[i as usize + (j as usize * lda as usize)];

                let get_b = |i, j| b[i as usize + (j as usize * ldb as usize)];

                for i in 0..m
                {

                    for j in 0..n
                    {

                        let mut sum = Self::default();

                        for l in 0..k
                        {

                            sum += get_a(i, l) * get_b(l, j);
                        }

                        let idx = i as usize + (j as usize * ldc as usize);

                        c[idx] = alpha * sum + beta * c[idx];
                    }
                }
            }
        }
    };
}

apply_for_all_integer_types!(impl_naive_gemm);

//}}}
//{{{ trait: Gemv
/// Trait which signifies matrices of type can perform a general matrix-vector multiplication (GEMV)
/// operation.

pub trait Gemv: Copy
{
    /// Performs a general matrix-vector multiplication (GEMV) operation.
    ///
    /// This function computes the matrix-vector product:
    ///
    /// $$
    /// y = alpha * op(A) * x + beta * y
    /// $$
    ///
    /// where:
    /// - `op(A)` is the operation applied to the matrix `A`, which can be normal (no transpose), transpose, or conjugate transpose.
    /// - `m` is the number of rows of the matrix `A`.
    /// - `k` is the number of columns of the matrix `A`.
    /// - `alpha` and `beta` are scalar values.
    /// - `a` is the input matrix `A`.
    /// - `lda` is the leading dimension of the matrix `A`.
    /// - `x` is the input vector.
    /// - `incx` is the increment for the elements of `x`.
    /// - `y` is the output vector.
    /// - `incy` is the increment for the elements of `y`.

    fn gemv(
        tr: cblas::Transpose,
        m: i32,
        k: i32,
        alpha: Self,
        a: &[Self],
        lda: i32,
        x: &[Self],
        incx: i32,
        beta: Self,
        y: &mut [Self],
        incy: i32,
    );
}

//}}}
//{{{ impl: Gemv for f64
impl Gemv for f64
{
    #[inline]

    fn gemv(
        tr: cblas::Transpose,
        m: i32,
        k: i32,
        alpha: Self,
        a: &[Self],
        lda: i32,
        x: &[Self],
        incx: i32,
        beta: Self,
        y: &mut [Self],
        incy: i32,
    )
    {

        unsafe {

            cblas::dgemv(
                cblas::Layout::ColumnMajor,
                tr,
                m,
                k,
                alpha,
                a,
                lda,
                x,
                incx,
                beta,
                y,
                incy,
            )
        }
    }
}

//}}}
//{{{ impl: Gemv for f32
impl Gemv for f32
{
    #[inline]

    fn gemv(
        tr: cblas::Transpose,
        m: i32,
        k: i32,
        alpha: Self,
        a: &[Self],
        lda: i32,
        x: &[Self],
        incx: i32,
        beta: Self,
        y: &mut [Self],
        incy: i32,
    )
    {

        unsafe {

            cblas::sgemv(
                cblas::Layout::ColumnMajor,
                tr,
                m,
                k,
                alpha,
                a,
                lda,
                x,
                incx,
                beta,
                y,
                incy,
            )
        }
    }
}

//}}}
//{{{ impl: Gemv for all integer types
macro_rules! impl_naive_gemv {
    ($t:ty) => {
        impl Gemv for $t
        {
            #[inline]

            fn gemv(
                tr: cblas::Transpose,
                m: i32,
                k: i32,
                alpha: Self,
                a: &[Self],
                lda: i32,
                x: &[Self],
                incx: i32,
                beta: Self,
                y: &mut [Self],
                incy: i32,
            )
            {

                let get_a = |i, j| a[i as usize + (j as usize * lda as usize)];

                match tr
                {
                    cblas::Transpose::None =>
                    {

                        for i in 0..m
                        {

                            let mut sum = Self::default();

                            for j in 0..k
                            {

                                sum += get_a(i, j) * x[(j * incx) as usize];
                            }

                            y[(i * incy) as usize] = alpha * sum + beta * y[(i * incy) as usize];
                        }
                    }
                    cblas::Transpose::Ordinary =>
                    {

                        for i in 0..m
                        {

                            let mut sum = Self::default();

                            for j in 0..k
                            {

                                sum += get_a(j, i) * x[(j * incx) as usize];
                            }

                            y[(i * incy) as usize] = alpha * sum + beta * y[(i * incy) as usize];
                        }
                    }
                    _ =>
                    {} // Handle other transpose cases if needed
                }
            }
        }
    };
}

apply_for_all_integer_types!(impl_naive_gemv);

//}}}
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
    T: Gemm + Gemv + Field + Zero + One + Copy + fmt::Display + Default,
{
    type Output = SMatrix<T, M, N>;

    fn matmul(
        self,
        rhs: &'a SMatrix<T, K, N>,
    ) -> Self::Output
    {

        let mut result = SMatrix::<T, M, N>::default();

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

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]

mod tests
{

    use super::*;
    use super::super::super::common::{SMatrixConstructors, DMatrixConstructors};

    // struct TestData1
}

//}}}
