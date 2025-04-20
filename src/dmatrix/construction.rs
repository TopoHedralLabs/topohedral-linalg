//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::common::{Field, One, Zero};
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
use rand::distributions::{uniform::SampleUniform, Distribution, Uniform};
//}}}
//--------------------------------------------------------------------------------------------------

impl<T> DMatrix<T>
where
    T: Field + Copy,
{
    //{{{ fun: zeros
    /// Creates a new `DMatrix` initialized with zeros.
    pub fn zeros(
        nrows: usize,
        ncols: usize,
    ) -> Self
    where
        T: Zero,
    {
        Self {
            data: vec![T::zero(); nrows * ncols],
            nrows,
            ncols,
        }
    }
    //}}}
    //{{{ fun: ones
    /// Creates a new `DMatrix` initialized with ones.
    pub fn ones(
        nrows: usize,
        ncols: usize,
    ) -> Self
    where
        T: One,
    {
        Self {
            data: vec![T::one(); nrows * ncols],
            nrows,
            ncols,
        }
    }
    //}}}
    //{{{ fun: from_value
    /// Creates a new `DMatrix` initialized with the given value.
    pub fn from_value(
        value: T,
        nrows: usize,
        ncols: usize,
    ) -> Self
    {
        Self {
            data: vec![value; nrows * ncols],
            nrows,
            ncols,
        }
    }
    //}}}
    //{{{ fun: from_row_slice
    /// Creates a new `DMatrix` from a slice of values in row-major order.
    pub fn from_col_slice(
        slice: &[T],
        nrows: usize,
        ncols: usize,
    ) -> Self
    where
        T: Zero,
    {
        assert_eq!(slice.len(), nrows * ncols);
        Self {
            data: slice.to_vec(),
            nrows,
            ncols,
        }
    }
    //}}}
    //{{{ fun: from_col_slice
    /// Creates a new `DMatrix` from a slice of values in column-major order.
    pub fn from_row_slice(
        slice: &[T],
        nrows: usize,
        ncols: usize,
    ) -> Self
    where
        T: Zero,
    {
        assert_eq!(slice.len(), nrows * ncols);
        let mut out = Self::zeros(nrows, ncols);
        for i in 0..nrows
        {
            for j in 0..ncols
            {
                out[(i, j)] = slice[i * ncols + j];
            }
        }
        out
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
        nrows: usize,
        ncols: usize,
    ) -> Self
    where
        T: SampleUniform + Zero,
    {
        let mut out = Self::zeros(nrows, ncols);
        let range = Uniform::<T>::new(low, high);
        let mut rng = rand::thread_rng();
        for i in 0..out.nrows * out.ncols
        {
            out[i] = range.sample(&mut rng);
        }
        out
    }
    //}}}
    //{{{ fun: identity
    /// Creates a new `DMatrix` initialized as the identity matrix.
    ///
    /// The identity matrix is a square matrix with 1s on the main diagonal and 0s elsewhere.
    pub fn identity(nrows: usize, ncols: usize) -> Self
    where
        T: Field + One + Zero,
    {
        let mut out = Self::zeros(nrows, ncols);
        let l = nrows.min(ncols);
        for i in 0..l
        {
            out[(i, i)] = T::one()
        }
        out
    }
    //}}}
}
