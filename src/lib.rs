//! # Welcome to Topohedral-Linalg!
//!
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------
#![feature(clamp_magnitude)]
#![feature(float_algebraic)]
#![feature(float_erf)]
#![feature(float_gamma)]
#![feature(float_minimum_maximum)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

//{{{ collection: private modules
mod blaslapack;
mod common;
mod expression;
//}}}
//{{{ collection: public API
pub use common::{
    Abs, Complex, Dimension, Float, FloatTransformOps, FloatVectorOps, GreaterThan, MatMul,
    MatrixOps, ReduceOps, Shape, TransformOps, VectorOps,
};
pub mod dmatrix;
pub mod dvector;
pub mod scvector;
pub mod smatrix;
pub mod srvector;
//}}}
