# DMatrix Vectorisation: Diagnosis and Fix

## Background

The `DMatrix` benchmark adds nine matrices together using a lazy expression tree:

```rust
let j: DMatrix<f64> = (&a + &b + &c + &d + &e + &f + &g + &h + &i).into();
```

Each `+` operator builds a zero-cost `BinopExpr` node (no computation, just stores
references). The `.into()` call triggers a single `From<BinopExpr>::from` that walks
the tree and writes results into a freshly allocated output matrix.

Despite this theoretically optimal structure — one allocation, one pass — the
original implementation was **2–3× slower than nalgebra** and matched only raw scalar
`Vec` performance.

---

## The Problem: LLVM Could Not Vectorise

The original evaluation loop in `From<BinopExpr>`:

```rust
for i in 0..total {
    out.data[i] = expr.index_value(i);
}
```

looks like a candidate for SIMD vectorisation: a counted loop, contiguous memory,
no control flow. Yet the generated assembly on aarch64 was entirely scalar:

```asm
LBB_loop:
    ldr x3, [x10, #8]          ; reload Vec.ptr from DMatrix struct — EVERY ITERATION
    ldr d0, [x3, x11, lsl #3]  ; load data[i] through that pointer
    ldr x3, [x12, #8]          ; reload for matrix b
    ldr d1, [x3, x11, lsl #3]
    fadd d0, d0, d1             ; scalar add — 1 double per iteration
    ...
    str d0, [x8, x11, lsl #3]  ; write output
```

Two distinct problems are visible here.

### Problem 1 — Bounds Checks

`out.data[i]` and each `self.data[index]` in `index_value` compile to a checked Vec
index. With nine input matrices there are ten runtime bounds checks per element (nine
reads + one write). Each check branches on a length loaded from a heap-allocated Vec
struct. LLVM cannot prove these lengths are constant across iterations, so it cannot
hoist, unroll, or vectorise the loop.

### Problem 2 — Aliasing (the deeper issue)

Even with bounds checks removed, LLVM still reloaded `[x10, #8]` — the Vec's
internal data pointer — on every iteration. The reason:

`DMatrix<T>` stores its data in a `Vec<T>`. To read `data[i]`, LLVM must first load
the Vec's raw heap pointer from the DMatrix struct, then dereference it at offset `i`.
The write destination (`out.data[i]`) is reached through the same kind of indirection.

LLVM asks: *can the write to `out.data[i]` modify the memory at `[x10 + 8]` (the Vec
pointer field of an input DMatrix)?* Both are heap allocations. Without being able to
prove they are disjoint, LLVM conservatively reloads every pointer every iteration —
and a loop with nine unprovable pointer reloads per element cannot be vectorised.

This is a failure of **alias analysis**, not arithmetic. The data itself is perfectly
suited to SIMD; LLVM simply cannot prove the memory regions do not overlap.

---

## Why nalgebra Was Faster

nalgebra evaluates `&a + &b + ... + &i` eagerly and pairwise. `&Matrix + &Matrix`
allocates a new `Matrix` and fills it; subsequent additions consume the accumulator
in-place via `Matrix + &Matrix`. Each individual addition is a simple two-slice zip:

```asm
LBB_nalgebra:
    ldp q0, q1, [x15, #-32]   ; load 4 doubles from input
    ldp q2, q3, [x15], #64
    ldp q4, q5, [x14, #-32]   ; load 4 doubles from accumulator
    ldp q6, q7, [x14]
    fadd.2d v0, v0, v4         ; 2 doubles at once × 4 = 8 doubles per iteration
    fadd.2d v1, v1, v5
    fadd.2d v2, v2, v6
    fadd.2d v3, v3, v7
    stp q0, q1, [x14, #-32]
    stp q2, q3, [x14], #64
    subs x16, x16, #8
    b.ne LBB_nalgebra
```

Because each call to `Add::add` takes two `&Matrix` parameters — annotated `noalias
readonly` in LLVM IR — and writes to a freshly allocated `*mut f64`, LLVM trivially
proves non-aliasing and generates fully vectorised code.

The cost: nalgebra makes eight passes over the data and moves roughly 2 400 doubles in
total for a 10×10 addition chain. The lazy tree approach should need only one pass and
move only ~1 000 doubles — but only if it can be vectorised.

---

## The Fix: Three Changes

### Fix 1 — `get_unchecked` on Leaf Reads

**File:** [`src/dmatrix/indexing.rs`](https://github.com/TopoHedralLabs/topohedral-linalg/blob/main/src/dmatrix/indexing.rs)

```rust
// Before
fn index_value(&self, index: usize) -> Self::Output {
    self.data[index]
}

// After
fn index_value(&self, index: usize) -> Self::Output {
    unsafe { *self.data.get_unchecked(index) }
}
```

Removing the ten bounds checks per element is necessary but not sufficient. The
expression tree is constructed with matching dimensions (asserted at build time) and
`From<BinopExpr>` iterates exactly `0..nrows*ncols`, so the invariant holds.

This alone produced a measurable improvement (from ~313 ns to a lower figure) but
still left LLVM generating scalar code due to the aliasing problem.

### Fix 2 — The `EvalInto<T>` Trait (the key change)

**Files:** [`src/common.rs`](https://github.com/TopoHedralLabs/topohedral-linalg/blob/main/src/common.rs), [`src/expression/binary_expr.rs`](https://github.com/TopoHedralLabs/topohedral-linalg/blob/main/src/expression/binary_expr.rs), [`src/dmatrix/mod.rs`](https://github.com/TopoHedralLabs/topohedral-linalg/blob/main/src/dmatrix/mod.rs)

The root cause of the aliasing failure was **where the output was written through**:

```rust
// OLD — write through Vec internals; LLVM loses noalias provenance
unsafe { *out.data.get_unchecked_mut(i) = expr.index_value(i) }
```

`out.data` is a `Vec<T>`. LLVM loads the Vec's raw pointer from a struct field and
writes through it. That raw `*mut T` carries no `noalias` annotation, so LLVM cannot
rule out aliasing with the input DMatrix structs.

The fix is a new trait:

```rust
pub trait EvalInto<T: Field + Copy> {
    fn eval_into(&self, out: &mut [T]);
}
```

A Rust `&mut [T]` slice reference is annotated `noalias` in LLVM IR. This is the
language's aliasing guarantee: a mutable reference cannot coexist with any other
reference to the same memory. The nine input DMatrices are accessed through `&DMatrix`
(annotated `noalias readonly`). LLVM therefore knows:

- The output slice **cannot overlap** with any input DMatrix struct.
- Writing to `out[i]` **cannot modify** the Vec pointer fields inside any DMatrix.
- Those Vec pointer loads can be **hoisted once**, before the loop body.

`From<BinopExpr>` is updated to call `eval_into` rather than indexing through `out.data`:

```rust
fn from(expr: BinopExpr<A, B, T, Op>) -> DMatrix<T> {
    let nrows = expr.nrows;
    let ncols = expr.ncols;
    let total = nrows * ncols;
    let mut data: Vec<T> = Vec::with_capacity(total);
    unsafe { data.set_len(total) };
    expr.eval_into(&mut data);   // out: &mut [T] — noalias parameter
    DMatrix { data, nrows, ncols }
}
```

### Fix 3 — Single-Pass Evaluation

**File:** [`src/expression/binary_expr.rs`](https://github.com/TopoHedralLabs/topohedral-linalg/blob/main/src/expression/binary_expr.rs)

With aliasing now provable, the natural implementation of `BinopExpr::eval_into` uses
`index_value` for **both** operands in a single loop:

```rust
fn eval_into(&self, out: &mut [T]) {
    let len = out.len();
    for i in 0..len {
        unsafe {
            *out.get_unchecked_mut(i) =
                Op::apply(self.a.index_value(i), self.b.index_value(i));
        }
    }
}
```

For the full eight-level nested `BinopExpr` representing nine matrices, LLVM inlines
all the `#[inline]` `index_value` calls and sees the flattened computation:

```
out[i] = a[i] + b[i] + c[i] + d[i] + e[i] + f[i] + g[i] + h[i] + i[i]
```

With all nine reads proven `noalias readonly` and the write proven `noalias`, LLVM
hoists all nine Vec data pointers into registers before the loop and generates a
**single vectorised loop**:

```asm
; All 9 data pointers loaded ONCE, outside the loop:
ldr x10, [x10, #8]   ; DMatrix_a.data.ptr
ldr x11, [x11, #8]   ; DMatrix_b.data.ptr
...

; One SIMD loop — 2 doubles per iteration:
LBB_11:
    ldr q0, [x3], #16    ; load 2 doubles from a, advance pointer
    ldr q1, [x4], #16    ; load 2 doubles from b
    fadd.2d v0, v0, v1   ; a + b
    ldr q1, [x5], #16    ; load from c
    fadd.2d v0, v0, v1   ; + c
    ldr q1, [x6], #16
    fadd.2d v0, v0, v1   ; + d
    ...                  ; 8 adds total
    str q0, [x26], #16   ; store 2 doubles to output
    subs x27, x27, #2
    b.ne LBB_11
```

---

## Why This Beats nalgebra

| Approach | Passes | Doubles moved (10×10) | Loop structure |
|---|---|---|---|
| nalgebra | 8 | ~2 400 | 8 × 2-input SIMD (8 doubles/iter) |
| topohedral (final) | 1 | ~1 000 | 1 × 9-input SIMD (2 doubles/iter) |

nalgebra's pairwise approach re-reads and re-writes the accumulator eight times. The
lazy expression tree describes the whole computation at once, so after the aliasing
proof is in place LLVM can fuse all nine operands into a single scan through memory.

The per-iteration instruction mix is different — nalgebra unrolls to 8 doubles per
SIMD iteration while ours processes 2 — but the dominant cost at these matrix sizes
is **memory bandwidth, not arithmetic throughput**. Moving 2.4× less data wins.

### Benchmark Results

| Size | Before | After | nalgebra | raw `Vec` |
|---|---|---|---|---|
| 10×10 | 313 ns | **91 ns** | 149 ns | 312 ns |
| 20×20 | 1.2 µs | **242 ns** | 395 ns | 1.19 µs |
| 30×30 | 2.7 µs | **526 ns** | 889 ns | 2.68 µs |
| 40×40 | 4.9 µs | **950 ns** | 1.58 µs | 4.75 µs |

Final result: **1.6–1.7× faster than nalgebra**, **3.5–5× faster than raw `Vec`**.

---

## Summary

The lazy expression tree was never the problem — it was always the right design.
The issue was entirely in how LLVM received information about memory ownership:

| Step | What changed | Why it mattered |
|---|---|---|
| `get_unchecked` | Removed 10 bounds checks per element | Unblocked loop analysis |
| `EvalInto<T>` + `&mut [T]` | Output goes through a `noalias` parameter | Proved output ∩ inputs = ∅; Vec ptrs hoistable |
| Single-pass `eval_into` | Both operands read via `index_value` in one loop | One memory pass instead of N−1 fold passes |
