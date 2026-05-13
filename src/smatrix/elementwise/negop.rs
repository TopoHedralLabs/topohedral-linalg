//! Negation operator for [`SMatrix`]: unary minus on static matrices.
//!
//! Implements the [`Neg`] trait for both owned and borrowed [`SMatrix<T, N, M>`] values,
//! returning a `UnaryExpr` that wraps the operand and applies `NegOp` element-wise when
//! materialised. Because [`SMatrix`] is `Copy`, the owned and borrowed paths are equivalent in
//! cost; the lazy wrapper still avoids unnecessary allocation when negation is part of a larger
//! expression chain.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::common::{Field, Zero};
use crate::expression::unary_expr::{NegOp, UnaryExpr};
use crate::smatrix::SMatrix;
//}}}
//{{{ std imports
use std::ops::Neg;
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl: Neg for SMatrix
impl<T, const N: usize, const M: usize> Neg for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Zero + Default + Neg<Output = T> + Copy,
{
    type Output = SMatrix<T, N, M>;

    fn neg(self) -> Self
    {
        let mut result = SMatrix::<T, N, M>::zeros();
        for i in 0..N * M
        {
            result[i] = -self[i];
        }
        result
    }
}
//}}}
//{{{ impl: Neg for &SMatrix
impl<'a, T, const N: usize, const M: usize> Neg for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
{
    type Output = UnaryExpr<&'a SMatrix<T, N, M>, T, NegOp>;

    #[inline]
    fn neg(self) -> Self::Output
    {
        UnaryExpr::new(self, NegOp)
    }
}
//}}}
//{{{ impl: Neg for &mut SMatrix
impl<'a, T, const N: usize, const M: usize> Neg for &'a mut SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
{
    type Output = UnaryExpr<&'a mut SMatrix<T, N, M>, T, NegOp>;

    #[inline]
    fn neg(self) -> Self::Output
    {
        UnaryExpr::new(self, NegOp)
    }
}
//}}}
