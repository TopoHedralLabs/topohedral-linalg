//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
//}}}
//{{{ std imports 
use::std::ops::{Add, Sub, Mul, Div};
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

pub trait Field: Sized + Add + Sub + Mul + Div  {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn div(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
}

macro_rules! impl_field {
    ($type:ty) => {
        impl Field for $type {
            fn add(&self, other: &Self) -> Self {
                self + other
            }
            
            fn sub(&self, other: &Self) -> Self {
                self - other
            }
            
            fn div(&self, other: &Self) -> Self {
                self / other
            }
            
            fn mul(&self, other: &Self) -> Self {
                self * other
            }
        }
    };
}


impl_field!(f32);
impl_field!(f64);
impl_field!(i8);
impl_field!(i16);
impl_field!(i32);
impl_field!(i64);
impl_field!(i128);
impl_field!(u8);
impl_field!(u16);
impl_field!(u32);
impl_field!(u64);
impl_field!(u128);





pub trait Expression {
    type Output;
    fn eval(&self) -> Self::Output;  
}


pub struct AddExpr<Lhs, Rhs> 
where 
    Lhs: Expression, Rhs: Expression
{

    lhs: Lhs,
    rhs: Rhs,   
}

impl<Lhs, Rhs> Expression for AddExpr<Lhs, Rhs> 
where 
    Lhs: Expression, Rhs: Expression
{
    type Output = Lhs::Output;    

    fn eval(&self) -> Self::Output {
        let lhs = self.lhs.eval();
        let rhs = self.rhs.eval();

    }
}



