//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::*;
use super::smatrix::{Evaluate, SMatrix};
//}}}
//{{{ std imports 
use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
//}}}
//{{{ dep imports 
use topohedral_tracing::*;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ struct AddOp
pub struct AddOp; 
//}}}
//{{{ struct SubOp
pub struct SubOp;
//}}}
//{{{ struct MulOp
pub struct MulOp;
//}}}
//{{{ struct DivOp
pub struct DivOp;
//}}}
//{{{ trait: BinOp
pub trait BinOp {
    fn apply<T: Field>(a: T, b: T) -> T;
}
//}}}
//{{{ impl BinOp for AddOp
impl BinOp for AddOp {
    #[inline]
    fn apply<T: Field>(a: T, b: T) -> T { a + b }
}
//}}}
//{{{ impl BinOp for SubOp
impl BinOp for SubOp {
    #[inline]
    fn apply<T: Field>(a: T, b: T) -> T { a - b }
}
//}}}
//{{{ impl BinOp for MulOp
impl BinOp for MulOp {
    #[inline]
    fn apply<T: Field>(a: T, b: T) -> T { a * b }
}
//}}}
//{{{ impl BInOp for DivOp
impl BinOp for DivOp {
    #[inline]
    fn apply<T: Field>(a: T, b: T) -> T { a / b }
}
//}}}
//{{{ struct: BinopExpr 
pub struct BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
{
    pub a: A,
    pub b: B,
    pub _marker: std::marker::PhantomData<(T, Op)>,
}
//}}}
//{{{ impl: IndexValue for BinopExpr
impl<A, B, T, Op> IndexValue<usize> for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
{
    type Output = T;
    fn index_value(&self, index: usize) -> Self::Output {
        //{{{ trace
        debug!("Calling BinopExpr::index_value with index = {}", index);
        debug!("a.index_value(index) = {}", self.a.index_value(index)); 
        debug!("b.index_value(index) = {}", self.b.index_value(index)); 
        //}}}
        Op::apply(self.a.index_value(index), self.b.index_value(index))
    }
}
//}}}
//{{{ impl: Evaluate for BinopExpr
impl <A, B, T, const N: usize, const M: usize, Op> Evaluate<T, N, M>  for BinopExpr<A, B, T, Op>
where
    [(); N * M]:,
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
{
    fn eval(&self) -> SMatrix<T, N, M> {

        //{{{ trace
        debug!("Calling BinopExpr::eval()");
        //}}}
        let mut out = SMatrix::<T, N, M>::default();

        for i in 0..N*M
        {
            out.data[i] = self.index_value(i);
        }
        out
    }
}
//}}}

