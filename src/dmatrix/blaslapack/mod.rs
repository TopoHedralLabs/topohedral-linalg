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


mod eig;
mod lu;
mod matmul;
mod qr;
mod schur;
mod solve;
mod symeig;

pub use eig::{Error as DEigError, Return as DEigReturn};
pub use lu::{Error as DLuError, Return as DLuReturn};
pub use qr::{Error as DQrError, Return as DQrReturn};
pub use schur::{Error as DSchurError, Return as DSchurReturn};
pub use solve::{Error as DSolveError};
pub use symeig::{Error as DSymEigError, Return as SymEigReturn};
