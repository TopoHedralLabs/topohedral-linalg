//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::apply_for_all_integer_types;
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

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
