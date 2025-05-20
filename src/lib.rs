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
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![feature(impl_trait_in_assoc_type)]

//{{{ collection: private modules
mod blaslapack;
mod common;
mod expression;
//}}}
//{{{ collection: public API
pub use common::{Complex, FloatVectorOps, GreaterThan, MatMul, MatrixOps, VectorOps};
pub mod dmatrix;
pub mod dvector;
pub mod scvector;
pub mod smatrix;
pub mod srvector;
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{}

//}}}
