# GPU Support Roadmap and Scope Boundaries

This roadmap is intentionally conservative. The purpose is to find a technically sound fit for GPU compute within the current crate architecture, not to promise blanket GPU acceleration for every existing API.

## Current State

The crate has three relevant properties today:

- `DMatrix` stores dynamic dense matrices in one contiguous column-major `Vec<T>`; see [src/dmatrix/mod.rs](../../src/dmatrix/mod.rs#L42).
- elementwise lazy expressions are represented as nested `BinopExpr` values; see [src/expression/binary_expr.rs](../../src/expression/binary_expr.rs#L108).
- evaluation is CPU-only and happens during `From<BinopExpr> for DMatrix<T>` by iterating linear indices; see [src/dmatrix/mod.rs](../../src/dmatrix/mod.rs#L163).

Separately, matrix multiplication is eager and BLAS-backed today; see [src/dmatrix/matmul.rs](../../src/dmatrix/matmul.rs#L18).

## Phase 1: Explicit `DMatrix<f32>` Elementwise GPU Evaluation

Target:

- `DMatrix<f32>` only,
- elementwise expressions only,
- explicit `GpuContext` or `GpuRuntime`,
- fused evaluation of add/sub/mul/div and scalar combinations,
- CPU fallback for everything else.

This phase should answer the main architecture question:

Can the crate preserve its lazy, temporary-free elementwise model while evaluating selected expressions as one GPU kernel?

If the answer is yes, then later work can focus on performance and coverage.

## Why `f32` First

WGSL is not a drop-in match for the crate's current scalar surface.

The crate currently supports:

- `f32`
- `f64`
- signed integer types through `i128`

WGSL's scalar space is much narrower for practical compute kernels. The cleanest phase-1 rule is:

- GPU path supports `f32`,
- unsupported scalar types stay on CPU,
- no fake parity claims for `f64`, `i64`, or `i128`.

This is especially important because the current crate API is generic, while a realistic GPU backend will initially be selective.

## Recommended Public API Direction

Prefer explicit opt-in evaluation:

```rust
let expr = &a + &b + 2.0;
let out = gpu.eval(&expr).await?;
```

or

```rust
let out = expr.eval_with(&gpu).await?;
```

Avoid these in phase 1:

- automatic backend selection hidden behind existing `into()`,
- matrix residency embedded into `DMatrix`,
- silent data movement between CPU and GPU.

Those approaches all introduce bigger policy decisions than the crate needs for an initial experiment.

## Why Matrix-Vector Comes Next

After fused elementwise expressions, matrix-vector multiply is the next best GPU candidate.

Reasons:

- it is still simpler than full matrix-matrix multiplication,
- it is a common building block,
- it benefits from dedicated kernel structure rather than generic elementwise fusion.

But it should be designed as a separate kernel family. It does not belong in the same code generator as:

$$
\mathrm{out}[i] = f(a[i], b[i], \ldots)
$$

because matrix-vector multiply changes the access pattern to:

$$
y_r = \sum_{c=0}^{n-1} A_{r,c} x_c
$$

That requires reduction logic and a very different performance strategy.

## Matrix-Matrix Multiply Should Stay Separate

The current `matmul` path is eager and BLAS-backed, not part of the elementwise expression-template system; see [src/dmatrix/matmul.rs](../../src/dmatrix/matmul.rs#L18).

That separation should remain true in the GPU roadmap:

- fused elementwise evaluator,
- matrix-vector kernel family,
- matrix-matrix kernel family.

Trying to unify all three too early would blur semantics and make the first implementation harder to reason about.

## Performance Caveat: Transfer Cost Matters More Than Syntax

The expression-template fit is architecturally promising, but performance will depend heavily on:

- buffer upload cost,
- buffer readback cost,
- whether results remain GPU-resident for follow-on operations,
- matrix size,
- dispatch overhead.

This matters because the current add benchmark coverage is small and CPU-oriented. The benchmark file at [benches/dmatrix_addop.rs](../../benches/dmatrix_addop.rs) only exercises dimensions `10`, `20`, `30`, and `40`, and uses `f64`.

Those sizes are far too small to justify claims about GPU speedups.

## Benchmarking Guidance for the Prototype

Do not make performance claims until the prototype measures at least:

- `256x256`
- `512x512`
- `1024x1024`

and distinguishes:

- upload + compute + readback end-to-end cost,
- compute-only cost for already-resident data,
- CPU fused evaluation baseline.

The architectural write-up should be explicit that a GPU backend may lose badly for small matrices even if the implementation is correct.

## Suggested Milestone Sequence

1. Document the current lazy evaluation boundary.
2. Introduce runtime lowering for elementwise expressions.
3. Add a fused CPU evaluator over the lowered IR.
4. Add a `wgpu`-based `DMatrix<f32>` add kernel.
5. Generalize GPU codegen to fused `Add/Sub/Mul/Div`.
6. Add CPU fallback using the same lowered IR.
7. Benchmark larger matrices and compare transfer-sensitive and compute-only cases.
8. Explore matrix-vector as a separate GPU subsystem.

## Acceptance Cases for the Exploration

- `&a + &b` on `DMatrix<f32>` matches CPU output.
- `(&a + &b + &c + 2.0)` evaluates without temporary matrices.
- unsupported scalar types fall back to CPU without changing expression syntax.
- missing adapters or device creation failure also fall back cleanly to CPU.
- the docs remain explicit that this is an exploratory backend, not a full replacement for current BLAS/LAPACK paths.

## Baseline Validation

The current branch already has a green baseline on the core expression and matrix-op test suites:

- `cargo test --test elementwise --quiet`
- `cargo test --test matrix_ops --quiet`

That is useful because the first GPU work should preserve existing CPU semantics rather than redefining them.

## Source Links

- `wgpu` crate docs: <https://docs.rs/wgpu/latest/wgpu/>
- WGSL specification: <https://www.w3.org/TR/WGSL/>
