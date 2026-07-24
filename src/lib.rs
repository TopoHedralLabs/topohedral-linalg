//! # Topohedral-Linalg
//!
//! This crate provides a Rust library for small, dense linear algebra. It provides
//! two matrix types:
//!
//! - a runtime-sized `DMatrix`
//! - a compile-time-sized `SMatrix`
//!
//! Both use column-major memory layout and can hold floating-point, integral, and boolean data.
//! Both have the following features defined for them:
//!
//! - Accelerated matrix-matrix and matrix-vector multiplication via BLAS/LAPACK (floating point only)
//! - Accelerated matrix decompositions and linear system solution via LAPACK (floating point only)
//! - Lazily evaluated elementwise expressions.
//! - Matrix subviews
//! - Lazy element-wise comparisons and boolean masked selection
//! - Reductions and transformations
//! - Elementwise functions which mirror those supported for primitive integral and floating-point
//!   types.
//!
//! Matrix storage is column-major. Constructors named `from_row_*` convert row-major input, while
//! constructors named `from_col_*`, slice conversions, and iterators use column-major order.
//!
//! ## Basic use
//!
//! ```
//! use topohedral_linalg::{DMatrix, MatMul, SMatrix, SquareMatrixOps};
//!
//! let dynamic = DMatrix::<f64>::from_row_slice(&[1.0, 2.0, 3.0, 4.0], 2, 2);
//! let fixed = SMatrix::<f64, 2, 2>::identity();
//! let product = (&dynamic).matmul(&fixed);
//!
//! assert_eq!(product.trace(), 5.0);
//! ```
//!
//! ## Views, serialization, and decomposition
//!
//! ```
//! use topohedral_linalg::{DMatrix, SubViewable};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let matrix = DMatrix::<f64>::from_row_slice(&[4.0, 1.0, 1.0, 3.0], 2, 2);
//! let view = matrix.subview_range(0, 1, 0, 0);
//! assert_eq!(view.iter().copied().collect::<Vec<_>>(), [4.0, 1.0]);
//!
//! let encoded = serde_json::to_string(&matrix)?;
//! let decoded: DMatrix<f64> = serde_json::from_str(&encoded)?;
//! let cholesky = decoded.cholesky()?;
//! assert_eq!(cholesky.l[(0, 0)], 2.0);
//! # Ok(())
//! # }
//! ```
//!
//! For guides, tutorials, examples and developer documentation, see the
//! [TopoHedral-Linalg documentation site](https://topohedrallabs.github.io/topohedral-linalg/).
#![warn(missing_docs)]
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------
//{{{ collection: private modules
mod blaslapack;
mod common;
mod dmatrix;
#[allow(missing_docs)]
mod expression;
mod float;
mod smatrix;
mod subviews;
//}}}
pub(crate) use common::{apply_for_all_integer_types, apply_for_all_types};
//{{{ collection: public API
pub use crate::float::{Float, FloatTransformOps, FloatVectorOps};
pub use blaslapack::{BlasScalar, LapackScalar};
pub use common::{
    Abs, Complex, Dimension, Field, MatMul, MatrixElementDisplay, MatrixExpr, MatrixOps, One,
    ReduceOps, Shape, SquareMatrixOps, TransformOps, UniformRandom, VectorOps, Zero,
};
pub use dmatrix::*;
pub use expression::comparison_expr::ElementwiseCompare;
pub use expression::outer_product_expr::OuterProduct;
pub use expression::unary_expr::{
    abs, abs_sub, acos, acosh, asin, asinh, atan, atan2, atanh, cbrt, ceil, clamp, copysign, cos,
    cosh, div_euclid, exp, exp2, exp_m1, floor, fract, hypot, ln, ln_1p, log, log10, log2, max,
    midpoint, min, mul_add, next_down, next_up, powf, powi, recip, rem_euclid, round,
    round_ties_even, signum, sin, sinh, sqrt, tan, tanh, to_degrees, to_radians, trunc,
};
pub use smatrix::*;
pub use subviews::{
    IndexedMatrixView, IndexedMatrixViewIter, IndexedMatrixViewMut, IndexedMatrixViewMutIter,
    IndexedMatrixViewMutIterMut, Maskable, MaskedView, MaskedViewIter, MatrixView, MatrixViewIter,
    MatrixViewMut, MatrixViewMutIter, MatrixViewMutIterMut, SubViewable, SubViewableMut,
};
//}}}
