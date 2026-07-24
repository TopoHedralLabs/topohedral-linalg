# Topohedral-Linalg

`topohedral-linalg` is a Rust crate for small, dense linear algebra. It provides runtime-sized
`DMatrix` and compile-time-sized `SMatrix` types with column-major storage, BLAS/LAPACK-backed
matrix multiplication and decompositions, matrix views, reductions, transformations, and
lazily-evaluated elementwise expressions.

```rust
use topohedral_linalg::{DMatrix, MatMul, SquareMatrixOps};

let a = DMatrix::<f64>::from_row_slice(&[1.0, 2.0, 3.0, 4.0], 2, 2);
let identity = DMatrix::<f64>::identity(2, 2);
let product = a.matmul(identity);

assert_eq!(product.trace(), 5.0);
```

See the [documentation site](https://topohedrallabs.github.io/topohedral-linalg/) for installation,
the user guide, API documentation, and the v4-to-v5 migration guide.

This library is part of the Topohedral Project, an open-source geometric-modelling kernel from
TopoHedralLabs. It is licensed under the MIT License.
