//! Common traits and functions for blas/lapack operations
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------


//{{{ trait: AsI32
pub trait AsI32 {
    fn as_i32(&self) -> i32;
}
impl AsI32 for f32 {
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}
impl AsI32 for f64 {
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}
//}}}