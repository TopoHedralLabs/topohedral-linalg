//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::{
    subviews::{MatrixView, MatrixViewMut},
    DMatrix,
};
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

impl<'a, T> ReduceOps for MatrixView<'a, T>
where
    T: Field + Copy,
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
        for col in 0..self.ncols
        {
            for row in 0..self.nrows
            {
                acc = f(acc, self[(row, col)]);
            }
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
        for col in 0..self.ncols
        {
            for row in 0..self.nrows
            {
                acc = f(acc, (row, col), self[(row, col)]);
            }
        }
        acc
    }
}

impl<'a, T> ReduceOps for MatrixViewMut<'a, T>
where
    T: Field + Copy,
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
        for col in 0..self.ncols
        {
            for row in 0..self.nrows
            {
                acc = f(acc, self[(row, col)]);
            }
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
        for col in 0..self.ncols
        {
            for row in 0..self.nrows
            {
                acc = f(acc, (row, col), self[(row, col)]);
            }
        }
        acc
    }
}
