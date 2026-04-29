//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::apply_for_all_types;
use crate::common::{Field, IndexValue, LazyExpr, Shape};
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
//{{{ impl: Shape for BinopExpr
#[doc(hidden)]
impl<A, B, T, Op> Shape for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Copy,
    Op: BinOp,
{
    #[inline]
    fn nrows(&self) -> usize
    {
        self.nrows
    }

    #[inline]
    fn ncols(&self) -> usize
    {
        self.ncols
    }
}

//}}}
//{{{ impl: LazyExpr for BinopExpr
impl<A, B, T, Op> LazyExpr for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Copy,
    Op: BinOp,
{
    type ScalarType = T;
}

//}}}
//{{{ macro: impl_binop_expr_binary_op
macro_rules! impl_binop_expr_binary_op {
    ($trait:ident, $method:ident, $op:ty) => {
        #[doc(hidden)]
        impl<A, B, T, Op, Rhs> $trait<Rhs> for BinopExpr<A, B, T, Op>
        where
            A: IndexValue<usize, Output = T>,
            B: IndexValue<usize, Output = T>,
            T: Field + Copy,
            Op: BinOp,
            Rhs: LazyExpr<ScalarType = T> + IndexValue<usize, Output = T>,
        {
            type Output = BinopExpr<Self, Rhs, T, $op>;

            #[inline]
            fn $method(
                self,
                rhs: Rhs,
            ) -> Self::Output
            {
                debug_assert!(self.nrows == rhs.nrows());
                debug_assert!(self.ncols == rhs.ncols());
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
    };
}

impl_binop_expr_binary_op!(Add, add, AddOp);
impl_binop_expr_binary_op!(Sub, sub, SubOp);
impl_binop_expr_binary_op!(Mul, mul, MulOp);
impl_binop_expr_binary_op!(Div, div, DivOp);

//}}}
//{{{ impl: Add<$type> for BinopExpr
macro_rules! impl_add_binop_expr_scalar_rhs {
    ($type:ty) => {
        #[doc(hidden)]
        impl<A, B, Op> Add<$type> for BinopExpr<A, B, $type, Op>
        where
            A: IndexValue<usize, Output = $type>,
            B: IndexValue<usize, Output = $type>,
            Op: BinOp,
        {
            type Output = BinopExpr<Self, $type, $type, AddOp>;

            #[inline]
            fn add(
                self,
                rhs: $type,
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
    };
}

apply_for_all_types!(impl_add_binop_expr_scalar_rhs);

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
//{{{ impl: Sub<$type> for BinopExpr
macro_rules! impl_sub_binop_expr_scalar_rhs {
    ($type:ty) => {
        #[doc(hidden)]
        impl<A, B, Op> Sub<$type> for BinopExpr<A, B, $type, Op>
        where
            A: IndexValue<usize, Output = $type>,
            B: IndexValue<usize, Output = $type>,
            Op: BinOp,
        {
            type Output = BinopExpr<Self, $type, $type, SubOp>;

            #[inline]
            fn sub(
                self,
                rhs: $type,
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
    };
}

apply_for_all_types!(impl_sub_binop_expr_scalar_rhs);

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
//{{{ impl: Mul<$type> for BinopExpr
macro_rules! impl_mul_binop_expr_scalar_rhs {
    ($type:ty) => {
        #[doc(hidden)]
        impl<A, B, Op> Mul<$type> for BinopExpr<A, B, $type, Op>
        where
            A: IndexValue<usize, Output = $type>,
            B: IndexValue<usize, Output = $type>,
            Op: BinOp,
        {
            type Output = BinopExpr<Self, $type, $type, MulOp>;

            #[inline]
            fn mul(
                self,
                rhs: $type,
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
    };
}

apply_for_all_types!(impl_mul_binop_expr_scalar_rhs);

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
//{{{ impl: Div<$type> for BinopExpr
macro_rules! impl_div_binop_expr_scalar_rhs {
    ($type:ty) => {
        #[doc(hidden)]
        impl<A, B, Op> Div<$type> for BinopExpr<A, B, $type, Op>
        where
            A: IndexValue<usize, Output = $type>,
            B: IndexValue<usize, Output = $type>,
            Op: BinOp,
        {
            type Output = BinopExpr<Self, $type, $type, DivOp>;

            #[inline]
            fn div(
                self,
                rhs: $type,
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
    };
}

apply_for_all_types!(impl_div_binop_expr_scalar_rhs);

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
