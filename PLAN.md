# Eliminate Generic Const Expressions

## Summary

Replace flat `SMatrix` storage with stable nested arrays while preserving column-major layout, SIMD-friendly flattened access, public behavior, and element-wise performance within a +3% statistical threshold.

## Implementation Changes

- Change storage to `data: [[T; N]; M]`; retain `nrows`/`ncols` to minimize behavioral change.
- Add crate-private, inlined `as_slice()` and `as_mut_slice()` using `as_flattened()` and route all linear operations through them.
- Preserve single-loop `MatrixExpr::eval_into` evaluation and unchecked leaf reads so LLVM continues receiving contiguous, non-aliasing slices suitable for SIMD.
- Update constructors, indexing, iteration, sorting, transforms, reductions, subviews, expression materialization, and BLAS/LAPACK calls to use flattened slices.
- Convert flat LAPACK results with nested `std::array::from_fn`, avoiding computed array lengths.
- Preserve the existing flat column-major Serde representation, including `data`, `nrows`, and `ncols`.
- Replace owned iteration with the flattened nested-array iterator while preserving column-major order.
- Remove all `[(); N * M]:` witness bounds, computed generic array lengths, `generic_const_exprs` feature attributes, and `allow(incomplete_features)` where no longer needed.
- Remove the public `GreaterThan` trait and export. Permit `VectorOps` for all static vector lengths; retain existing method-specific runtime checks for operations such as angle and cross product.
- Keep unrelated nightly floating-point features and the nightly toolchain unchanged.

## Performance Validation

- Extend `smatrix_addop` into a representative static element-wise suite covering:
  - Existing nine-input fused addition.
  - Mixed fused add/multiply/subtract/divide expression.
  - Scalar eager/in-place operation.
  - Unary operation.
  - Square sizes 10, 20, and 40.
  - Rectangular `2×512` and `512×2` cases to detect accidental per-column loops.
- Before implementation, save a Criterion baseline using the same machine, toolchain, power state, and `-C target-cpu=native`.
- After implementation, compare against that saved baseline.
- Each benchmark passes when the upper bound of Criterion’s 95% change interval is at most +3%.
- Rerun a failing/noisy case twice; accept only if at least two of three runs meet the +3% upper-bound gate.
- Inspect optimized LLVM IR or assembly for the fused `f64` kernel and confirm vector loads/arithmetic/stores remain, with one flattened loop and no per-column inner loop.
- Keep benchmark names stable where they already exist so historical Criterion results remain comparable.

## Test Plan

- Add tests for flattened column-major order, mutable flattening, owned/borrowed iteration, rectangular matrices, transpose, expression materialization, and BLAS inputs.
- Add an exact Serde fixture confirming `data` remains a flat sequence and round-trips correctly.
- Run formatting, Clippy, all-target/all-feature build, and the complete test suite.
- Verify repository searches find no `generic_const_exprs`, `Assert<{...}>`, `[(); expression]`, or computed generic array lengths.
- Confirm existing matrix arithmetic, decomposition, boolean-expression, comparison, sorting, subview, and vector tests remain unchanged semantically.

## Assumptions

- Internal storage is private, so changing its Rust representation is not a public source API change.
- Serialized data compatibility and column-major iteration order must be preserved.
- A regression beyond the +3% gate blocks completion until the flattened SIMD path is restored.
