//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::common::{Field, Zero};
use crate::expression::unary_expr::{NegOp, UnaryExpr};
//}}}
//{{{ std imports
use std::ops::Neg;
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

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
