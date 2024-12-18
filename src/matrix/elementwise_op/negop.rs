//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::*;
use crate::matrix::smatrix::SMatrix;
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
        let mut result = SMatrix::<T, N, M>::zeros();
        for i in 0..N*M {
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
    fn test_neg() {

        let a  = SMatrix::<i32, 2, 2>::from_slice_row(&[ 1, 2, 3, 4]);
        let b = -a.clone(); 

        for i in 0..4 {
            assert_eq!(b[i], -a[i]);
        }
    }
}
//}}}
