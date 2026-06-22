# Welcome to TopoHedral-Linalg

This crate provides a Rust library for small, dense linear algebra. It provides two matrix types:

- a runtime-sized DMatrix
- a compile-time-sized SMatrix

Both use column-major memory layout and can hold floating-point, integral, and boolean data. Both have
the following features defined for them:

- Accelerated matrix-matrix and matrix-vector multiplication via BLAS/LAPACK (floating point only)
- Accelerated matrix decompositions, including Cholesky, and linear system solution via BLAS/LAPACK (floating point only)
- Lazily evaluated elementwise arithmetic and comparison expressions
- Boolean matrices, composable masks, and masked element selection
- Rectangular matrix subviews
- Reductions and transformations
- Elementwise functions which mirror those supported for primitive integral and floating-point
  types.
