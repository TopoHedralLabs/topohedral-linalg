# Changelog

All notable changes to this project are documented in this file.

## 5.0.0 - Unreleased

### Changed

- Matrix decompositions and linear solves now consume their input matrices so LAPACK can reuse
  owned storage. Clone an input explicitly before calling the operation if it must be retained.
- Static square-only operations are available only on `SMatrix<T, N, N>`. Static `solve` accepts
  any number of right-hand-side columns and returns the same shape.
- `trace` and `determinant` moved from `MatrixOps` to `SquareMatrixOps`.
- Transformation methods now consistently use `*_mut`, `to_*`, and `into_*` naming. The borrowed
  sorting method is now `to_sorted`.
- Public decomposition errors use semantic variants such as `Singular`,
  `NotPositiveDefinite`, and `NoConvergence`.
- Indexed views take ownership of index vectors; `*_from_slice` constructors copy borrowed index
  slices explicitly.

### Fixed

- BLAS/LAPACK calls validate dimensions and storage lengths before entering FFI.
- Dynamic square operations reject rectangular matrices.
- Trace starts from the additive identity, determinant uses the LU diagonal product, and
  permutation parity is counted correctly.
- Constructors detect dimension overflow and reject storage with the wrong length.
- Mutable indexed views reject duplicate indices.
- Empty and invalid subview ranges fail with deterministic panic messages.

### Added

- Public sealed `BlasScalar`, `LapackScalar`, and `UniformRandom` capability traits.
- Owned row-major and column-major `DMatrix` constructors.
- `AsRef<[T]>`, `AsMut<[T]>`, and owned/borrowed iteration for matrix storage.
- Package metadata, the MIT license text, a dependency-license policy, and expanded CI checks.

### Removed

- The unused `enable_trace` feature and its tracing dependency.
- Unused `log` dependency; `serde_json` is now development-only.

Lazy-expression bounds checking, expression materialisation, expression type exposure, the
expression-dependent dynamic-vector redesign, and owned elementwise allocation reuse are
intentionally deferred to a separate performance-focused change.
