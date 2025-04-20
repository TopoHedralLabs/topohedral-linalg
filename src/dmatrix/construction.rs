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
    pub fn from_col_slice(
        slice: &[T],
        nrows: usize,
        ncols: usize,
    ) -> Self
    where
        T: Zero,
    {
        assert_eq!(slice.len(), nrows * ncols);

        let mut out = Self::zeros(nrows, ncols);
        out.data.copy_from_slice(slice);
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
    /// Creates a new `SMatrix` initialized as the identity matrix.
    ///
    /// The identity matrix is a square matrix with 1s on the main diagonal and 0s elsewhere.
    /// The dimensions of the identity matrix are determined by the generic parameters `N` and `M`.
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
