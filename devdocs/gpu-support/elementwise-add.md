# `DMatrix<f32>` Elementwise Add via `wgpu`

This document uses elementwise addition of two `DMatrix<f32>` values as the smallest realistic GPU test case for this crate.

## Why Addition Is the Right First Kernel

Elementwise add matches the current lazy expression system very closely:

- inputs are flat buffers of equal logical shape,
- output shape is known immediately,
- each output element is independent,
- the current CPU evaluator already computes one linear index at a time.

That means a GPU prototype can preserve the crate's current semantics while changing only the evaluation backend.

## Data Layout Match

`DMatrix<T>` stores column-major data contiguously in a single `Vec<T>`; see [src/dmatrix/mod.rs](../../src/dmatrix/mod.rs#L42) and [src/dmatrix/indexing.rs](../../src/dmatrix/indexing.rs#L160).

For elementwise work, the GPU does not need to care that the logical matrix is column-major. It only needs the same linear storage order as the CPU evaluator:

$$
\mathrm{out}[i] = \mathrm{lhs}[i] + \mathrm{rhs}[i], \qquad 0 \le i < n_{\mathrm{rows}} n_{\mathrm{cols}}
$$

So long as buffers are uploaded in the same order as `DMatrix::data`, the kernel is correct.

## `wgpu` Execution Flow

For a minimal compute path, the host-side flow is:

1. Create a [`wgpu::Instance`](https://docs.rs/wgpu/latest/wgpu/struct.Instance.html).
2. Request an [`Adapter`](https://docs.rs/wgpu/latest/wgpu/struct.Adapter.html) with `Instance::request_adapter`.
3. Request a [`wgpu::Device`](https://docs.rs/wgpu/latest/wgpu/struct.Device.html) and [`wgpu::Queue`](https://docs.rs/wgpu/latest/wgpu/struct.Queue.html) with `Adapter::request_device`.
4. Build a WGSL shader module with `Device::create_shader_module`.
5. Build a compute pipeline with `Device::create_compute_pipeline`.
6. Create storage buffers for `a`, `b`, and `out`.
7. Encode a compute pass with `CommandEncoder::begin_compute_pass`.
8. Bind buffers, dispatch workgroups with `ComputePass::dispatch_workgroups`, then submit via `Queue::submit`.
9. Copy the output into a CPU-readable buffer and map it back to host memory.

## Minimal WGSL Kernel

The kernel can be one thread per linear element:

```wgsl
@group(0) @binding(0)
var<storage, read> a: array<f32>;

@group(0) @binding(1)
var<storage, read> b: array<f32>;

@group(0) @binding(2)
var<storage, read_write> out: array<f32>;

struct Params {
    len: u32,
}

@group(0) @binding(3)
var<uniform> params: Params;

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let idx = gid.x;
    if (idx < params.len) {
        out[idx] = a[idx] + b[idx];
    }
}
```

This is the right mental model for the first proof of concept:

- each invocation owns exactly one output element,
- bounds are explicit,
- the kernel operates on flat storage,
- shape metadata stays on the host except for the total length.

## Host-Side Prototype Shape

The eventual Rust-side proof of concept should look conceptually like this:

```rust
pub struct GpuContext {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    add_pipeline: wgpu::ComputePipeline,
}

impl GpuContext {
    pub async fn new() -> Result<Self, GpuError> { /* setup */ }

    pub async fn add_f32(
        &self,
        lhs: &DMatrix<f32>,
        rhs: &DMatrix<f32>,
    ) -> Result<DMatrix<f32>, GpuError> {
        /* upload buffers, dispatch, read back, rebuild DMatrix */
    }
}
```

The key point is that this should be explicit opt-in, not transparent auto-dispatch.

## Prototype Buffer Plan

Use these buffers:

- `lhs_buffer`: `STORAGE | COPY_DST`
- `rhs_buffer`: `STORAGE | COPY_DST`
- `out_buffer`: `STORAGE | COPY_SRC`
- `readback_buffer`: `MAP_READ | COPY_DST`
- `params_buffer`: `UNIFORM | COPY_DST`

Input upload can use:

- [`Queue::write_buffer`](https://docs.rs/wgpu/latest/wgpu/struct.Queue.html), or
- [`wgpu::util::DeviceExt::create_buffer_init`](https://docs.rs/wgpu/latest/wgpu/util/trait.DeviceExt.html) for fixed initialization data.

Readback can use:

- `CommandEncoder::copy_buffer_to_buffer`,
- followed by `Buffer::map_async` on the CPU-visible buffer.

## Shape and Safety Checks

The first GPU evaluator should mirror existing CPU checks:

- `lhs.nrows == rhs.nrows`
- `lhs.ncols == rhs.ncols`
- `lhs.data.len() == rhs.data.len()`

The host should allocate `out` as one flat `Vec<f32>` of length `nrows * ncols`, then rebuild:

```rust
DMatrix::<f32>::from_col_slice(&out_data, nrows, ncols)
```

## Prototype API Recommendation

Keep the operator syntax unchanged and move backend choice to evaluation:

```rust
let expr = &a + &b;
let out = gpu.eval(&expr).await?;
```

or

```rust
let out = expr.eval_with(&gpu).await?;
```

This avoids a large refactor of `Add` implementations while still giving a clean place to introduce GPU behavior.

## What This Prototype Does Not Solve Yet

- fused multi-op expressions,
- scalar broadcasting in GPU codegen,
- reuse of GPU-resident buffers across evaluations,
- matrix-vector multiplication,
- matrix-matrix multiplication,
- fallback policy for unsupported dtypes or missing adapters.

Those belong in the next layer: expression lowering and backend selection.

## Acceptance Criteria for the Add Prototype

- `gpu.add_f32(&a, &b)` matches the CPU result for representative `DMatrix<f32>` inputs.
- The kernel uses one output allocation and one dispatch.
- The implementation does not reorder data relative to the crate's existing column-major storage.
- The API is explicitly opt-in.

## Source Links

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
