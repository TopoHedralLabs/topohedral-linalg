# User Guide

This page covers the full surface area of `topohedral-linalg`, with examples for
both `SMatrix` (fixed size) and `DMatrix` (dynamic size).

## Imports

There is no prelude — import what you need. The module structure is:

| What you need | Import path |
|---|---|
| `SMatrix` | `topohedral_linalg::smatrix::SMatrix` |
| `DMatrix` | `topohedral_linalg::dmatrix::DMatrix` |
| Traits (`MatMul`, `MatrixOps`, `ReduceOps`, `TransformOps`, `FloatTransformOps`, `Shape`, `VectorOps`, …) | `topohedral_linalg::{TraitName}` |
| `Dimension` (for sorting) | `topohedral_linalg::Dimension` |
| Lazy unary functions (`sin`, `cos`, `sqrt`, …) | `topohedral_linalg::{fn_name}` |

A typical set of imports for general use:

```rust
use topohedral_linalg::smatrix::SMatrix;
use topohedral_linalg::dmatrix::DMatrix;
use topohedral_linalg::{
    Dimension, MatMul, MatrixOps, ReduceOps, Shape,
    TransformOps, FloatTransformOps,
};
```

Add individual trait imports only when the methods they provide are actually needed;
the Rust compiler will tell you which trait to bring into scope if a method call
fails to resolve.

---

## Creating matrices

### Zeros and ones

```rust
// SMatrix
let z = SMatrix::<f64, 3, 4>::zeros();
let o = SMatrix::<f64, 3, 4>::ones();

// DMatrix
let z = DMatrix::<f64>::zeros(3, 4);
let o = DMatrix::<f64>::ones(3, 4);
```

### Constant fill

```rust
let m = SMatrix::<f64, 2, 2>::from_value(3.14);
let m = DMatrix::<f64>::from_value(3.14, 2, 2);
```

### Identity

```rust
let eye = SMatrix::<f64, 4, 4>::identity();
let eye = DMatrix::<f64>::identity(4, 4);
```

### From a slice

Both constructors accept flat slices and a layout flag. The slice length must equal
`nrows * ncols`.

```rust
// Row-major input (most natural when writing data inline)
let a = SMatrix::<f64, 2, 3>::from_row_slice(&[
    1.0, 2.0, 3.0, // row 0
    4.0, 5.0, 6.0, // row 1
]);

// Column-major input (matches internal storage)
let a = SMatrix::<f64, 2, 3>::from_col_slice(&[
    1.0, 4.0,   // column 0
    2.0, 5.0,   // column 1
    3.0, 6.0,   // column 2
]);

// DMatrix equivalents pass dimensions explicitly
let a = DMatrix::<f64>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3);
let a = DMatrix::<f64>::from_col_slice(&[1.0, 4.0, 2.0, 5.0, 3.0, 6.0], 2, 3);
```

### Random

```rust
let r = SMatrix::<f64, 3, 3>::from_uniform_random(0.0, 1.0);
let r = DMatrix::<f64>::from_uniform_random(0.0, 1.0, 3, 3);
```

---

## Indexing and element access

Both types use **(row, col)** tuple indexing (0-based). A flat linear index is also
accepted; the mapping is column-major: `linear = row + col * nrows`.

```rust
let mut m = DMatrix::<f64>::zeros(3, 3);

// Read
let v = m[(1, 2)];       // row 1, col 2
let v = m[4];             // linear index 4

// Write
m[(0, 0)] = 1.0;
m[4] = 99.0;
```

---

## Elementwise arithmetic

### Binary operators

All four operators (`+`, `-`, `*`, `/`) are supported between two matrices of
identical dimensions, and between a matrix and a scalar.

```rust
let a = DMatrix::<f64>::ones(2, 2);
let b = DMatrix::<f64>::from_value(2.0, 2, 2);

let c = a.clone() + b.clone();   // elementwise add
let c = a.clone() - b.clone();   // elementwise subtract
let c = a.clone() * b.clone();   // elementwise multiply
let c = a.clone() / b.clone();   // elementwise divide

let c = a.clone() + 5.0;         // scalar add
let c = 3.0 * a.clone();         // scalar multiply
```

### Lazy expressions (expression templates)

When operands are taken **by reference**, no computation happens immediately —
instead a zero-cost expression tree is built. The actual evaluation is deferred
until you convert the expression to a concrete matrix:

```rust
let a = DMatrix::<f64>::from_uniform_random(0.0, 1.0, 100, 100);
let b = DMatrix::<f64>::from_uniform_random(0.0, 1.0, 100, 100);
let c = DMatrix::<f64>::from_uniform_random(0.0, 1.0, 100, 100);

// Builds a tree — no allocations yet
let expr = &a + &b + &c;

// Single pass, single allocation
let result: DMatrix<f64> = expr.into();
```

Chains of any length collapse into one vectorised loop; see the
[developer notes](developers/elementwise-vectorization.md) for the full analysis.

### Negation

```rust
let neg = -a.clone();               // owned negation
let neg: DMatrix<f64> = (-&a).into(); // lazy negation via UnaryExpr
```

---

## Matrix multiplication

Matrix multiplication uses the `.matmul()` method (not the `*` operator, which is
elementwise). Both operands are taken by reference; the library dispatches to
BLAS `gemm` for general matrices and `gemv` for matrix–vector products.

```rust
use topohedral_linalg::MatMul;

let a = SMatrix::<f64, 3, 4>::from_uniform_random(0.0, 1.0);
let b = SMatrix::<f64, 4, 5>::from_uniform_random(0.0, 1.0);
let c: SMatrix<f64, 3, 5> = (&a).matmul(&b);

// DMatrix — dimensions checked at runtime
let a = DMatrix::<f64>::from_uniform_random(0.0, 1.0, 50, 60);
let b = DMatrix::<f64>::from_uniform_random(0.0, 1.0, 60, 40);
let c = (&a).matmul(&b);  // 50×40 result
```

For `SMatrix` the output dimensions are inferred at compile time from the generic
parameters, so a dimension mismatch is a compile error.

---

## Elementwise transformations

### Scalar operations

Every transformation is available in three flavours:

| Flavour | Example | Effect |
|---|---|---|
| In-place | `m.transform()` | Modifies `m` |
| Copying | `m.transformed()` | Returns new matrix, `m` unchanged |
| Consuming | `m.into_transformed()` | Consumes `m`, returns new matrix |

these are the three general purpose transformations which take a closure of type
`FnMut(S) -> S` where `S` is the scalar type of the matrix. In additions to these
general-purpose transformations we provide three standard transformations: `shift`,
`scale` and `fill` for convenience. For example:

```rust
use topohedral_linalg::TransformOps;
let mut m = DMatrix::<f64>::ones(3, 3);
m.shift(2.0);    // add 2.0 to every element in-place
let m2 = m.shifted(2.0);    // Copies m, adds 2.0 to every element, m unchanged
let m3 = m.into_shifted(2.0); // Consumes m, adds 2.0 to every element and returns new matrix
```

### Built-in math functions

Every standard floating-point function is available in three flavours, using `sqrt` as an
example:

| Flavour | Example | Effect |
|---|---|---|
| In-place | `m.sqrt()` | Modifies `m` |
| Copying | `m.sqrted()` | Returns new matrix, `m` unchanged |
| Consuming | `m.into_sqrted()` | Consumes `m`, returns new matrix |

```rust
use topohedral_linalg::FloatTransformOps;

let mut m = DMatrix::<f64>::from_uniform_random(0.0, 1.0, 4, 4);

m.abs();
m.exp();
m.ln();
m.sin();
m.cos();
m.sqrt();
m.powf(2.0);   // raise every element to power 2.0
m.clamp(0.0, 1.0);

let m2 = m.sqrted();   // non-destructive
let m3 = m.into_exped(); // consuming
```

The full list mirrors Rust's `f64` intrinsics: `abs`, `acos`, `acosh`, `asin`,
`asinh`, `atan`, `atan2`, `atanh`, `cbrt`, `ceil`, `clamp`, `cos`, `cosh`, `exp`,
`exp2`, `exp_m1`, `floor`, `fract`, `gamma`, `hypot`, `ln`, `ln_1p`, `log`,
`log10`, `log2`, `mul_add`, `powf`, `powi`, `recip`, `round`, `signum`, `sin`,
`sinh`, `sqrt`, `tan`, `tanh`, `to_degrees`, `to_radians`, `trunc`, and more.

---

## Reduction operations

```rust
use topohedral_linalg::ReduceOps;

let m = DMatrix::<f64>::from_row_slice(&[3.0, 1.0, 4.0, 1.0], 2, 2);

let s  = m.sum();            // sum of all elements
let p  = m.product();        // product of all elements
let lo = m.min();            // smallest element (Option)
let hi = m.max();            // largest element (Option)

// Absolute-value extrema (no sign)
let alo = m.abs_min();
let ahi = m.abs_max();

// Index of extremum: returns Option<(index, value)>
// For DMatrix the index is (row, col); for SMatrix the same.
let (idx, val) = m.argmin().unwrap();
let (idx, val) = m.argmax().unwrap();

// Key-based extrema
let lo = m.min_by_key(|x| (x * 100.0) as i64);
let hi = m.max_by_key(|x| (x * 100.0) as i64);

// Generic fold
let count_positive = m.fold(0usize, |acc, x| if x > 0.0 { acc + 1 } else { acc });

// Fold with position
m.fold_indexed(0.0f64, |acc, (r, c), x| {
    println!("({r},{c}) = {x}");
    acc + x
});
```

---

## Core matrix operations

```rust
use topohedral_linalg::MatrixOps;

let m = DMatrix::<f64>::from_row_slice(&[1.0, 2.0, 3.0, 4.0], 2, 2);

let t   = m.transpose();     // returns a new matrix
let det = m.determinant();   // f64, square matrices only
let tr  = m.trace();         // sum of diagonal
```

Dimension queries are available on all matrix and vector types:

```rust
use topohedral_linalg::Shape;

let (rows, cols) = m.size();
let r = m.nrows();
let c = m.ncols();
```

---

## Sorting

Sort elements along rows, columns, or the full matrix (treating it as a flat array).

```rust
use topohedral_linalg::Dimension;

let mut m = DMatrix::<f64>::from_row_slice(&[3.0, 1.0, 4.0, 1.0, 5.0, 9.0], 2, 3);

m.sort(Dimension::Rows);           // sort within each row
m.sort(Dimension::Cols);           // sort within each column
m.sort(Dimension::All);            // sort all elements globally

let m2 = m.sorted(Dimension::All);        // returns a sorted copy
let m3 = m.into_sorted(Dimension::Cols);  // consumes m, returns sorted copy
```

---

## Matrix decompositions

All decompositions are methods on the matrix and return `Result` structs.
They are backed by LAPACK routines.

### LU decomposition

Performs the LU decomposition with partial pivoting, which for matrix $\mathbf{A}$
computes the permutation matrix $\mathbf{P}$, the lower-triangular matrix $\mathbf{L}$,
and the upper-triangular matrix $\mathbf{U}$ such that: $\mathbf{P}\mathbf{A} = \mathbf{L}\mathbf{U}$.

```rust
let m = DMatrix::<f64>::from_row_slice(&[2.0, 1.0, 1.0, 3.0], 2, 2);
let lu = m.lu().unwrap();

let l = &lu.l;         // lower-triangular factor
let u = &lu.u;         // upper-triangular factor
let p = &lu.p;         // permutation matrix (PA = LU)
let swaps = lu.num_swaps; // parity of the permutation
```

LAPACK routine: `dgetrf` / `sgetrf`

### QR decomposition

Performs the QR decomposition, which for matrix $\mathbf{A}$ computes a
decomposition $\mathbf{A} = \mathbf{Q}\mathbf{R}$ where $\mathbf{Q}$
is a **unitary** with unit-vector columns spanning the column space of the
matrix and $\mathbf{R}$ is an upper-triangular matrix.

!!! note
    This is not a pivot-based QR decompositon therefore it is not rank-revealing.


```rust
let m = DMatrix::<f64>::from_uniform_random(0.0, 1.0, 4, 3);
let qr = m.qr().unwrap();

let q = &qr.q;  // orthogonal factor
let r = &qr.r;  // upper-triangular factor
```

LAPACK routine: `dgeqrf` + `dorgqr` / single-precision equivalents

### Eigenvalue decomposition (general)

Returns complex eigenvalues even for real input matrices.

```rust
use num_complex::Complex;

let m = SMatrix::<f64, 3, 3>::from_uniform_random(0.0, 1.0);
let eig = m.eig().unwrap();

let vals: &[Complex<f64>; 3] = &eig.eigvals;
let lvecs = &eig.left_eigvecs;   // left eigenvectors (columns)
let rvecs = &eig.right_eigvecs;  // right eigenvectors (columns)
```

LAPACK routine: `dgeev` / `sgeev`

### Symmetric eigenvalue decomposition

For real symmetric matrices; eigenvalues are guaranteed real.

```rust
let m = SMatrix::<f64, 3, 3>::from_uniform_random(0.0, 1.0);
// (in practice m should be symmetric)
let symeig = m.symeig().unwrap();

let vals:  &[f64; 3] = &symeig.eigvals;  // real, ascending
let vecs = &symeig.eigvecs;              // orthonormal columns
```

LAPACK routine: `dsyev` / `ssyev`

### Schur decomposition

Decomposes a matrix as A = Q T Q^T where T is quasi-upper-triangular.

```rust
let m = DMatrix::<f64>::from_uniform_random(0.0, 1.0, 4, 4);
let schur = m.schur().unwrap();

let q = &schur.q;  // orthogonal factor
let t = &schur.t;  // quasi-upper-triangular factor
```

LAPACK routine: `dgees` / `sgees`

---

## Linear system solver

Solves `A * X = B` for `X` using LAPACK `gesv` (LU with partial pivoting).

```rust
let a = DMatrix::<f64>::from_row_slice(&[2.0, 1.0, 5.0, 3.0], 2, 2);
let b = DMatrix::<f64>::from_row_slice(&[1.0, 2.0], 2, 1);

let x = a.solve(&b).unwrap();
// a.matmul(&x) ≈ b
```

LAPACK routine: `dgesv` / `sgesv`

---

## Subviews

A subview provides a zero-copy window into a rectangular region of a matrix.
The view borrows the original matrix for its lifetime.

```rust
let m = DMatrix::<f64>::from_uniform_random(0.0, 1.0, 6, 6);

// Immutable view of rows 1..3, cols 2..4 (exclusive end)
let view = m.subview(1, 2, 2, 2);  // (start_row, start_col, nrows, ncols)
let v = view[(0, 1)];              // relative (row, col) within the view

// Mutable view
let mut m = DMatrix::<f64>::zeros(4, 4);
let mut view = m.subview_mut(1, 1, 2, 2);
view[(0, 0)] = 42.0;
```

Subviews support iteration and `fold` / `fold_indexed` reductions.

---

## Iteration

Iterators traverse elements in **column-major** order (matching internal storage).

```rust
// Owned iteration
for val in m {
    println!("{val}");
}

// Borrowed iteration
for val in &m {
    println!("{val}");
}

// Mutable iteration
for val in &mut m {
    *val *= 2.0;
}

// Low-level slice iterators
for val in m.iter() { /* ... */ }
for val in m.iter_mut() { /* ... */ }
```

---

## Serialization

All matrix types implement `serde::Serialize` and `serde::Deserialize`.

```rust
use serde_json;

let m = DMatrix::<f64>::from_row_slice(&[1.0, 2.0, 3.0, 4.0], 2, 2);

let json = serde_json::to_string(&m).unwrap();
let m2: DMatrix<f64> = serde_json::from_str(&json).unwrap();
```

The serialised form stores `data` (column-major flat array), `nrows`, and `ncols`.
