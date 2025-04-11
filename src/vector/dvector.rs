//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

use core::panic;

//{{{ crate imports 
use super::common::DVectorConstructors;
use crate::common::{Field, Zero, One, Float ,VectorOps, FloatVectorOps};
use crate::matrix::dmatrix::DMatrix;
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

pub type DCVector<T> = DMatrix<T>;
pub type DRVector<T> = DMatrix<T>;


impl<T> DVectorConstructors<T> for DRVector<T>
where
    T: Field + Default + Copy + Clone,
{

    
    fn zeros(nelem: usize) -> Self 
    where 
        T: Zero,
    {
        Self { data: vec![T::zero(); nelem], nrows: 1, ncols: nelem}
    }
    
    fn ones(nelem: usize) -> Self 
    where 
        T: One,
    {
        Self { data: vec![T::one(); nelem], nrows: 1, ncols: nelem}
    }
    
    fn from_value(num_elements: usize, value: T) -> Self
    {
        Self { data: vec![value; num_elements], nrows: 1, ncols: num_elements}
    }

    fn from_slice(values: &[T]) -> Self {
        Self {
            data: values.to_vec(),
            nrows: 1,
            ncols: values.len(),
        }
    }
}

impl<T>  VectorOps for DRVector<T>
where
    T: Field + Default + Copy + Clone + Zero + One,
{

    type ScalarType = T;

    fn len(&self) -> usize
    {
        self.ncols * self.nrows
    }
}

impl<T> FloatVectorOps for DRVector<T> 
where
    T: Float + Default + Copy + Clone + Zero + One,
{}