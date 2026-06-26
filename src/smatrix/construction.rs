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
use rand::distr::{uniform::SampleUniform, Distribution, Uniform};
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl: SMatrix
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    T: Copy,
{
    //{{{ fun: zeros
    /// Creates a new matrix with all elements set to zero.
    pub fn zeros() -> Self
    where
        T: Zero,
    {
        Self {
            data: [[T::zero(); N]; M],
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
            data: [[T::one(); N]; M],
            nrows: N,
            ncols: M,
        }
    }
    //}}}
    //{{{ fun: from_value
    /// Creates a new matrix with every element set to `value`.
    pub fn from_value(value: T) -> Self {
        Self {
            data: [[value; N]; M],
            nrows: N,
            ncols: M,
        }
    }
    //}}}
    //{{{ fun: from_row_slice
    /// Takes N*M element array in row-major order and creates a new SMatrix
    pub fn from_row_slice(slice: &[T]) -> Self {
        assert_eq!(slice.len(), N * M);

        Self {
            data: std::array::from_fn(|col| std::array::from_fn(|row| slice[row * M + col])),
            nrows: N,
            ncols: M,
        }
    }
    //}}}
    //{{{ fun: from_col_slice
    /// Takes N*M element array in column-major order and creates a new SMatrix
    pub fn from_col_slice(slice: &[T]) -> Self {
        assert_eq!(slice.len(), N * M);
        Self {
            data: std::array::from_fn(|col| std::array::from_fn(|row| slice[col * N + row])),
            nrows: N,
            ncols: M,
        }
    }
    //}}}
    //{{{ fun: from_col_vec
    pub(crate) fn from_col_vec(data: Vec<T>) -> Self {
        assert_eq!(data.len(), N * M);
        Self::from_col_slice(&data)
    }
    //}}}
    //{{{ fun: from_uniform_random
    /// Creates a new `SMatrix` with elements initialized to random values within the given range.
    ///
    /// The `low` and `high` parameters specify the inclusive range of the random values.
    /// The matrix is initialized using a uniform random distribution.
    pub fn from_uniform_random(
        low: T,
        high: T,
    ) -> Self
    where
        T: SampleUniform + Zero,
    {
        let mut out = Self::zeros();

        let range = Uniform::<T>::new(low, high).unwrap();

        let mut rng = rand::rng();

        for value in out.as_mut_slice() {
            *value = range.sample(&mut rng);
        }

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
            out[(i, i)] = T::one()
        }
        out
    }
    //}}}
}
//}}}
