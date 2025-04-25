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

//{{{ trait: Gemm
/// Trait which signifies matrices of type can perform a general matrix multiplication (GEMM)
/// operation.

pub trait Gemm: Copy
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
