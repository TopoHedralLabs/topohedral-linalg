//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::*;
use super::super::smatrix::{Evaluate, SMatrix};
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

impl<A, B, Op1, C, D, Op2, T> Add<BinopExpr<A, B, T, Op1>> for BinopExpr<C, D, T, Op2>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    Op1: BinOp,
    C: IndexValue<usize, Output = T>,
    D: IndexValue<usize, Output = T>,
    Op2: BinOp,
    T: Field + Default + Copy + fmt::Display + Clone, 
{
    type Output = BinopExpr<BinopExpr<C, D, T, Op2>, BinopExpr<A, B, T, Op1>, T, AddOp>;

    fn add(self, rhs: BinopExpr<A, B, T, Op1>) ->  BinopExpr<BinopExpr<C, D, T, Op2>, BinopExpr<A, B, T, Op1>, T, AddOp>
    {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<A, B, Op1, C, D, Op2, T> Sub<BinopExpr<A, B, T, Op1>> for BinopExpr<C, D, T, Op2>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    Op1: BinOp,
    C: IndexValue<usize, Output = T>,
    D: IndexValue<usize, Output = T>,
    Op2: BinOp,
    T: Field + Default + Copy + fmt::Display + Clone, 
{
    type Output = BinopExpr<BinopExpr<C, D, T, Op2>, BinopExpr<A, B, T, Op1>, T, SubOp>;

    fn sub(self, rhs: BinopExpr<A, B, T, Op1>) ->  BinopExpr<BinopExpr<C, D, T, Op2>, BinopExpr<A, B, T, Op1>, T, SubOp>
    {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<A, B, Op1, C, D, Op2, T> Mul<BinopExpr<A, B, T, Op1>> for BinopExpr<C, D, T, Op2>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    Op1: BinOp,
    C: IndexValue<usize, Output = T>,
    D: IndexValue<usize, Output = T>,
    Op2: BinOp,
    T: Field + Default + Copy + fmt::Display + Clone, 
{
    type Output = BinopExpr<BinopExpr<C, D, T, Op2>, BinopExpr<A, B, T, Op1>, T, MulOp>;

    fn mul(self, rhs: BinopExpr<A, B, T, Op1>) ->  BinopExpr<BinopExpr<C, D, T, Op2>, BinopExpr<A, B, T, Op1>, T, MulOp>
    {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<A, B, Op1, C, D, Op2, T> Div<BinopExpr<A, B, T, Op1>> for BinopExpr<C, D, T, Op2>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    Op1: BinOp,
    C: IndexValue<usize, Output = T>,
    D: IndexValue<usize, Output = T>,
    Op2: BinOp,
    T: Field + Default + Copy + fmt::Display + Clone, 
{
    type Output = BinopExpr<BinopExpr<C, D, T, Op2>, BinopExpr<A, B, T, Op1>, T, DivOp>;

    fn div(self, rhs: BinopExpr<A, B, T, Op1>) ->  BinopExpr<BinopExpr<C, D, T, Op2>, BinopExpr<A, B, T, Op1>, T, DivOp>
    {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{
  use super::*;


  #[test]
  fn test_add_sub()
  {
    let a = SMatrix::<f64, 2, 2>::from_value(1.0);
    let b = SMatrix::<f64, 2, 2>::from_value(10.0);
    let c = SMatrix::<f64, 2, 2>::from_value(100.0);
    let d = SMatrix::<f64, 2, 2>::from_value(100.0);

    // let e = (a + b) - (c - d);
  }
}
//}}}
