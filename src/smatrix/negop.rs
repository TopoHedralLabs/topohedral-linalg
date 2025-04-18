//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use super::SMatrix;
use crate::common::{Field, Zero};
//}}}
//{{{ std imports 
use std::ops::Neg;
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
            result[i] = -self[i];
        }   
        result
    }
}
