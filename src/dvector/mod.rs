//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::dmatrix::DMatrix;
use crate::common::{Field, One, Zero, VectorOps, FloatVectorOps, Float};
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
use rand::distributions::uniform::SampleUniform;
//}}}
//--------------------------------------------------------------------------------------------------


pub type DVector<T> = DMatrix<T>;

pub enum VecType {
    Row,
    Col,
}

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
        match vec_type {
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
        match vec_type {
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
        match vec_type {
            VecType::Row => Self::ones(1, nelem),
            VecType::Col => Self::ones(nelem, 1),
        }
    }

    pub fn from_slice_vec(
        slice: &[T],
        nelem: usize,
        vec_type: VecType,
    ) -> Self
    where 
        T: Zero
    {
        match vec_type {
            VecType::Row => Self::from_col_slice(slice, 1, nelem),
            VecType::Col => Self::from_col_slice(slice, nelem, 1),
        }
    }

    pub fn from_uniform_random_cvec(
        low: T, 
        high: T, 
        nelem: usize, 
        vec_type: VecType,
    ) -> Self
    where
        T: SampleUniform + Field + Copy + Zero,
    {
        match vec_type {
            VecType::Row => Self::from_uniform_random(low, high, 1, nelem),
            VecType::Col => Self::from_uniform_random(low, high, nelem, 1),
        }
    }
}

impl<T> VectorOps for DVector<T>
where
    T: Field + Default + Copy + Clone + Zero + One,
{
    type ScalarType = T;

    fn len(&self) -> usize
    {
        if self.nrows != 1 && self.ncols != 1 {
            panic!("Vector must be either a row or column vector");
        }

        if self.nrows == 1 {
            self.ncols
        } else {
            self.nrows
        }
    }
}

impl<T> FloatVectorOps for DVector<T>
where 
    T: Float + Default + Copy + Clone + Zero + One,
{

}