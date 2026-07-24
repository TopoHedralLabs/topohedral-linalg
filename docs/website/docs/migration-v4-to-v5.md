# Migrating from v4 to v5

Version 5 is intentionally breaking. It removes ambiguous method names, makes ownership costs
explicit, and enforces non-expression shape and FFI contracts consistently.

## Method renames

| v4 | v5 |
|---|---|
| `transform(f)` | `transform_mut(f)` |
| `transformed(f)` | `to_transformed(f)` |
| `shifted(x)` | `to_shifted(x)` |
| `scaled(x)` | `to_scaled(x)` |
| `filled(x)` | `to_filled(x)` |
| `sorted(dimension)` | `to_sorted(dimension)` |
| `abs()` (in place) | `abs_mut()` |
| `absed()` | `to_abs()` |
| `into_absed()` | `into_abs()` |

Every eager floating transformation follows the same pattern: `operation_mut` mutates,
`to_operation` clones and transforms, and `into_operation` consumes and transforms. For example,
`sqrt_mut`, `to_sqrt`, and `into_sqrt`.

## Decompositions consume matrices

In v4, LAPACK-backed methods borrowed an input and cloned its storage internally. In v5 they
consume it:

```rust
use topohedral_linalg::DMatrix;

let a = DMatrix::<f64>::identity(3, 3);
let retained = a.clone();
let lu = a.lu()?;
assert_eq!(retained[(0, 0)], 1.0);
# Ok::<(), topohedral_linalg::DLuError>(())
```

`solve` consumes both matrices. Static solve now permits any right-hand-side column count:

```rust
use topohedral_linalg::SMatrix;

let a = SMatrix::<f64, 2, 2>::identity();
let b = SMatrix::<f64, 2, 3>::ones();
let x: SMatrix<f64, 2, 3> = a.solve(b)?;
# Ok::<(), topohedral_linalg::SSolveError>(())
```

## Square-only operations

Import `SquareMatrixOps` for trace and determinant:

```rust
use topohedral_linalg::{DMatrix, SquareMatrixOps};

let a = DMatrix::<f64>::identity(2, 2);
assert_eq!(a.trace(), 2.0);
assert_eq!(a.determinant(), 1.0);
```

For static matrices, square-only decomposition methods exist only on `SMatrix<T, N, N>`.
Dynamic matrices check squareness at runtime.

## Indexed views

Indexed view constructors now take an owned collection of indices. Pass a `Vec`, an array, or use
the explicitly copying slice form:

```rust
use topohedral_linalg::{DMatrix, SubViewable};

let matrix = DMatrix::<i32>::zeros(4, 4);
let owned = matrix.subview_indices(vec![0, 2], vec![1, 3]);
let rows = [0, 2];
let cols = [1, 3];
let copied = matrix.subview_indices_from_slices(&rows, &cols);
assert_eq!(owned[(0, 0)], copied[(0, 0)]);
```

Mutable indexed views reject duplicate row or column indices.

## Errors

Decomposition error enums remain operation-specific and are `#[non_exhaustive]`, but raw private
LAPACK wrapper errors are no longer exposed. Match semantic variants with a fallback arm:

```rust
use topohedral_linalg::{DMatrix, DSolveError};

let a = DMatrix::from_row_slice(&[1.0_f64, 2.0, 2.0, 4.0], 2, 2);
let b = DMatrix::ones(2, 1);
match a.solve(b) {
    Err(DSolveError::Singular { pivot }) => assert_eq!(pivot, 2),
    Err(other) => return Err(other),
    Ok(_) => unreachable!("the example matrix is singular"),
}
# Ok::<(), DSolveError>(())
```

## Feature and dependency changes

The unused `enable_trace` feature was removed. `enable_checks` remains temporarily for legacy
lazy-expression checks; all non-expression public contract and FFI checks are unconditional.
