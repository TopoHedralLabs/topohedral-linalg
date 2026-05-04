# Getting Started

`topohedral-linalg` is a Rust library for small, dense linear algebra. It provides
two matrix types — a compile-time-sized `SMatrix` and a runtime-sized `DMatrix` —
with BLAS/LAPACK acceleration for matrix multiplication and decompositions, and
lazily-evaluated expression templates for elementwise arithmetic.

---

## Installation

The crate will be published to [crates.io](https://crates.io) shortly. Once available,
add it to your project with:

```toml
[dependencies]
topohedral-linalg = "2.0"
```

---

## Feature flags

| Flag | Effect |
|---|---|
| `enable_checks` | Enables runtime dimension checks on operations. Recommended during development, disable for release builds once you are confident in your matrix sizes. |
| `enable_trace` | Enables structured tracing via `topohedral-tracing`. |

```toml
[dependencies]
topohedral-linalg = { version = "2.0", features = ["enable_checks"] }
```

---

## BLAS / LAPACK backends

The library delegates heavy computation to a BLAS/LAPACK backend:

- **macOS** — Apple Accelerate (selected automatically, zero extra setup)
- **Linux** — OpenBLAS (link against `libopenblas-dev` or equivalent)

Matrix multiplication and all decompositions route through BLAS `gemm`/`gemv` and
the corresponding LAPACK routines.

---

## Choosing a matrix type

| | `SMatrix<T, N, M>` | `DMatrix<T>` |
|---|---|---|
| Size | Fixed at compile time | Set at runtime |
| Allocation | Stack (array) | Heap (`Vec`) |
| Copy | Yes (`Copy + Clone`) | Clone only |
| Size checks | Compile-time | Runtime (with `enable_checks`) |
| Best for | Small, known-size matrices | Matrices whose size varies at runtime |

Both types store data in **column-major** (Fortran) order for BLAS compatibility.

---

## Quick example

```rust
use topohedral_linalg::{SMatrix, DMatrix, MatMul, MatrixOps, ReduceOps};

// Static 3×3 identity matrix
let eye = SMatrix::<f64, 3, 3>::identity();

// Dynamic 2×2 matrix built from a row-major slice
let a = DMatrix::<f64>::from_row_slice(&[1.0, 2.0, 3.0, 4.0], 2, 2);
let b = DMatrix::<f64>::from_row_slice(&[5.0, 6.0, 7.0, 8.0], 2, 2);

// Elementwise addition (lazy expression, evaluated on .into())
let c: DMatrix<f64> = (&a + &b).into();

// BLAS matrix-matrix multiplication
let d = (&a).matmul(&b);

// Reduction
let total = c.sum();
```
