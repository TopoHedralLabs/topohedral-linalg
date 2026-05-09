# Welcome to TopoHedral-Linalg

This crate provides a Rust library for small, dense linear algebra. It provides two matrix types:

- a runtime-sized DMatrix
- a compile-time-sized SMatrix

Both use column-major memory layout and can hold both floating point and integral data. Both have
the following features defined for them:

- Accelerated matrix-matrix and matrix-mector multiplication via BLAS/LAPACK (floating point only)
- Accelerated Matrix decompositions and linear system solution via BLAS/LAPACK (floating point only)
- Lazily-evaluated, complex elementwide expressions.
- Matrix subviews
- Reductions and transformations
- Elementwise functions which mirror those supported for primitive integral and floating point
  types.