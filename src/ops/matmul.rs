use crate::blaslapack::gemm::Gemm;
use crate::blaslapack::gemv::Gemv;
use crate::common::{Field, One, Zero};

/// Core matrix-multiply dispatch shared by DMatrix and SMatrix impls.
///
/// Dispatches to GEMV when the result is a column vector (n == 1) or a row-times-matrix
/// product (m == 1), and to GEMM otherwise. `a` is m×k, `b` is k×n, `c` is m×n (column-major).
pub(crate) fn matmul_dispatch<T>(
    m: usize,
    k: usize,
    n: usize,
    a: &[T],
    b: &[T],
    c: &mut [T],
) where
    T: Gemm + Gemv + Field + Zero + One + Copy,
{
    if n == 1
    {
        T::gemv(
            cblas::Transpose::None,
            m as i32,
            k as i32,
            T::one(),
            a,
            m as i32,
            b,
            1,
            T::zero(),
            c,
            1,
        );
    }
    else if m == 1
    {
        T::gemv(
            cblas::Transpose::Ordinary,
            k as i32,
            n as i32,
            T::one(),
            b,
            k as i32,
            a,
            1,
            T::zero(),
            c,
            1,
        );
    }
    else
    {
        T::gemm(
            cblas::Transpose::None,
            cblas::Transpose::None,
            m as i32,
            n as i32,
            k as i32,
            T::one(),
            a,
            m as i32,
            b,
            k as i32,
            T::zero(),
            c,
            m as i32,
        );
    }
}
