//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 

use crate::common::*;
use super::super::common::{DMatrixConstructors, SMatrixConstructors};
use crate::matrix::smatrix::SMatrix;
use crate::matrix::dmatrix::DMatrix;
//}}}
//{{{ std imports 
use std::ops::Neg;
use std::fmt;
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------


impl<T, const N: usize, const M: usize> Neg for SMatrix<T, N, M>
where 
    [(); N * M]:,
    T: Field + Zero + Default + Neg<Output = T> + Copy
{
    type Output = SMatrix<T, N, M>;

    fn neg(self) -> Self {
        let mut result = SMatrix::<T, N, M>::zero();
        for i in 0..N*M {
            result.data[i] = -self.data[i];
        }   
        result
    }
}

impl<T> Neg for DMatrix<T>
where
    T: Field + Zero + Default + Neg<Output = T> + Copy
{
    type Output = DMatrix<T>;

    fn neg(self) -> Self {
        let mut result = DMatrix::<T>::zeros(self.nrows, self.ncols);
        for i in 0..self.nrows*self.ncols {
            result.data[i] = -self.data[i];
        }
        result
    }
}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{
}
//}}}
