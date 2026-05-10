//! Reduction operations for [`DMatrix`] and its views.
//!
//! Implements the [`ReduceOps`] trait for [`DMatrix<T>`] and [`MatrixView<T>`]. The two provided
//! methods, `fold` and `fold_indexed`, mirror [`Iterator::fold`] but operate over the matrix
//! elements in column-major order. `fold_indexed` additionally passes the linear element index
//! to the accumulator closure, enabling position-sensitive reductions such as computing the
//! Frobenius norm or finding the index of the maximum element.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::subviews::{MatrixView, MatrixViewMut};
use crate::common::{tuple_index, Field};
use crate::ReduceOps;
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl: ReduceOps for DMatrix
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
//}}}

//{{{ impl: ReduceOps for MatrixView
impl<'a, T> ReduceOps for MatrixView<'a, DMatrix<T>>
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
//}}}

//{{{ impl: ReduceOps for MatrixViewMut
impl<'a, T> ReduceOps for MatrixViewMut<'a, DMatrix<T>>
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
//}}}
