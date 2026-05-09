//! # Welcome to TopoHedral-Linalg!
//!
//! This crate provides a Rust library for small, dense linear algebra. It provides
//! two matrix types:
//!
//! - a runtime-sized `DMatrix`
//! - a compile-time-sized `SMatrix`
//!
//! Both use column-major memory layout and can hold both floating point and integral data.
//! Both have the following features defined for them:
//!
//! - Accelerated matrix-matrix and matrix-mector multiplication via BLAS/LAPACK (floating point only)
//! - Accelerated Matrix decompositions and linear system solution via BLAS/LAPACK (floating point only)
//! - Lazily-evaluated, complex elementwide expressions.
//! - Matrix subviews
//! - Reductions and transformations
//! - Elementwise functions which mirror those supported for primitive integral and floating point
//!   types.
//!
//! For guides, tutorials, examples and developer documentation, see the
//! [TopoHedral-Linalg documentation site](../../).
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
pub use expression::unary_expr::{
    abs, abs_sub, acos, acosh, algebraic_add, algebraic_div, algebraic_mul, algebraic_rem,
    algebraic_sub, asin, asinh, atan, atan2, atanh, cbrt, ceil, clamp, clamp_magnitude, copysign,
    cos, cosh, div_euclid, erf, erfc, exp, exp2, exp_m1, floor, fract, gamma, hypot, ln, ln_1p,
    log, log10, log2, max, maximum, midpoint, min, minimum, mul_add, next_down, next_up, powf,
    powi, recip, rem_euclid, round, round_ties_even, signum, sin, sinh, sqrt, tan, tanh,
    to_degrees, to_radians, trunc,
};
pub mod dmatrix;
pub mod dvector;
pub mod scvector;
pub mod smatrix;
pub mod srvector;
//}}}
