//! Core matrix operations for [`DMatrix`]: shape, transpose, trace, and determinant.
//!
//! Implements the [`Shape`] and [`MatrixOps`] traits for [`DMatrix<T>`]. [`Shape`] exposes
//! runtime `nrows` and `ncols`. [`MatrixOps`] adds `transpose()`, which produces a new matrix
//! with rows and columns exchanged, `trace()`, computed as the sum of diagonal elements, and
//! `determinant()`, derived from the LU factorisation via the product of diagonal entries of U
//! scaled by the sign of the permutation.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::blaslapack::{DLuError, DLuReturn};
use crate::blaslapack::LapackScalar;
use crate::common::Shape;
use crate::common::{Field, MatrixOps, One, SquareMatrixOps, Zero};
use crate::dmatrix::DMatrix;
use crate::float::Float;
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl: Shape for DMatrix
impl<T> Shape for DMatrix<T> {
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
}
//}}}

//{{{ impl: SquareMatrixOps for DMatrix
impl<T> SquareMatrixOps for DMatrix<T>
where
    T: Field + Zero + One + Copy,
{
    fn determinant(&self) -> Self::ScalarType
    where
        Self::ScalarType: LapackScalar + Float,
    {
        if self.nrows != self.ncols {
            panic!("determinant is only defined for square matrices");
        }
        if self.nrows == 0 {
            return Self::ScalarType::one();
        }
        let DLuReturn {
            l: _,
            u,
            p: _,
            num_swaps,
        } = match self.clone().lu() {
            Ok(decomposition) => decomposition,
            Err(DLuError::Singular { .. }) => return Self::ScalarType::zero(),
            Err(DLuError::BackendFailure { info }) => {
                panic!("validated LU backend failed with info code {info}")
            }
        };
        let mut diagonal_product = Self::ScalarType::one();
        for i in 0..self.nrows {
            diagonal_product *= u[(i, i)];
        }
        let sign = if num_swaps % 2 == 0 {
            Self::ScalarType::one()
        } else {
            -Self::ScalarType::one()
        };
        sign * diagonal_product
    }

    fn trace(&self) -> Self::ScalarType {
        assert_eq!(
            self.nrows, self.ncols,
            "trace is only defined for square matrices"
        );
        let mut out = Self::ScalarType::zero();
        for i in 0..self.nrows {
            out += self[(i, i)];
        }
        out
    }
}
//}}}
