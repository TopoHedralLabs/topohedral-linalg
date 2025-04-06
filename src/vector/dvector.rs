//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

use core::panic;

//{{{ crate imports 
use crate::common::{Field, Zero, One, Float};
use crate::matrix::dmatrix::DMatrix;
use super::common::{VectorOps, FloatVectorOps, DVectorConstructors};
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

    fn from_value(num_elements: usize, value: T) -> Self
    {
        Self { data: vec![value; num_elements], nrows: 1, ncols: num_elements}
    }
    
    fn zeros(nelem: usize) -> Self {
        todo!()
    }
    
    fn ones(nelem: usize) -> Self {
        todo!()
    }
    
    fn from_slice(values: &[T]) -> Self {
        todo!()
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