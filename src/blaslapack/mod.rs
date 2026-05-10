//! Thin, type-safe wrappers around BLAS Level 2/3 and LAPACK routines.
//!
//! Groups all raw BLAS and LAPACK bindings used by the crate into named sub-modules, each
//! corresponding to a single subroutine family. The wrappers expose Rust traits rather than raw
//! `unsafe` function pointers, so callers work with typed inputs and outputs and `unsafe` blocks
//! are confined to the implementation files. Sub-modules: [`common`] (shared types), [`gemm`]
//! (matrix–matrix multiply), [`gemv`] (matrix–vector multiply), [`getrf`] (LU), [`geqrf`] +
//! [`orgqr`] (QR), [`geev`] (general eigenvalues), [`syev`] (symmetric eigenvalues), [`gees`]
//! (Schur), [`gesv`] (linear solve).
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

pub mod common;
pub mod gees;
pub mod geev;
pub mod gemm;
pub mod gemv;
pub mod geqrf;
pub mod gesv;
pub mod getrf;
pub mod orgqr;
pub mod syev;

/// Abstracts over matrix storage for generic LAPACK dispatch.
#[allow(dead_code)]
pub(crate) trait MatrixBuffer: crate::common::Shape
{
    type Scalar: crate::common::Field + Copy;
    fn as_slice(&self) -> &[Self::Scalar];
    fn as_mut_slice(&mut self) -> &mut [Self::Scalar];
}
