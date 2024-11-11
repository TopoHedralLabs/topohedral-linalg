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

pub trait Field: Sized + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn div(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
}

macro_rules! impl_field {
    ($type:ty) => {
        impl Field for $type {
            #[inline]
            fn add(&self, other: &Self) -> Self {
                self + other
            }
            
            #[inline]
            fn sub(&self, other: &Self) -> Self {
                self - other
            }
            
            #[inline]
            fn div(&self, other: &Self) -> Self {
                self / other
            }
            
            #[inline]
            fn mul(&self, other: &Self) -> Self {
                self * other
            }
        }

        impl IndexValue<usize> for $type {
            type Output = Self;

            #[inline]
            fn index_value(&self, index: usize) -> Self::Output {
                *self
            }
        }
    };
}

pub trait IndexValue<I> {

    type Output;
    fn index_value(&self, index: usize) -> Self::Output;
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


