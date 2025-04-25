//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::lu;
use super::DMatrix;
use crate::blaslapack::getrf::Getrf;
use crate::common::{Field, Float, MatrixOps, One, Zero};
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

impl<T> MatrixOps for DMatrix<T>
where
    T: Field + Zero + One + Copy,
{
    type ScalarType = T;
    type TransposeType = DMatrix<T>;

    fn size(&self) -> (usize, usize)
    {
        (self.nrows, self.ncols)
    }

    fn transpose(&self) -> Self::TransposeType
    {
        let mut transposed = DMatrix::<T>::zeros(self.ncols, self.nrows);

        for i in 0..self.nrows
        {
            for j in 0..self.ncols
            {
                transposed[(j, i)] = self[(i, j)];
            }
        }
        transposed
    }

    fn determinant(&self) -> Self::ScalarType
    where
        Self::ScalarType: Getrf + Float,
    {
        if self.nrows != self.ncols
        {
            panic!("Determinant is only defined for square matrices");
        }
        let lu::Return {
            l: _,
            u,
            p: _,
            num_swaps,
        } = self.lu().unwrap();
        (-Self::ScalarType::one()).powi(num_swaps as i32) * u.trace()
    }

    fn trace(&self) -> Self::ScalarType
    {
        let mut out = Self::ScalarType::one();
        for i in 0..self.nrows
        {
            out *= self[(i, i)];
        }
        out
    }
}
