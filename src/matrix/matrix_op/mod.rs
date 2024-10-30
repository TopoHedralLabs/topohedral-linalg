//! This module contains the matrix operations.
//!
//! Operations include:
//!
//! - Matrix multiplication [[`matmul`]]
//! - Matrix decomposition 
//!     - LU decomposition [[`lu`]]
//!     - QR decompositoin [[`qr`]]
//!     - Schur decomposition [[`schur`]]
//!     - Eigenvalue decomposition [[`eig`]]
//! - Linear system solver  

//--------------------------------------------------------------------------------------------------

//{{{ crate imports
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

mod common;
pub mod matmul;
pub mod lu;
pub mod qr;
pub mod schur;
pub mod solve;
pub mod eig;
