//! Core matrix operations for [`DMatrix`]: shape, transpose, trace, and determinant.
//!
//! Implements the [`Shape`] and [`MatrixOps`] traits for [`DMatrix<T>`]. [`Shape`] exposes
//! runtime `nrows` and `ncols`. [`MatrixOps`] adds `transpose()`, which produces a new matrix
//! with rows and columns exchanged, `trace()`, computed as the sum of diagonal elements, and
//! `determinant()`, derived from the LU factorisation via the product of diagonal entries of U
//! scaled by the sign of the permutation.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::blaslapack::DLuReturn;
use crate::blaslapack::Getrf;
use crate::common::Shape;
use crate::common::{Field, MatrixOps, One, Zero};
use crate::dmatrix::DMatrix;
use crate::float::Float;
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl: Shape for DMatrix
impl<T> Shape for DMatrix<T>
where
    T: Copy,
{
    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}
//}}}

//{{{ impl: MatrixOps for DMatrix
impl<T> MatrixOps for DMatrix<T>
where
    T: Field + Zero + One + Copy,
{
    type ScalarType = T;
    type TransposeType = DMatrix<T>;

    fn transpose(&self) -> Self::TransposeType {
        let mut transposed = DMatrix::<T>::zeros(self.ncols, self.nrows);

        for i in 0..self.nrows {
            for j in 0..self.ncols {
                transposed[(j, i)] = self[(i, j)];
            }
        }
        transposed
    }

    fn determinant(&self) -> Self::ScalarType
    where
        Self::ScalarType: Getrf + Float,
    {
        if self.nrows != self.ncols {
            panic!("Determinant is only defined for square matrices");
        }
        let DLuReturn {
            l: _,
            u,
            p: _,
            num_swaps,
        } = self.lu().unwrap();
        (-Self::ScalarType::one()).powi(num_swaps as i32) * u.trace()
    }

    fn trace(&self) -> Self::ScalarType {
        let mut out = Self::ScalarType::one();
        for i in 0..self.nrows {
            out *= self[(i, i)];
        }
        out
    }
}
//}}}
