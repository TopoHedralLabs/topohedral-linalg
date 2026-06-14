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

mod cholesky;
mod eig;
mod lu;
mod matmul;
mod qr;
mod schur;
mod solve;
mod symeig;

pub use cholesky::{Error as SCholeskyError, Return as SCholeskyReturn};
pub use eig::{Error as SEigError, Return as SEigReturn};
pub use lu::{Error as SLuError, Return as SLuReturn};
pub use qr::{Error as SQrError, Return as SQrReturn};
pub use schur::{Error as SSchurError, Return as SSchurReturn};
pub use solve::Error as SSolveError;
pub use symeig::{Error as SSymEigError, Return as SSymEigReturn};
