//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::*;
use super::super::smatrix::{Evaluate, SMatrix};
use crate::apply_for_all_types;
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
    #[inline]
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
//{{{ impl: Add for BinopExpr
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

    #[inline]
    fn add(self, rhs: BinopExpr<A, B, T, Op1>) ->  BinopExpr<BinopExpr<C, D, T, Op2>, BinopExpr<A, B, T, Op1>, T, AddOp>
    {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Sub for BinopExpr
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

    #[inline]
    fn sub(self, rhs: BinopExpr<A, B, T, Op1>) ->  BinopExpr<BinopExpr<C, D, T, Op2>, BinopExpr<A, B, T, Op1>, T, SubOp>
    {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Mul for BinopExpr
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

    #[inline]
    fn mul(self, rhs: BinopExpr<A, B, T, Op1>) ->  BinopExpr<BinopExpr<C, D, T, Op2>, BinopExpr<A, B, T, Op1>, T, MulOp>
    {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Div for BinopExpr
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

    #[inline]
    fn div(self, rhs: BinopExpr<A, B, T, Op1>) ->  BinopExpr<BinopExpr<C, D, T, Op2>, BinopExpr<A, B, T, Op1>, T, DivOp>
    {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Add<T> for BinopExpr
impl<A, B, T, Op> Add<T> for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone + IndexValue<usize, Output = T>,
    Op: BinOp,
{
    type Output = BinopExpr<Self, T, T, AddOp>;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Add<BinopExpr> for $type
macro_rules! impl_add_binop_expr {
    ($type:ty) => {
        impl<A, B, Op> Add<BinopExpr<A, B, $type, Op>> for $type
        where
            A: IndexValue<usize, Output = $type>,
            B: IndexValue<usize, Output = $type>,
            Op: BinOp,
        {
            type Output = BinopExpr<Self, BinopExpr<A, B, $type, Op>, $type, AddOp>;

            #[inline]
            fn add(self, rhs: BinopExpr<A, B, $type, Op>) -> Self::Output {
                BinopExpr {
                    a: self,
                    b: rhs,
                    _marker: std::marker::PhantomData,
                }
        }
    }
    };
}
apply_for_all_types!(impl_add_binop_expr);
//}}}
//{{{ impl: Sub<T> for BinopExpr
impl<A, B, T, Op> Sub<T> for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone + IndexValue<usize, Output = T>,
    Op: BinOp,
{
    type Output = BinopExpr<Self, T, T, SubOp>;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Sub<BinopExpr> for $type
macro_rules! impl_sub_binop_expr {
    ($type:ty) => {
        impl<A, B, Op> Sub<BinopExpr<A, B, $type, Op>> for $type
        where
            A: IndexValue<usize, Output = $type>,
            B: IndexValue<usize, Output = $type>,
            Op: BinOp,
        {
            type Output = BinopExpr<Self, BinopExpr<A, B, $type, Op>, $type, SubOp>;

            #[inline]
            fn sub(self, rhs: BinopExpr<A, B, $type, Op>) -> Self::Output {
                BinopExpr {
                    a: self,
                    b: rhs,
                    _marker: std::marker::PhantomData,
                }
        }
    }
    };
}
apply_for_all_types!(impl_sub_binop_expr);
//}}}
//{{{ impl: Mul<T> for BinopExpr
impl<A, B, T, Op> Mul<T> for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone + IndexValue<usize, Output = T>,
    Op: BinOp,
{
    type Output = BinopExpr<Self, T, T, MulOp>;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Mul<BinopExpr> for $type
macro_rules! impl_mul_binop_expr {
    ($type:ty) => {
        impl<A, B, Op> Mul<BinopExpr<A, B, $type, Op>> for $type
        where
            A: IndexValue<usize, Output = $type>,
            B: IndexValue<usize, Output = $type>,
            Op: BinOp,
        {
            type Output = BinopExpr<Self, BinopExpr<A, B, $type, Op>, $type, MulOp>;

            #[inline]
            fn mul(self, rhs: BinopExpr<A, B, $type, Op>) -> Self::Output {
                BinopExpr {
                    a: self,
                    b: rhs,
                    _marker: std::marker::PhantomData,
                }
        }
    }
    };
}
apply_for_all_types!(impl_mul_binop_expr);
//}}}
//{{{ impl: Div<T> for BinopExpr
impl<A, B, T, Op> Div<T> for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone + IndexValue<usize, Output = T>,
    Op: BinOp,
{
    type Output = BinopExpr<Self, T, T, DivOp>;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Div<BinopExpr> for $type
macro_rules! impl_div_binop_expr {
    ($type:ty) => {
        impl<A, B, Op> Div<BinopExpr<A, B, $type, Op>> for $type
        where
            A: IndexValue<usize, Output = $type>,
            B: IndexValue<usize, Output = $type>,
            Op: BinOp,
        {
            type Output = BinopExpr<Self, BinopExpr<A, B, $type, Op>, $type, DivOp>;

            #[inline]
            fn div(self, rhs: BinopExpr<A, B, $type, Op>) -> Self::Output {
                BinopExpr {
                    a: self,
                    b: rhs,
                    _marker: std::marker::PhantomData,
                }
        }
    }
    };
}
apply_for_all_types!(impl_div_binop_expr);
//}}}
