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

//{{{ trait: Field
pub trait Field: Sized + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn div(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
}
//}}}
//{{{ trait: IndexValue
pub trait IndexValue<I> {

    type Output;
    fn index_value(&self, index: usize) -> Self::Output;
}
//}}}
//{{{ macro: apply_for_all_types
#[macro_export]
macro_rules! apply_for_all_types {
    ($macro:ident) => {
        $macro!(f32);
        $macro!(f64);
        $macro!(i8);
        $macro!(i16);
        $macro!(i32);
        $macro!(i64);
        $macro!(i128);
        $macro!(u8);
        $macro!(u16);
        $macro!(u32);
        $macro!(u64);
        $macro!(u128);
    }

}
//}}}
//{{{ macro: impl_field
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
//}}}
//{{{ collection: impl_field implementations
apply_for_all_types!(impl_field);
//}}}


