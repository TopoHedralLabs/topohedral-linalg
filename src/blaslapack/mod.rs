//! Thin, type-safe wrappers around BLAS Level 2/3 and LAPACK routines.
//!
//! Groups all raw BLAS and LAPACK bindings used by the crate into named sub-modules, each
//! corresponding to a single subroutine family. The wrappers expose Rust traits rather than raw
//! `unsafe` function pointers, so callers work with typed inputs and outputs and `unsafe` blocks
//! are confined to the implementation files. Sub-modules: [`common`] (shared types), [`gemm`]
//! (matrix–matrix multiply), [`gemv`] (matrix–vector multiply), [`getrf`] (LU), [`potrf`]
//! (Cholesky), [`geqrf`] + [`orgqr`] (QR), [`geev`] (general eigenvalues), [`syev`] (symmetric
//! eigenvalues), [`gees`] (Schur), [`gesv`] (linear solve).
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

mod common;
mod gees;
mod geev;
mod gemm;
mod gemv;
mod geqrf;
mod gesv;
mod getrf;
mod orgqr;
mod potrf;
mod syev;

mod sealed {
    pub trait BlasScalar: super::gemm::Gemm + super::gemv::Gemv {}

    impl BlasScalar for f32 {}
    impl BlasScalar for f64 {}
    impl BlasScalar for i8 {}
    impl BlasScalar for i16 {}
    impl BlasScalar for i32 {}
    impl BlasScalar for i64 {}
    impl BlasScalar for i128 {}

    pub trait LapackScalar:
        super::common::AsI32
        + super::gees::Gees
        + super::geev::Geev
        + super::gemm::Gemm
        + super::gemv::Gemv
        + super::geqrf::Geqrf
        + super::gesv::Gesv
        + super::getrf::Getrf
        + super::orgqr::Orgqr
        + super::potrf::Potrf
        + super::syev::Syev
    {
    }

    impl LapackScalar for f32 {}
    impl LapackScalar for f64 {}
}

/// Scalar types supported by matrix multiplication.
///
/// Floating-point implementations dispatch to BLAS. Signed integer implementations use the
/// crate's column-major fallback kernel.
pub trait BlasScalar:
    sealed::BlasScalar + crate::common::Field + crate::common::Zero + crate::common::One + Copy
{
}

impl BlasScalar for f32 {}
impl BlasScalar for f64 {}
impl BlasScalar for i8 {}
impl BlasScalar for i16 {}
impl BlasScalar for i32 {}
impl BlasScalar for i64 {}
impl BlasScalar for i128 {}

/// Scalar types supported by the crate's BLAS/LAPACK backend.
///
/// This trait is sealed because adding an implementation requires matching native BLAS and
/// LAPACK entry points. It is currently implemented for [`f32`] and [`f64`].
pub trait LapackScalar:
    sealed::LapackScalar
    + crate::common::Field
    + crate::common::Zero
    + crate::common::One
    + Default
    + Copy
{
}

impl LapackScalar for f32 {}
impl LapackScalar for f64 {}

/// Abstracts over matrix storage for generic LAPACK dispatch.
#[allow(dead_code)]
pub(crate) trait MatrixBuffer: crate::common::Shape {
    type Scalar: crate::common::Field + Copy;
    fn as_slice(&self) -> &[Self::Scalar];
    fn as_mut_slice(&mut self) -> &mut [Self::Scalar];
}

pub(crate) use gees::{schur_raw, Error as ShurRawError};
pub(crate) use geev::{eig_raw, Error as EigRawError};
pub(crate) use gemm::matmul_dispatch;
pub(crate) use geqrf::{qr_raw, QrRawError};
pub(crate) use gesv::{solve_raw, Error as SolveRawError};
pub(crate) use getrf::{lu_raw, Error as LuRawError};
pub(crate) use potrf::{cholesky_raw, Error as CholeskyRawError};
pub(crate) use syev::{symeig_raw, Error as SymEigRawError};
