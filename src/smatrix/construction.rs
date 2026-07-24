//! Construction helpers for [`SMatrix`]: zeros, ones, fill, and slice initialisation.
//!
//! Provides factory methods on [`SMatrix<T, N, M>`] for the most common initialisation patterns.
//! `zeros` and `ones` fill every element with the additive and multiplicative identities.
//! `from_value` fills with a caller-supplied constant. `from_col_slice` accepts a flat slice in
//! column-major order; `from_row_slice` accepts row-major input and transposes it to the internal
//! column-major layout. When the `rand` feature is enabled, additional constructors accept
//! distributions from the `rand` crate to produce randomly populated matrices.
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::common::{One, Zero};
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl: SMatrix
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    T: Clone,
{
    //{{{ fun: zeros
    /// Creates a new matrix with all elements set to zero.
    pub fn zeros() -> Self
    where
        T: Zero,
    {
        Self {
            data: std::array::from_fn(|_| std::array::from_fn(|_| T::zero())),
            nrows: N,
            ncols: M,
        }
    }
    //}}}
    //{{{ fun: ones
    /// Creates a new matrix with all elements set to one.
    pub fn ones() -> Self
    where
        T: One,
    {
        Self {
            data: std::array::from_fn(|_| std::array::from_fn(|_| T::one())),
            nrows: N,
            ncols: M,
        }
    }
    //}}}
    //{{{ fun: from_value
    /// Creates a new matrix with every element set to `value`.
    pub fn from_value(value: T) -> Self {
        Self {
            data: std::array::from_fn(|_| std::array::from_fn(|_| value.clone())),
            nrows: N,
            ncols: M,
        }
    }
    //}}}
    //{{{ fun: from_row_slice
    /// Takes N*M element array in row-major order and creates a new SMatrix
    pub fn from_row_slice(slice: &[T]) -> Self {
        let len = N.checked_mul(M).expect("matrix dimensions overflow usize");
        assert_eq!(
            slice.len(),
            len,
            "slice length must match matrix dimensions"
        );

        Self {
            data: std::array::from_fn(|col| {
                std::array::from_fn(|row| slice[row * M + col].clone())
            }),
            nrows: N,
            ncols: M,
        }
    }
    //}}}
    //{{{ fun: from_col_slice
    /// Takes N*M element array in column-major order and creates a new SMatrix
    pub fn from_col_slice(slice: &[T]) -> Self {
        let len = N.checked_mul(M).expect("matrix dimensions overflow usize");
        assert_eq!(
            slice.len(),
            len,
            "slice length must match matrix dimensions"
        );
        Self {
            data: std::array::from_fn(|col| {
                std::array::from_fn(|row| slice[col * N + row].clone())
            }),
            nrows: N,
            ncols: M,
        }
    }
    //}}}
    //{{{ fun: from_col_vec
    /// Builds a static matrix from an owned column-major buffer.
    pub(crate) fn from_col_vec(data: Vec<T>) -> Self {
        let len = N.checked_mul(M).expect("matrix dimensions overflow usize");
        assert_eq!(
            data.len(),
            len,
            "vector length must match matrix dimensions"
        );
        Self::from_col_slice(&data)
    }
    //}}}
    //{{{ fun: from_uniform_random
    /// Creates a new `SMatrix` with elements initialized to random values within the given range.
    ///
    /// Samples each element independently from the half-open range `low..high`.
    ///
    /// # Panics
    ///
    /// Panics if `N * M` overflows [`usize`] or `low..high` is not a valid nonempty uniform
    /// range.
    pub fn from_uniform_random(
        low: T,
        high: T,
    ) -> Self
    where
        T: crate::common::UniformRandom + Zero,
    {
        let mut out = Self::zeros();
        T::fill_uniform(out.as_mut_slice(), low, high);
        out
    }
    //}}}
    //{{{ fun: identity
    /// Creates a new `SMatrix` initialized as the identity matrix.
    ///
    /// The identity matrix is a square matrix with 1s on the main diagonal and 0s elsewhere.
    /// The dimensions of the identity matrix are determined by the generic parameters `N` and `M`.
    pub fn identity() -> Self
    where
        T: One + Zero,
    {
        let mut out = Self::zeros();
        let l = N.min(M);
        for i in 0..l {
            out.data[i][i] = T::one()
        }
        out
    }
    //}}}
}
//}}}
