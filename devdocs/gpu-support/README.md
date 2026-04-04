# GPU Support Exploration for `DMatrix`

This folder captures a practical path for introducing GPU-backed compute into `topohedral-linalg` with [`wgpu`](https://docs.rs/wgpu/latest/wgpu/).

The short version is:

- `DMatrix` already has a storage model that maps cleanly to GPU buffers: contiguous, column-major data in a `Vec<T>`.
- The current expression-template system already delays elementwise work until materialization.
- A GPU backend becomes realistic once expression evaluation stops being "walk the nested Rust type tree directly" and starts by lowering that tree into a small runtime expression plan.

The main recommendation is to start with an explicit, opt-in `DMatrix<f32>` proof of concept for fused elementwise expressions. Matrix-vector and matrix-matrix operations should be treated as separate kernel families after that, not folded into the same elementwise fusion system.

## Why This Fits the Current Code

`DMatrix` stores its payload as a single contiguous column-major buffer in `data: Vec<T>` with `nrows` and `ncols`; see [src/dmatrix/mod.rs](../../src/dmatrix/mod.rs#L42). The linear storage index is

$$
i = r + c\,n_{\mathrm{rows}}
$$

for row \(r\) and column \(c\). That same linear indexing is already what the elementwise expression evaluator uses today.

`BinopExpr` nodes represent nested lazy elementwise operations, and the current `From<BinopExpr> for DMatrix<T>` implementation materializes them with a CPU loop over all linear indices; see [src/expression/binary_expr.rs](../../src/expression/binary_expr.rs#L108) and [src/dmatrix/mod.rs](../../src/dmatrix/mod.rs#L163).

```mermaid
flowchart LR
    A["Nested `BinopExpr` tree"] --> B["Lower to runtime elementwise IR"]
    B --> C["CPU evaluator"]
    B --> D["GPU evaluator (`wgpu`)"]
    C --> E["`DMatrix` output"]
    D --> E
```

## Findings

- Elementwise add, subtract, multiply, divide, and scalar combinations are a strong fit for compute kernels.
- Column-major layout is not a blocker for elementwise kernels, because the GPU can operate on the exact same linear storage order the crate already uses.
- The current lazy model avoids temporaries structurally, but only on CPU. A GPU backend needs an intermediate runtime IR so a chain such as `(&a + &b + &c + 2.0)` can be emitted as one fused kernel instead of multiple passes.
- The current generic scalar surface is much broader than WGSL's practical kernel target space. The safest phase-1 scope is `DMatrix<f32>`.
- Existing `matmul` is eager and BLAS-backed today; see [src/dmatrix/matmul.rs](../../src/dmatrix/matmul.rs#L18). It should remain separate from the first fused elementwise design.

## Recommended Phase 1

- Scope: `DMatrix<f32>` only.
- Backend: native `wgpu` compute only.
- API style: explicit opt-in evaluation through a `GpuContext` or `GpuRuntime`.
- Goal: one output allocation, one kernel dispatch, no intermediate matrices for chained elementwise expressions.

## Doc Index

- [Elementwise add proof of concept](./elementwise-add.md)
- [Expression lowering for lazy fused evaluation](./expression-lowering.md)
- [Roadmap and scope boundaries](./roadmap.md)

## Source Links

- `wgpu` crate docs: <https://docs.rs/wgpu/latest/wgpu/>
- `wgpu::Instance`: <https://docs.rs/wgpu/latest/wgpu/struct.Instance.html>
- `wgpu::Adapter`: <https://docs.rs/wgpu/latest/wgpu/struct.Adapter.html>
- `wgpu::Device`: <https://docs.rs/wgpu/latest/wgpu/struct.Device.html>
- `wgpu::CommandEncoder`: <https://docs.rs/wgpu/latest/wgpu/struct.CommandEncoder.html>
- `wgpu::ComputePass`: <https://docs.rs/wgpu/latest/wgpu/struct.ComputePass.html>
- `wgpu::Queue`: <https://docs.rs/wgpu/latest/wgpu/struct.Queue.html>
- `wgpu::Buffer`: <https://docs.rs/wgpu/latest/wgpu/struct.Buffer.html>
- `wgpu::BufferUsages`: <https://docs.rs/wgpu/latest/wgpu/struct.BufferUsages.html>
- `wgpu::util::DeviceExt`: <https://docs.rs/wgpu/latest/wgpu/util/trait.DeviceExt.html>
- WGSL specification: <https://www.w3.org/TR/WGSL/>
