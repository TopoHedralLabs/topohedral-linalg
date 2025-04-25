//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::common::{Field, Zero};
//}}}
//{{{ std imports
use std::ops::Neg;
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

impl<T> Neg for DMatrix<T>
where
    T: Field + Zero + Default + Neg<Output = T> + Copy,
{
    type Output = DMatrix<T>;

    fn neg(self) -> Self
    {
        let mut result = self.clone();
        for i in 0..self.nrows * self.ncols
        {
            result[i] = -self[i];
        }
        result
    }
}
