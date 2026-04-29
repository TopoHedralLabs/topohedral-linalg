//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::common::{Field, Zero};
use crate::expression::unary_expr::{NegOp, UnaryExpr};
//}}}
//{{{ std imports
use std::ops::Neg;
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

impl<T> Neg for DMatrix<T>
where
    T: Field + Zero + Default + Neg<Output = T> + Copy,
{
    type Output = DMatrix<T>;

    fn neg(self) -> Self
    {
        let mut result = self.clone();
        for i in 0..self.nrows * self.ncols
        {
            result[i] = -self[i];
        }
        result
    }
}

impl<'a, T> Neg for &'a DMatrix<T>
where
    T: Field + Copy,
{
    type Output = UnaryExpr<&'a DMatrix<T>, T, NegOp>;

    #[inline]
    fn neg(self) -> Self::Output
    {
        UnaryExpr::new(self, NegOp)
    }
}

impl<'a, T> Neg for &'a mut DMatrix<T>
where
    T: Field + Copy,
{
    type Output = UnaryExpr<&'a mut DMatrix<T>, T, NegOp>;

    #[inline]
    fn neg(self) -> Self::Output
    {
        UnaryExpr::new(self, NegOp)
    }
}
