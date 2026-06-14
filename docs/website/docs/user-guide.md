# User Guide

This page covers the full surface area of `topohedral-linalg`, with examples for
both `SMatrix` (fixed size) and `DMatrix` (dynamic size).

## Imports

There is no prelude — import what you need. Everything is re-exported from the crate root:

| What you need | Import path |
|---|---|
| `SMatrix` | `topohedral_linalg::SMatrix` |
| `DMatrix` | `topohedral_linalg::DMatrix` |
| `SRVector`, `SCVector` (static row/column vector aliases) | `topohedral_linalg::{SRVector, SCVector}` |
| `DVector`, `VecType` (dynamic vector alias and orientation) | `topohedral_linalg::{DVector, VecType}` |
| `Field` (scalar element bound) | `topohedral_linalg::Field` |
| Traits (`MatMul`, `MatrixOps`, `ReduceOps`, `TransformOps`, `FloatTransformOps`, `Shape`, `VectorOps`, …) | `topohedral_linalg::{TraitName}` |
| `SubViewable`, `SubViewableMut` (subview traits) | `topohedral_linalg::{SubViewable, SubViewableMut}` |
| `Dimension` (for sorting) | `topohedral_linalg::Dimension` |
| Lazy unary functions (`sin`, `cos`, `sqrt`, …) | `topohedral_linalg::{fn_name}` |

A typical set of imports for general use:

```rust
use topohedral_linalg::{
    DMatrix, SMatrix,
    Dimension, MatMul, MatrixOps, ReduceOps, Shape,
    TransformOps, FloatTransformOps,
};
```

Add individual trait imports only when the methods they provide are actually needed;
the Rust compiler will tell you which trait to bring into scope if a method call
fails to resolve.

---

## The `Field` trait

`Field` is the scalar element bound used throughout the library. It requires the four
arithmetic operators and their assignment variants, negation, and a partial order:

```
Field = Add + Sub + Mul + Div + AddAssign + SubAssign + MulAssign + DivAssign + Neg + PartialOrd + PartialEq
```

The following primitive types implement `Field`:

| Type | Kind |
|---|---|
| `f32`, `f64` | Floating point |
| `i8`, `i16`, `i32`, `i64`, `i128` | Signed integer |

Use `Field` as a bound when writing generic code over any matrix element type:

```rust
use topohedral_linalg::{DMatrix, Field};

fn scale_matrix<T: Field + Copy>(m: &DMatrix<T>, factor: T) -> DMatrix<T> {
    m.clone() * factor
}
```

For floating-point-only operations (decompositions, transcendental functions, norms),
use `Float` instead, which is a stricter bound that implies `Field`.

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

## Vector types

The library provides type aliases for 1-D vectors on top of the matrix types.

### Static vectors (`SRVector` and `SCVector`)

`SRVector<T, N>` is a 1×N row vector (`SMatrix<T, 1, N>`).  
`SCVector<T, N>` is an N×1 column vector (`SMatrix<T, N, 1>`).  
All `SMatrix` methods are available on these types automatically.

```rust
use topohedral_linalg::{SRVector, SCVector};

let row: SRVector<f64, 3> = SRVector::<f64, 3>::from_row_slice(&[1.0, 2.0, 3.0]);
let col: SCVector<f64, 3> = SCVector::<f64, 3>::from_col_slice(&[1.0, 2.0, 3.0]);
```

### Dynamic vector (`DVector`)

`DVector<T>` is a type alias for `DMatrix<T>`. Use `VecType` to select orientation when constructing:

```rust
use topohedral_linalg::{DVector, VecType};

// Column vector (N×1)
let col = DVector::<f64>::zeros_vec(5, VecType::Col);
let col = DVector::<f64>::ones_vec(5, VecType::Col);
let col = DVector::<f64>::from_value_vec(3.14, 5, VecType::Col);
let col = DVector::<f64>::from_slice_vec(&[1.0, 2.0, 3.0], VecType::Col);
let col = DVector::<f64>::from_uniform_random_vec(0.0, 1.0, 5, VecType::Col);

// Row vector (1×N)
let row = DVector::<f64>::zeros_vec(5, VecType::Row);
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

Return and error types follow a prefixed naming convention: `D` for `DMatrix` results,
`S` for `SMatrix` results. For example, `DMatrix::lu()` returns `Result<DLuReturn<T>, DLuError>`
and `SMatrix::lu()` returns `Result<SLuReturn<T, N, M>, SLuError>`. The full set:

| Decomposition | `DMatrix` types | `SMatrix` types |
|---|---|---|
| LU | `DLuReturn<T>`, `DLuError` | `SLuReturn<T, N, M>`, `SLuError` |
| QR | `DQrReturn<T>`, `DQrError` | `SQrReturn<T, N, M>`, `SQrError` |
| Cholesky | `DCholeskyReturn<T>`, `DCholeskyError` | `SCholeskyReturn<T, N>`, `SCholeskyError` |
| Eigenvalue (general) | `DEigReturn<T>`, `DEigError` | `SEigReturn<T, N, M>`, `SEigError` |
| Eigenvalue (symmetric) | `DSymEigReturn<T>`, `DSymEigError` | `SSymEigReturn<T, N, M>`, `SSymEigError` |
| Schur | `DSchurReturn<T>`, `DSchurError` | `SSchurReturn<T, N, M>`, `SSchurError` |
| Linear solve | `DSolveError` | `SSolveError` |

All of these are re-exported from the crate root.

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

### Cholesky decomposition

For a real symmetric positive-definite matrix $\mathbf{A}$, computes a lower-triangular
factor $\mathbf{L}$ such that:

$$\mathbf{A} = \mathbf{L}\mathbf{L}^T$$

Cholesky is often the preferred factorization for covariance matrices, Gram matrices,
normal equations, and other symmetric positive-definite systems because it is faster and
uses less storage than a general LU decomposition.

!!! note
    Only the lower triangular part of the input is read; the upper triangle is ignored.
    If the matrix is not positive definite, `cholesky()` returns an error from LAPACK.

```rust
use topohedral_linalg::{DMatrix, MatMul, MatrixOps};

let a = DMatrix::<f64>::from_row_slice(
    &[
        4.0, 12.0, -16.0,
        12.0, 37.0, -43.0,
        -16.0, -43.0, 98.0,
    ],
    3,
    3,
);

let chol = a.cholesky().unwrap();

let l = &chol.l;                 // lower-triangular factor
let reconstructed = l.matmul(l.transpose()); // reconstructed ≈ a
```

The same method is available on static matrices:

```rust
use topohedral_linalg::SMatrix;

let a = SMatrix::<f64, 3, 3>::from_row_slice(&[
    4.0, 12.0, -16.0,
    12.0, 37.0, -43.0,
    -16.0, -43.0, 98.0,
]);

let chol = a.cholesky().unwrap();
let l = &chol.l;
```

LAPACK routine: `dpotrf` / `spotrf`

### Eigenvalue decomposition (general)

For a square matrix $\mathbf{A}$, computes the eigenvalues
$\lambda_{i}$ and the left and right eigenvectors satisfying:

$$
\mathbf{A}\mathbf{v}_{i}
=
\lambda_{i} \mathbf{v}_{i}
\qquad \text{(right eigenvectors)}
$$

$$
\mathbf{u}_{i}^H \mathbf{A}
=
\lambda_{i} \mathbf{u}_{i}^H
\qquad \text{(left eigenvectors)}
$$

For real matrices, complex eigenvalues always appear as conjugate
pairs $\lambda, \bar{\lambda}$.  Unlike the symmetric case, the right
eigenvectors do not generally form an orthogonal basis, and for
defective matrices (repeated eigenvalues with a shortage of independent eigenvectors)
the decomposition may be ill-conditioned.

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

For a real symmetric matrix $\mathbf{A} = \mathbf{A}^T$, computes the spectral decomposition:

$$\mathbf{A} = \mathbf{Q}\boldsymbol{\Lambda}\mathbf{Q}^T$$

where $\mathbf{Q}$ is **orthogonal** (columns are the orthonormal eigenvectors) and
$\boldsymbol{\Lambda} = \operatorname{diag}(\lambda_1, \ldots, \lambda_n)$ with real eigenvalues
in ascending order $\lambda_1 \leq \lambda_2 \leq \cdots \leq \lambda_n$.

!!! note
    Only the lower triangular part of the input is read; the upper triangle is ignored.
    This routine is more efficient than the general eigenvalue decomposition and should be
    preferred whenever the matrix is known to be symmetric.

```rust
let m = SMatrix::<f64, 3, 3>::from_uniform_random(0.0, 1.0);
// (in practice m should be symmetric)
let symeig = m.symeig().unwrap();

let vals:  &[f64; 3] = &symeig.eigvals;  // real, ascending
let vecs = &symeig.eigvecs;              // orthonormal columns
```

LAPACK routine: `dsyev` / `ssyev`

### Schur decomposition

For a square matrix $\mathbf{A}$, computes an **orthogonal** $\mathbf{Q}$ and a
quasi-upper-triangular $\mathbf{T}$ such that:

$$\mathbf{A} = \mathbf{Q}\mathbf{T}\mathbf{Q}^T$$

$\mathbf{T}$ is in real Schur form: block upper-triangular with $1 \times 1$ diagonal blocks
for real eigenvalues and $2 \times 2$ blocks for conjugate complex pairs. The columns of
$\mathbf{Q}$ are **Schur vectors** — orthonormal, but not eigenvectors in general. The
eigenvalues of $\mathbf{A}$ are the eigenvalues of the diagonal blocks of $\mathbf{T}$.

!!! note
    The Schur decomposition exists for any square matrix and always yields an orthogonal $\mathbf{Q}$,
    even when $\mathbf{A}$ is defective (i.e. does not have a full set of linearly independent
    eigenvectors and therefore cannot be diagonalized).

```rust
let m = DMatrix::<f64>::from_uniform_random(0.0, 1.0, 4, 4);
let schur = m.schur().unwrap();

let q = &schur.q;  // orthogonal factor
let t = &schur.t;  // quasi-upper-triangular factor
```

LAPACK routine: `dgees` / `sgees`

---

## Linear system solver

Solves $\mathbf{A} * \mathbf{X} = \mathbf{B}$ for $\mathbf{X}$ using an LU decomposition
with partial pivoting.

```rust
let a = DMatrix::<f64>::from_row_slice(&[2.0, 1.0, 5.0, 3.0], 2, 2);
let b = DMatrix::<f64>::from_row_slice(&[1.0, 2.0], 2, 1);

let x = a.solve(&b).unwrap();
// a.matmul(&x) ≈ b
```

LAPACK routine: `dgesv` / `sgesv`

---

## Subviews

A subview is a zero-copy, borrowed window into a rectangular region of a matrix.
All subview indices are **inclusive** on both ends: `subview(r0, r1, c0, c1)` covers
rows `r0..=r1` and columns `c0..=c1`.

### Immutable views

The `SubViewable` trait provides methods that return a `MatrixView` borrowing `&self`:

```rust
use topohedral_linalg::SubViewable;

let m = DMatrix::<f64>::from_uniform_random(0.0, 1.0, 6, 6);

// Arbitrary rectangular block: rows 1..=3, cols 2..=4
let view = m.subview(1, 3, 2, 4);
let v = view[(0, 1)];   // relative indices within the view

// Convenience shortcuts
let row2  = m.row(2);           // entire row 2  (1 × ncols view)
let rows  = m.rows(1, 3);       // rows 1..=3    (3 × ncols view)
let col1  = m.col(1);           // entire col 1  (nrows × 1 view)
let cols  = m.cols(2, 4);       // cols 2..=4    (nrows × 3 view)
```

### Mutable views

The `SubViewableMut` trait mirrors `SubViewable` with `&mut self` variants that return
a `MatrixViewMut`. Writing through a `MatrixViewMut` writes directly into the parent matrix.

```rust
use topohedral_linalg::SubViewableMut;

let mut m = DMatrix::<f64>::zeros(6, 6);

// Element-by-element writes using (row, col) or linear index
{
    let mut view = m.subview_mut(1, 3, 2, 4);
    view[(0, 0)] = 1.0;   // row 0, col 0 of the view → m[(1, 2)]
    view[(1, 2)] = 5.0;   // row 1, col 2 of the view → m[(2, 4)]
    view[0]      = 9.0;   // linear index 0 (column-major) → m[(1, 2)]
}

// Mutable convenience shortcuts
let mut row_view = m.row_mut(2);       // entire row 2
let mut col_view = m.col_mut(1);       // entire col 1
let mut rows_view = m.rows_mut(1, 3);  // rows 1..=3
let mut cols_view = m.cols_mut(2, 4);  // cols 2..=4
```

#### Iterating over a mutable view

`MatrixViewMut` provides both a shared and a mutable column-major iterator:

```rust
let mut m = DMatrix::<f64>::from_uniform_random(0.0, 1.0, 4, 4);
let mut view = m.subview_mut(0, 1, 0, 1);  // top-left 2×2 block

// Read-only pass
for val in view.iter() {
    println!("{val}");
}

// Mutating pass — doubles every element in the block
for val in view.iter_mut() {
    *val *= 2.0;
}
```

#### Bulk copy into a mutable view

`copy_from` writes all elements of a same-sized matrix or view into the mutable view:

```rust
let src = DMatrix::<f64>::ones(2, 2);
let mut dst = DMatrix::<f64>::zeros(4, 4);

// Copy src into the bottom-right 2×2 block of dst
let mut view = dst.subview_mut(2, 3, 2, 3);
view.copy_from(src);
```

### Write-helper methods on `DMatrix` and `SMatrix`

Both `DMatrix` and `SMatrix` provide the same three higher-level helpers that skip
constructing the view explicitly:

| Method | Effect |
|---|---|
| `set_row(row, rhs)` | Overwrites a single row |
| `set_col(col, rhs)` | Overwrites a single column |
| `set_subview(r0, r1, c0, c1, rhs)` | Overwrites a rectangular block (inclusive indices) |

`rhs` can be any matrix or view type with matching dimensions.

```rust
// DMatrix example
let mut m  = DMatrix::<f64>::zeros(4, 4);
let row    = DMatrix::<f64>::ones(1, 4);
let col    = DMatrix::<f64>::ones(4, 1);
let block  = DMatrix::<f64>::ones(2, 2);

m.set_row(1, row);                // overwrite row 1
m.set_col(2, col);                // overwrite col 2
m.set_subview(1, 2, 1, 2, block); // overwrite rows 1..=2, cols 1..=2

// SMatrix example — identical API
let mut s   = SMatrix::<f64, 4, 4>::zeros();
let s_row   = SMatrix::<f64, 1, 4>::ones();
let s_block = SMatrix::<f64, 2, 2>::ones();

s.set_row(1, s_row);
s.set_subview(1, 2, 1, 2, s_block);
```

Both types also have a top-level `copy_from` that replaces the entire matrix contents
from any same-sized source:

```rust
let src = DMatrix::<f64>::identity(4, 4);
let mut dst = DMatrix::<f64>::zeros(4, 4);
dst.copy_from(src);

let s_src = SMatrix::<f64, 3, 3>::identity();
let mut s_dst = SMatrix::<f64, 3, 3>::zeros();
s_dst.copy_from(s_src);
```

### Converting a view to an owned matrix

`MatrixView` and `MatrixViewMut` over either `DMatrix` or `SMatrix` can be
materialised into a new heap-allocated `DMatrix` via `.to_dmatrix()`. Note that
views over an `SMatrix` always produce a `DMatrix` — there is no `to_smatrix()`:

```rust
// From a DMatrix view
let m = DMatrix::<f64>::from_uniform_random(0.0, 1.0, 6, 6);
let owned: DMatrix<f64> = m.subview(1, 3, 1, 3).to_dmatrix();

// From an SMatrix view — result is always DMatrix
let s = SMatrix::<f64, 6, 6>::from_uniform_random(0.0, 1.0);
let owned: DMatrix<f64> = s.subview(1, 3, 1, 3).to_dmatrix();
```

### Using subview traits in generic code

`SubViewable` and `SubViewableMut` are re-exported from the crate root and can be used
as bounds in generic functions:

```rust
use topohedral_linalg::{SubViewable, SubViewableMut};

fn sum_block<M: SubViewable<Output = f64>>(mat: &M, r0: usize, r1: usize, c0: usize, c1: usize) -> f64 {
    let view = mat.subview(r0, r1, c0, c1);
    view.iter().copied().sum()
}

fn zero_block<M: SubViewableMut<Output = f64>>(mat: &mut M, r0: usize, r1: usize, c0: usize, c1: usize) {
    let mut view = mat.subview_mut(r0, r1, c0, c1);
    for val in view.iter_mut() {
        *val = 0.0;
    }
}
```

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
