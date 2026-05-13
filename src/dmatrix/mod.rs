//! Dynamic matrix type with heap-allocated, column-major storage.
//!
//! Defines [`DMatrix<T>`], a general-purpose 2-D matrix whose dimensions are determined at
//! runtime and whose elements are stored in a contiguous `Vec<T>` in column-major (Fortran) order.
//! Sub-modules add element-wise arithmetic ([`addop`], [`subop`], [`mulop`], [`divop`], [`negop`]),
//! BLAS-backed matrix multiplication ([`matmul`]), standard linear-algebra decompositions
//! ([`lu`], [`qr`], [`eig`], [`symeig`], [`schur`], [`solve`]), and supporting utilities for
//! construction, indexing, iteration, I/O, sub-matrix views, and reduction/transformation
//! operations.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

// elementwise expressions
mod blaslapack;
mod construction;
mod elementwise;
mod indexing;
mod io;
mod iteration;
mod matrix_ops;
mod objects;
mod reduce_ops;
mod sorting;
mod subviews;
mod transform_ops;

pub use blaslapack::{
    DEigError, DEigReturn, DLuError, DLuReturn, DQrError, DQrReturn, DSchurError, DSchurReturn,
    DSolveError, DSymEigError, DSymEigReturn,
};
pub use objects::{DMatrix, DVector, VecType};
