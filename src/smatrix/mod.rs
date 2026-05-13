//! Static matrix type with compile-time dimensions and stack-allocated storage.
//!
//! Defines [`SMatrix<T, N, M>`], a 2-D matrix whose row count N and column count M are
//! const-generic parameters baked into the type. Elements are stored in a fixed-size array
//! `[T; N*M]` in column-major order, enabling `Copy` semantics and stack allocation for small
//! matrices. Sub-modules add element-wise arithmetic, BLAS-backed matrix multiplication, standard
//! decompositions (LU, QR, eigenvalue, Schur, linear solve), and supporting utilities for
//! construction, indexing, iteration, serialisation, sub-views, and reduction/transformation
//! operations.
//--------------------------------------------------------------------------------------------------

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
    SEigError, SEigReturn, SLuError, SLuReturn, SQrError, SQrReturn, SSchurError, SSchurReturn,
    SSolveError, SSymEigError, SSymEigReturn,
};
pub use objects::{SCVector, SMatrix, SRVector};
