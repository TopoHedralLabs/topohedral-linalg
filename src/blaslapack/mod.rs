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

/// Abstracts over matrix storage for generic LAPACK dispatch.
#[allow(dead_code)]
pub(crate) trait MatrixBuffer: crate::common::Shape
{
    type Scalar: crate::common::Field + Copy;
    fn as_slice(&self) -> &[Self::Scalar];
    fn as_mut_slice(&mut self) -> &mut [Self::Scalar];
}

pub(crate) use common::AsI32;
pub(crate) use gees::{schur_raw, Error as ShurRawError, Gees};
pub(crate) use geev::{eig_raw, Error as EigRawError, Geev};
pub(crate) use gemm::{matmul_dispatch, Gemm};
pub(crate) use gemv::Gemv;
pub(crate) use geqrf::{qr_raw, Geqrf, QrRawError};
pub(crate) use gesv::{solve_raw, Error as SolveRawError, Gesv};
pub(crate) use getrf::{lu_raw, Error as LuRawError, Getrf};
pub(crate) use orgqr::Orgqr;
pub(crate) use potrf::{cholesky_raw, Error as CholeskyRawError, Potrf};
pub(crate) use syev::{symeig_raw, Error as SymEigRawError, Syev};
