//! Dynamic vector type built on [`DMatrix`], with row and column vector orientations.
//!
//! Defines the `DVector<T>` type alias (`DMatrix<T>` with a single column or row) along with the
//! `VecType` discriminant that selects between `Row` and `Col` orientation. Factory methods —
//! `zeros_cvec`, `ones_cvec`, `from_slice_vec`, `from_value_vec` — construct column vectors
//! directly. When the element type implements [`Float`], the module also implements
//! `FloatVectorOps`, which adds dot products, norms, and other vector-specific operations built
//! on top of the underlying [`DMatrix`] arithmetic and BLAS routines.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::common::{Field, Float, FloatVectorOps, One, VectorOps, Zero};
use crate::dmatrix::DMatrix;
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
use rand::distr::uniform::SampleUniform;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ type: DVector
/// A dynamic vector stored as a single-row or single-column [`DMatrix`].
pub type DVector<T> = DMatrix<T>;
//}}}

//{{{ enum: VecType
/// Selects whether a `DVector` is oriented as a row vector or a column vector.
pub enum VecType
{
    /// A 1×N row vector.
    Row,
    /// An N×1 column vector.
    Col,
}
//}}}

//{{{ impl: DVector<T>
impl<T> DVector<T>
where
    T: Field + Copy,
{
    /// Creates a new `DVector` initialized with the given value.
    pub fn from_value_vec(
        value: T,
        nelem: usize,
        vec_type: VecType,
    ) -> Self
    {
        match vec_type
        {
            VecType::Row => Self::from_value(value, 1, nelem),
            VecType::Col => Self::from_value(value, nelem, 1),
        }
    }

    /// Creates a new `DVector` initialized with zeros.
    pub fn zeros_cvec(
        nelem: usize,
        vec_type: VecType,
    ) -> Self
    where
        T: Zero,
    {
        match vec_type
        {
            VecType::Row => Self::zeros(1, nelem),
            VecType::Col => Self::zeros(nelem, 1),
        }
    }

    /// Creates a new `DVector` initialized with ones.
    pub fn ones_cvec(
        nelem: usize,
        vec_type: VecType,
    ) -> Self
    where
        T: One,
    {
        match vec_type
        {
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
        match vec_type
        {
            VecType::Row => Self::from_col_slice(slice, 1, nelem),
            VecType::Col => Self::from_col_slice(slice, nelem, 1),
        }
    }

    /// Creates a new `DVector` with elements drawn from a uniform random distribution over `[low, high)`.
    pub fn from_uniform_random_cvec(
        low: T,
        high: T,
        nelem: usize,
        vec_type: VecType,
    ) -> Self
    where
        T: SampleUniform + Field + Copy + Zero,
    {
        match vec_type
        {
            VecType::Row => Self::from_uniform_random(low, high, 1, nelem),
            VecType::Col => Self::from_uniform_random(low, high, nelem, 1),
        }
    }
}
//}}}

//{{{ impl: VectorOps for DVector<T>
impl<T> VectorOps for DVector<T>
where
    T: Field + Default + Copy + Clone + Zero + One + Float,
{
    type ScalarType = T;

    fn len(&self) -> usize
    {
        if self.nrows != 1 && self.ncols != 1
        {
            panic!("Vector must be either a row or column vector");
        }

        if self.nrows == 1
        {
            self.ncols
        }
        else
        {
            self.nrows
        }
    }
}
//}}}

//{{{ impl: FloatVectorOps for DVector<T>
impl<T> FloatVectorOps for DVector<T> where T: Float + Default + Copy + Clone + Zero + One {}
//}}}
