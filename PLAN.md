# Boolean Matrices and Masked Indexing

## Summary

Add first-class boolean matrix storage and lazy boolean expressions, enabling:

```rust
let a = b.masked(c.gt(&d)).to_dmatrix();
```

The exact `c > d` syntax is impossible in Rust because comparison operators must return one `bool`. Existing coordinate-based `subview` remains unchanged.

## Key Changes

- Relax structural APIs from `T: Field + Copy` to the minimum required bounds:
  - Matrix storage, construction, indexing, iteration, views, copying, transforms, reductions, serde, and `MatrixExpr`.
  - Retain `Field` and BLAS-specific bounds only on numeric operations.
  - Implement `Zero` and `One` for `bool` as `false` and `true`.
  - Preserve numeric display formatting and display booleans as `true`/`false`.

- Generalize expressions:
  - Change `MatrixExpr::ScalarType` to require `Copy`, not `Field`.
  - Keep arithmetic expression nodes numerically bounded.
  - Add lazy comparison expressions returning `bool`.
  - Export an elementwise comparison trait with `eq`, `ne`, `lt`, `le`, `gt`, and `ge`, supporting matrix and scalar right-hand sides.
  - Add lazy `&`, `|`, `^`, and `!` composition for boolean matrices and mask expressions.
  - Permit comparison and boolean expressions to materialize as `SMatrix<bool, N, M>` or `DMatrix<bool>`.

- Add masked selection:
  - Export a `masked(mask)` method and immutable `MaskedView`.
  - Accept any boolean `MatrixExpr` with dimensions identical to the source; panic clearly on mismatch.
  - Store the mask expression by value so temporary expressions such as `c.gt(&d)` remain valid.
  - Iterate selected source entries in column-major order without first materializing the mask.
  - Report the masked view as `K × 1`, where `K` is the number of `true` entries.
  - `to_dmatrix()` returns `DMatrix<T>` with shape `K × 1`; an empty selection returns `0 × 1`.
  - Keep this first version read-only; no masked assignment or mutable masked view.

## Test Plan

- Instantiate, construct, index, mutate, copy, serialize, deserialize, transform, view, and iterate both boolean matrix types.
- Test all six comparisons for static and dynamic matrices, matrix/scalar operands, borrowed operands, views, and nested arithmetic expressions.
- Materialize comparison and composed-mask expressions into boolean matrices.
- Verify `AND`, `OR`, `XOR`, and `NOT`, including nested expressions.
- Verify masked selection for mixed, all-true, all-false, rectangular, static-source, dynamic-source, and view-source cases.
- Assert `K × 1` shape and crate-native column-major selection order.
- Assert clear panics for source/mask dimension mismatches.
- Run the existing suite to confirm numeric arithmetic, expression evaluation, rectangular subviews, serde, and BLAS APIs remain unchanged.

## Assumptions

- This is a foundational trait-bound refactor but should preserve existing source-level numeric APIs.
- Boolean matrices do not gain arithmetic, trace, determinant, vector norms, or BLAS/LAPACK operations.
- Existing `subview(row_start, row_end, col_start, col_end)` calls remain fully compatible.
