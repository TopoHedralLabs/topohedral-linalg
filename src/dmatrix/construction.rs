//! Construction helpers for [`DMatrix`]: zeros, ones, fill, and slice initialisation.
//!
//! Provides factory methods on [`DMatrix<T>`] for the most common initialisation patterns.
//! `zeros` and `ones` fill every element with the additive and multiplicative identities.
//! `from_value` fills with a caller-supplied constant. `from_col_slice` builds a matrix from
//! a flat slice already in column-major order. When the `rand` feature is enabled, additional
//! constructors accept arbitrary distributions from the `rand` crate to produce randomly
//! populated matrices.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::{DMatrix, DVector, VecType};
use crate::common::{Field, One, Zero};
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl: DMatrix<T>
impl<T> DMatrix<T> {
    //{{{ fun: from_col_vec
    /// Creates a matrix by taking ownership of column-major storage.
    ///
    /// # Panics
    ///
    /// Panics if the dimensions overflow or `data.len() != nrows * ncols`.
    pub fn from_col_vec(
        data: Vec<T>,
        nrows: usize,
        ncols: usize,
    ) -> Self {
        let len = nrows
            .checked_mul(ncols)
            .expect("matrix dimensions overflow usize");
        assert_eq!(
            data.len(),
            len,
            "vector length must match matrix dimensions"
        );
        Self { data, nrows, ncols }
    }
    //}}}
    //{{{ fun: from_row_vec
    /// Creates a matrix by taking ownership of row-major values and converting them to
    /// column-major storage.
    ///
    /// # Panics
    ///
    /// Panics if the dimensions overflow or `data.len() != nrows * ncols`.
    pub fn from_row_vec(
        data: Vec<T>,
        nrows: usize,
        ncols: usize,
    ) -> Self {
        let len = nrows
            .checked_mul(ncols)
            .expect("matrix dimensions overflow usize");
        assert_eq!(
            data.len(),
            len,
            "vector length must match matrix dimensions"
        );

        let mut row_major: Vec<Option<T>> = data.into_iter().map(Some).collect();
        let data = (0..ncols)
            .flat_map(|col| (0..nrows).map(move |row| row * ncols + col))
            .map(|index| {
                row_major[index]
                    .take()
                    .expect("each matrix element is moved exactly once")
            })
            .collect();
        Self { data, nrows, ncols }
    }
    //}}}
}

//{{{ impl: DMatrix<T> where T: Clone
impl<T> DMatrix<T>
where
    T: Clone,
{
    //{{{ fun: zeros
    /// Creates a new `DMatrix` initialized with zeros.
    ///
    /// # Panics
    ///
    /// Panics if `nrows * ncols` overflows [`usize`].
    pub fn zeros(
        nrows: usize,
        ncols: usize,
    ) -> Self
    where
        T: Zero,
    {
        let len = nrows
            .checked_mul(ncols)
            .expect("matrix dimensions overflow usize");
        Self {
            data: vec![T::zero(); len],
            nrows,
            ncols,
        }
    }
    //}}}
    //{{{ fun: ones
    /// Creates a new `DMatrix` initialized with ones.
    ///
    /// # Panics
    ///
    /// Panics if `nrows * ncols` overflows [`usize`].
    pub fn ones(
        nrows: usize,
        ncols: usize,
    ) -> Self
    where
        T: One,
    {
        let len = nrows
            .checked_mul(ncols)
            .expect("matrix dimensions overflow usize");
        Self {
            data: vec![T::one(); len],
            nrows,
            ncols,
        }
    }
    //}}}
    //{{{ fun: from_value
    /// Creates a new `DMatrix` initialized with the given value.
    ///
    /// # Panics
    ///
    /// Panics if `nrows * ncols` overflows [`usize`].
    pub fn from_value(
        value: T,
        nrows: usize,
        ncols: usize,
    ) -> Self {
        let len = nrows
            .checked_mul(ncols)
            .expect("matrix dimensions overflow usize");
        Self {
            data: vec![value; len],
            nrows,
            ncols,
        }
    }
    //}}}
    //{{{ fun: from_col_slice
    /// Creates a new `DMatrix` from a slice of values in column-major order.
    ///
    /// # Panics
    ///
    /// Panics if the dimensions overflow or `slice.len() != nrows * ncols`.
    pub fn from_col_slice(
        slice: &[T],
        nrows: usize,
        ncols: usize,
    ) -> Self {
        let len = nrows
            .checked_mul(ncols)
            .expect("matrix dimensions overflow usize");
        assert_eq!(
            slice.len(),
            len,
            "slice length must match matrix dimensions"
        );
        Self {
            data: slice.to_vec(),
            nrows,
            ncols,
        }
    }
    //}}}
    //{{{ fun: from_row_slice
    /// Creates a new `DMatrix` from a slice of values in row-major order.
    ///
    /// # Panics
    ///
    /// Panics if the dimensions overflow or `slice.len() != nrows * ncols`.
    pub fn from_row_slice(
        slice: &[T],
        nrows: usize,
        ncols: usize,
    ) -> Self {
        let len = nrows
            .checked_mul(ncols)
            .expect("matrix dimensions overflow usize");
        assert_eq!(
            slice.len(),
            len,
            "slice length must match matrix dimensions"
        );
        let data = (0..ncols)
            .flat_map(|col| (0..nrows).map(move |row| slice[row * ncols + col].clone()))
            .collect();
        Self { data, nrows, ncols }
    }
    //}}}
    //{{{ fun: from_uniform_random
    /// Creates a new `SMatrix` with elements initialized to random values within the given range.
    ///
    /// Samples each element independently from the half-open range `low..high`.
    ///
    /// # Panics
    ///
    /// Panics if the dimensions overflow or `low..high` is not a valid nonempty uniform range.
    pub fn from_uniform_random(
        low: T,
        high: T,
        nrows: usize,
        ncols: usize,
    ) -> Self
    where
        T: crate::common::UniformRandom + Zero,
    {
        let mut out = Self::zeros(nrows, ncols);
        T::fill_uniform(&mut out.data, low, high);
        out
    }
    //}}}
    //{{{ fun: identity
    /// Creates a new `DMatrix` initialized as the identity matrix.
    ///
    /// The identity matrix is a square matrix with 1s on the main diagonal and 0s elsewhere.
    ///
    /// For a rectangular shape this creates the corresponding rectangular identity, with ones
    /// on the first `min(nrows, ncols)` diagonal positions.
    ///
    /// # Panics
    ///
    /// Panics if `nrows * ncols` overflows [`usize`].
    pub fn identity(
        nrows: usize,
        ncols: usize,
    ) -> Self
    where
        T: One + Zero,
    {
        let mut out = Self::zeros(nrows, ncols);
        let l = nrows.min(ncols);
        for i in 0..l {
            out.data[i + i * nrows] = T::one()
        }
        out
    }
    //}}}
}
//}}}
//{{{ impl: DVector<T>
impl<T> DVector<T>
where
    T: Copy,
{
    /// Creates a new `DVector` initialized with the given value.
    pub fn from_value_vec(
        value: T,
        nelem: usize,
        vec_type: VecType,
    ) -> Self {
        match vec_type {
            VecType::Row => Self::from_value(value, 1, nelem),
            VecType::Col => Self::from_value(value, nelem, 1),
        }
    }

    /// Creates a new `DVector` initialized with zeros.
    pub fn zeros_vec(
        nelem: usize,
        vec_type: VecType,
    ) -> Self
    where
        T: Zero,
    {
        match vec_type {
            VecType::Row => Self::zeros(1, nelem),
            VecType::Col => Self::zeros(nelem, 1),
        }
    }

    /// Creates a new `DVector` initialized with ones.
    pub fn ones_vec(
        nelem: usize,
        vec_type: VecType,
    ) -> Self
    where
        T: One,
    {
        match vec_type {
            VecType::Row => Self::ones(1, nelem),
            VecType::Col => Self::ones(nelem, 1),
        }
    }

    /// Creates a new `DVector` from a slice, copying `nelem` elements into the chosen orientation.
    pub fn from_slice_vec(
        slice: &[T],
        nelem: usize,
        vec_type: VecType,
    ) -> Self
    where
        T: Zero,
    {
        match vec_type {
            VecType::Row => Self::from_col_slice(slice, 1, nelem),
            VecType::Col => Self::from_col_slice(slice, nelem, 1),
        }
    }

    /// Creates a new `DVector` with elements drawn from a uniform random distribution over `[low, high)`.
    pub fn from_uniform_random_vec(
        low: T,
        high: T,
        nelem: usize,
        vec_type: VecType,
    ) -> Self
    where
        T: crate::common::UniformRandom + Field + Copy + Zero,
    {
        match vec_type {
            VecType::Row => Self::from_uniform_random(low, high, 1, nelem),
            VecType::Col => Self::from_uniform_random(low, high, nelem, 1),
        }
    }
}
//}}}
