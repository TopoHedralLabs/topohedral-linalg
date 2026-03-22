//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::common::{tuple_index, Field};
use crate::ReduceOps;
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

impl<T: Field + Copy> ReduceOps for DMatrix<T>
{
    type Item = T;
    type Index = (usize, usize);

    fn fold<B, F>(
        &self,
        init: B,
        mut f: F,
    ) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        let mut acc = init;
        for &value in &self.data
        {
            acc = f(acc, value);
        }
        acc
    }

    fn fold_indexed<B, F>(
        &self,
        init: B,
        mut f: F,
    ) -> B
    where
        F: FnMut(B, Self::Index, Self::Item) -> B,
    {
        let mut acc = init;
        for (linear_index, &value) in self.data.iter().enumerate()
        {
            acc = f(acc, tuple_index(linear_index, self.nrows), value);
        }
        acc
    }
}
