//! Reduction operations for [`SMatrix`] and its views.
//!
//! Implements the [`ReduceOps`] trait for [`SMatrix<T, N, M>`] and [`MatrixView<T, N, M>`].
//! `fold` reduces all elements with an accumulator closure; `fold_indexed` additionally passes
//! the linear element index to the closure, enabling position-sensitive reductions such as
//! computing the Frobenius norm or finding the index of the maximum element. Both methods
//! traverse elements in column-major order.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::common::tuple_index;
use crate::subviews::{MatrixView, MatrixViewMut};
use crate::ReduceOps;
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl: ReduceOps for SMatrix
impl<T, const N: usize, const M: usize> ReduceOps for SMatrix<T, N, M>
where
    T: Copy,
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
        for &value in self.as_slice()
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
        for (linear_index, &value) in self.as_slice().iter().enumerate()
        {
            acc = f(acc, tuple_index(linear_index, N), value);
        }
        acc
    }
}
//}}}
//{{{ impl: ReduceOps for MatrixView
impl<'a, T, const N: usize, const M: usize> ReduceOps for MatrixView<'a, SMatrix<T, N, M>>
where
    T: Copy,
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
impl<'a, T, const N: usize, const M: usize> ReduceOps for MatrixViewMut<'a, SMatrix<T, N, M>>
where
    T: Copy,
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
