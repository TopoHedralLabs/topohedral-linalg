//! Short Description of module
//!
//! Longer description of module

//--------------------------------------------------------------------------------------------------


//{{{ crate imports
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

mod common;
mod elementwise_op;

pub use common::{DMatrixConstructors, SMatrixConstructors};
pub mod matrix_op;
pub mod smatrix;
pub mod dmatrix;
