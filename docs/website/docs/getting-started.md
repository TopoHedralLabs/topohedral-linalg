# Getting Started

## Installation

The crate will be published to [crates.io](https://crates.io) shortly. Once available,
add it to your project with:

```toml
[dependencies]
topohedral-linalg = "3.0"
```

---

## Feature flags

| Flag | Effect |
|---|---|
| `enable_checks` | Enables runtime dimension checks on operations. Recommended during development, disable for release builds once you are confident in your matrix sizes. |
| `enable_trace` | Enables structured tracing via `topohedral-tracing`. |

```toml
[dependencies]
topohedral-linalg = { version = "3.0", features = ["enable_checks"] }
```

---

## BLAS / LAPACK backends

The library delegates heavy computation to a BLAS/LAPACK backend:

- **macOS** — Apple Accelerate (selected automatically, zero extra setup)
- **Linux** — OpenBLAS (link against `libopenblas-dev` or equivalent)

Matrix multiplication and decompositions such as LU, QR, Cholesky, eigendecomposition,
and Schur factorization route through BLAS `gemm`/`gemv` and the corresponding LAPACK
routines.

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
They can store numeric values or booleans; arithmetic and BLAS/LAPACK methods remain
restricted to their appropriate numeric scalar types.

---

## Quick example

```rust
use topohedral_linalg::{
    DMatrix, DVector, SMatrix, VecType,
    ElementwiseCompare, Maskable, MatMul, MatrixOps, ReduceOps,
    OuterProduct,
};

// Static 3×3 identity matrix
let eye = SMatrix::<f64, 3, 3>::identity();

// Dynamic 2×2 matrix built from a row-major slice
let a = DMatrix::<f64>::from_row_slice(&[1.0, 2.0, 3.0, 4.0], 2, 2);
let b = DMatrix::<f64>::from_row_slice(&[5.0, 6.0, 7.0, 8.0], 2, 2);

// Elementwise addition (lazy expression, evaluated on .into())
let c: DMatrix<f64> = (&a + &b).into();

// Lazy vector outer products can be used directly in elementwise expressions
let v = DVector::<f64>::from_slice_vec(&[1.0, 2.0], 2, VecType::Col);
let w = DVector::<f64>::from_slice_vec(&[10.0, 20.0], 2, VecType::Col);
let rank_one_update: DMatrix<f64> = (&a + v.outer(&w)).into();

// BLAS matrix-matrix multiplication
let d = (&a).matmul(&b);

// Reduction
let total = c.sum();

// Lazy comparison and boolean masked selection
let selected: DMatrix<f64> = a.masked(a.gt(2.0)).to_dmatrix();
```
