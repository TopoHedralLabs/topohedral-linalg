//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::apply_for_all_types;
use crate::common::{Field, IndexValue};
//}}}
//{{{ std imports
use std::ops::{Add, Div, Mul, Sub};
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ struct AddOp
#[doc(hidden)]
pub struct AddOp;

//}}}
//{{{ struct SubOp
#[doc(hidden)]
pub struct SubOp;

//}}}
//{{{ struct MulOp
#[doc(hidden)]
pub struct MulOp;

//}}}
//{{{ struct DivOp
#[doc(hidden)]
pub struct DivOp;

//}}}
//{{{ trait: BinOp
#[doc(hidden)]
pub trait BinOp
{
    fn apply<T: Field>(
        a: T,
        b: T,
    ) -> T;
}

//}}}
//{{{ impl BinOp for AddOp
#[doc(hidden)]
impl BinOp for AddOp
{
    #[inline]
    fn apply<T: Field>(
        a: T,
        b: T,
    ) -> T
    {
        a + b
    }
}

//}}}
//{{{ impl BinOp for SubOp
#[doc(hidden)]
impl BinOp for SubOp
{
    #[inline]
    fn apply<T: Field>(
        a: T,
        b: T,
    ) -> T
    {
        a - b
    }
}

//}}}
//{{{ impl BinOp for MulOp
#[doc(hidden)]
impl BinOp for MulOp
{
    #[inline]
    fn apply<T: Field>(
        a: T,
        b: T,
    ) -> T
    {
        a * b
    }
}

//}}}
//{{{ impl BInOp for DivOp
#[doc(hidden)]
impl BinOp for DivOp
{
    #[inline]
    fn apply<T: Field>(
        a: T,
        b: T,
    ) -> T
    {
        a / b
    }
}

//}}}
//{{{ struct: BinopExpr
#[doc(hidden)]
pub struct BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Copy,
    Op: BinOp,
{
    pub a: A,
    pub b: B,
    pub nrows: usize,
    pub ncols: usize,
    pub _marker: std::marker::PhantomData<(T, Op)>,
}

//}}}
//{{{ impl: IndexValue for BinopExpr
#[doc(hidden)]
impl<A, B, T, Op> IndexValue<usize> for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Copy,
    Op: BinOp,
{
    type Output = T;

    #[inline]
    fn index_value(
        &self,
        index: usize,
    ) -> Self::Output
    {
        Op::apply(self.a.index_value(index), self.b.index_value(index))
    }
}

//}}}
//{{{ impl: Add for BinopExpr
#[doc(hidden)]
impl<A, B, Op1, C, D, Op2, T> Add<BinopExpr<A, B, T, Op1>> for BinopExpr<C, D, T, Op2>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    Op1: BinOp,
    C: IndexValue<usize, Output = T>,
    D: IndexValue<usize, Output = T>,
    Op2: BinOp,
    T: Field + Copy,
{
    type Output = BinopExpr<BinopExpr<C, D, T, Op2>, BinopExpr<A, B, T, Op1>, T, AddOp>;

    #[inline]
    fn add(
        self,
        rhs: BinopExpr<A, B, T, Op1>,
    ) -> BinopExpr<BinopExpr<C, D, T, Op2>, BinopExpr<A, B, T, Op1>, T, AddOp>
    {
        let nr = self.nrows;
        let nc = self.ncols;
        BinopExpr {
            a: self,
            b: rhs,
            nrows: nr,
            ncols: nc,
            _marker: std::marker::PhantomData,
        }
    }
}

//}}}
//{{{ impl: Sub for BinopExpr
#[doc(hidden)]
impl<A, B, Op1, C, D, Op2, T> Sub<BinopExpr<A, B, T, Op1>> for BinopExpr<C, D, T, Op2>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    Op1: BinOp,
    C: IndexValue<usize, Output = T>,
    D: IndexValue<usize, Output = T>,
    Op2: BinOp,
    T: Field + Copy,
{
    type Output = BinopExpr<BinopExpr<C, D, T, Op2>, BinopExpr<A, B, T, Op1>, T, SubOp>;

    #[inline]
    fn sub(
        self,
        rhs: BinopExpr<A, B, T, Op1>,
    ) -> BinopExpr<BinopExpr<C, D, T, Op2>, BinopExpr<A, B, T, Op1>, T, SubOp>
    {
        debug_assert!(self.nrows == rhs.nrows);
        debug_assert!(self.ncols == rhs.ncols);
        let nr = self.nrows;
        let nc = self.ncols;
        BinopExpr {
            a: self,
            b: rhs,
            nrows: nr,
            ncols: nc,
            _marker: std::marker::PhantomData,
        }
    }
}

//}}}
//{{{ impl: Mul for BinopExpr
#[doc(hidden)]
impl<A, B, Op1, C, D, Op2, T> Mul<BinopExpr<A, B, T, Op1>> for BinopExpr<C, D, T, Op2>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    Op1: BinOp,
    C: IndexValue<usize, Output = T>,
    D: IndexValue<usize, Output = T>,
    Op2: BinOp,
    T: Field + Copy,
{
    type Output = BinopExpr<BinopExpr<C, D, T, Op2>, BinopExpr<A, B, T, Op1>, T, MulOp>;

    #[inline]
    fn mul(
        self,
        rhs: BinopExpr<A, B, T, Op1>,
    ) -> BinopExpr<BinopExpr<C, D, T, Op2>, BinopExpr<A, B, T, Op1>, T, MulOp>
    {
        debug_assert!(self.nrows == rhs.nrows);
        debug_assert!(self.ncols == rhs.ncols);
        let nr = self.nrows;
        let nc = self.ncols;
        BinopExpr {
            a: self,
            b: rhs,
            nrows: nr,
            ncols: nc,
            _marker: std::marker::PhantomData,
        }
    }
}

//}}}
//{{{ impl: Div for BinopExpr
#[doc(hidden)]
impl<A, B, Op1, C, D, Op2, T> Div<BinopExpr<A, B, T, Op1>> for BinopExpr<C, D, T, Op2>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    Op1: BinOp,
    C: IndexValue<usize, Output = T>,
    D: IndexValue<usize, Output = T>,
    Op2: BinOp,
    T: Field + Copy,
{
    type Output = BinopExpr<BinopExpr<C, D, T, Op2>, BinopExpr<A, B, T, Op1>, T, DivOp>;

    #[inline]
    fn div(
        self,
        rhs: BinopExpr<A, B, T, Op1>,
    ) -> BinopExpr<BinopExpr<C, D, T, Op2>, BinopExpr<A, B, T, Op1>, T, DivOp>
    {
        debug_assert!(self.nrows == rhs.nrows);
        debug_assert!(self.ncols == rhs.ncols);
        let nr = self.nrows;
        let nc = self.ncols;
        BinopExpr {
            a: self,
            b: rhs,
            nrows: nr,
            ncols: nc,
            _marker: std::marker::PhantomData,
        }
    }
}

//}}}
//{{{ impl: Add<T> for BinopExpr
#[doc(hidden)]
impl<A, B, T, Op> Add<T> for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Copy + IndexValue<usize, Output = T>,
    Op: BinOp,
{
    type Output = BinopExpr<Self, T, T, AddOp>;

    #[inline]
    fn add(
        self,
        rhs: T,
    ) -> Self::Output
    {
        let nr = self.nrows;
        let nc = self.ncols;
        BinopExpr {
            a: self,
            b: rhs,
            nrows: nr,
            ncols: nc,
            _marker: std::marker::PhantomData,
        }
    }
}

//}}}
//{{{ impl: Add<BinopExpr> for $type
macro_rules! impl_add_binop_expr {
    ($type:ty) => {
        #[doc(hidden)]
        impl<A, B, Op> Add<BinopExpr<A, B, $type, Op>> for $type
        where
            A: IndexValue<usize, Output = $type>,
            B: IndexValue<usize, Output = $type>,
            Op: BinOp,
        {
            type Output = BinopExpr<Self, BinopExpr<A, B, $type, Op>, $type, AddOp>;

            #[inline]
            fn add(
                self,
                rhs: BinopExpr<A, B, $type, Op>,
            ) -> Self::Output
            {
                let nr = rhs.nrows;
                let nc = rhs.ncols;
                BinopExpr {
                    a: self,
                    b: rhs,
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}

apply_for_all_types!(impl_add_binop_expr);

//}}}
//{{{ impl: Sub<T> for BinopExpr
#[doc(hidden)]
impl<A, B, T, Op> Sub<T> for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Copy + IndexValue<usize, Output = T>,
    Op: BinOp,
{
    type Output = BinopExpr<Self, T, T, SubOp>;

    #[inline]
    fn sub(
        self,
        rhs: T,
    ) -> Self::Output
    {
        let nr = self.nrows;
        let nc = self.ncols;
        BinopExpr {
            a: self,
            b: rhs,
            nrows: nr,
            ncols: nc,
            _marker: std::marker::PhantomData,
        }
    }
}

//}}}
//{{{ impl: Sub<BinopExpr> for $type
macro_rules! impl_sub_binop_expr {
    ($type:ty) => {
        #[doc(hidden)]
        impl<A, B, Op> Sub<BinopExpr<A, B, $type, Op>> for $type
        where
            A: IndexValue<usize, Output = $type>,
            B: IndexValue<usize, Output = $type>,
            Op: BinOp,
        {
            type Output = BinopExpr<Self, BinopExpr<A, B, $type, Op>, $type, SubOp>;

            #[inline]
            fn sub(
                self,
                rhs: BinopExpr<A, B, $type, Op>,
            ) -> Self::Output
            {
                let nr = rhs.nrows;
                let nc = rhs.ncols;
                BinopExpr {
                    a: self,
                    b: rhs,
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}

apply_for_all_types!(impl_sub_binop_expr);

//}}}
//{{{ impl: Mul<T> for BinopExpr
#[doc(hidden)]
impl<A, B, T, Op> Mul<T> for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Copy + IndexValue<usize, Output = T>,
    Op: BinOp,
{
    type Output = BinopExpr<Self, T, T, MulOp>;

    #[inline]
    fn mul(
        self,
        rhs: T,
    ) -> Self::Output
    {
        let nr = self.nrows;
        let nc = self.ncols;
        BinopExpr {
            a: self,
            b: rhs,
            nrows: nr,
            ncols: nc,
            _marker: std::marker::PhantomData,
        }
    }
}

//}}}
//{{{ impl: Mul<BinopExpr> for $type
macro_rules! impl_mul_binop_expr {
    ($type:ty) => {
        #[doc(hidden)]
        impl<A, B, Op> Mul<BinopExpr<A, B, $type, Op>> for $type
        where
            A: IndexValue<usize, Output = $type>,
            B: IndexValue<usize, Output = $type>,
            Op: BinOp,
        {
            type Output = BinopExpr<Self, BinopExpr<A, B, $type, Op>, $type, MulOp>;

            #[inline]
            fn mul(
                self,
                rhs: BinopExpr<A, B, $type, Op>,
            ) -> Self::Output
            {
                let nr = rhs.nrows;
                let nc = rhs.ncols;
                BinopExpr {
                    a: self,
                    b: rhs,
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}

apply_for_all_types!(impl_mul_binop_expr);

//}}}
//{{{ impl: Div<T> for BinopExpr
#[doc(hidden)]
impl<A, B, T, Op> Div<T> for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Copy + IndexValue<usize, Output = T>,
    Op: BinOp,
{
    type Output = BinopExpr<Self, T, T, DivOp>;

    #[inline]
    fn div(
        self,
        rhs: T,
    ) -> Self::Output
    {
        let nr = self.nrows;
        let nc = self.ncols;
        BinopExpr {
            a: self,
            b: rhs,
            nrows: nr,
            ncols: nc,
            _marker: std::marker::PhantomData,
        }
    }
}

//}}}
//{{{ impl: Div<BinopExpr> for $type
macro_rules! impl_div_binop_expr {
    ($type:ty) => {
        #[doc(hidden)]
        impl<A, B, Op> Div<BinopExpr<A, B, $type, Op>> for $type
        where
            A: IndexValue<usize, Output = $type>,
            B: IndexValue<usize, Output = $type>,
            Op: BinOp,
        {
            type Output = BinopExpr<Self, BinopExpr<A, B, $type, Op>, $type, DivOp>;

            #[inline]
            fn div(
                self,
                rhs: BinopExpr<A, B, $type, Op>,
            ) -> Self::Output
            {
                let nr = rhs.nrows;
                let nc = rhs.ncols;
                BinopExpr {
                    a: self,
                    b: rhs,
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}

apply_for_all_types!(impl_div_binop_expr);

//}}}
