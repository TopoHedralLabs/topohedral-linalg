//! This module contains functions for constructing SMatrix objects.
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::common::{Field, One, Zero};
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
use rand::distr::{uniform::SampleUniform, Distribution, Uniform};
//}}}
//--------------------------------------------------------------------------------------------------

impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
{
    //{{{ fun: zeros
    pub fn zeros() -> Self
    where
        T: Zero,
    {
        Self {
            data: [T::zero(); N * M],
            nrows: N,
            ncols: M,
        }
    }
    //}}}
    //{{{ fun: ones
    pub fn ones() -> Self
    where
        T: One,
    {
        Self {
            data: [T::one(); N * M],
            nrows: N,
            ncols: M,
        }
    }
    //}}}
    //{{{ fun: from_value
    pub fn from_value(value: T) -> Self
    {
        Self {
            data: [value; N * M],
            nrows: N,
            ncols: M,
        }
    }
    //}}}
    //{{{ fun: from_row_slice
    /// Takes N*M element array in row-major order and creates a new SMatrix
    pub fn from_row_slice(slice: &[T]) -> Self
    where
        T: Zero,
    {
        assert_eq!(slice.len(), N * M);

        let mut out = Self::zeros();

        for j in 0..M
        {
            for i in 0..N
            {
                out.data[j * N + i] = slice[i * M + j];
            }
        }

        out
    }
    //}}}
    //{{{ fun: from_col_slice
    /// Takes N*M element array in column-major order and creates a new SMatrix
    pub fn from_col_slice(slice: &[T]) -> Self
    where
        T: Zero,
    {
        assert_eq!(slice.len(), N * M);
        Self {
            data: slice.try_into().unwrap(),
            nrows: N,
            ncols: M,
        }
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

        for i in 0..N * M
        {
            out.data[i] = range.sample(&mut rng);
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
        T: Field + One + Zero,
    {
        let mut out = Self::zeros();
        let l = N.min(M);
        for i in 0..l
        {
            out.data[i + i * N] = T::one()
        }
        out
    }
    //}}}
}
