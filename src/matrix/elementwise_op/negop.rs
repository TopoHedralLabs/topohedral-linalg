//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::*;
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
    use super::*;

    #[test]
    fn test_neg_smatrix() {

        let a  = SMatrix::<i32, 2, 2>::from_slice_row(&[ 1, 2, 3, 4]);
        let b = -a.clone(); 

        for i in 0..4 {
            assert_eq!(b[i], -a[i]);
        }
    }
    #[test]
    fn test_neg_dmatrix() {

        let a  = DMatrix::<i32>::from_slice_row(2,2, &[ 1, 2, 3, 4]);
        let b = -a.clone(); 

        for i in 0..4 {
            assert_eq!(b[i], -a[i]);
        }
    }
}
//}}}
