//! Core matrix operations for [`SMatrix`]: shape, transpose, trace, and determinant.
//!
//! Implements the [`Shape`] and [`MatrixOps`] traits for [`SMatrix<T, N, M>`]. `transpose()`
//! produces an `SMatrix<T, M, N>` with rows and columns exchanged, encoded at the type level
//! through swapped const-generic parameters. `trace()` sums diagonal elements; `determinant()`
//! is derived from the LU factorisation.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::blaslapack::{SLuError, SLuReturn};
use crate::blaslapack::LapackScalar;
use crate::common::Shape;
use crate::common::{Field, MatrixOps, One, SquareMatrixOps, Zero};
use crate::float::Float;
use crate::smatrix::SMatrix;
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl: Shape for SMatrix
impl<T, const N: usize, const M: usize> Shape for SMatrix<T, N, M> {
    fn ncols(&self) -> usize {
        M
    }

    fn nrows(&self) -> usize {
        N
    }
}
//}}}
//{{{ impl: MatrixOps for SMatrix
impl<T, const N: usize, const M: usize> MatrixOps for SMatrix<T, N, M>
where
    T: Field + Zero + One + Copy,
{
    type ScalarType = T;
    type TransposeType = SMatrix<T, M, N>;

    fn transpose(&self) -> Self::TransposeType {
        let mut transposed = SMatrix::<T, M, N>::zeros();

        for i in 0..N {
            for j in 0..M {
                transposed[(j, i)] = self[(i, j)];
            }
        }
        transposed
    }
}
//}}}

//{{{ impl: SquareMatrixOps for SMatrix
impl<T, const N: usize> SquareMatrixOps for SMatrix<T, N, N>
where
    T: Field + Zero + One + Copy,
{
    fn determinant(&self) -> Self::ScalarType
    where
        Self::ScalarType: LapackScalar + Float,
    {
        if N == 0 {
            return Self::ScalarType::one();
        }
        let SLuReturn {
            l: _,
            u,
            p: _,
            num_swaps,
        } = match (*self).lu() {
            Ok(decomposition) => decomposition,
            Err(SLuError::Singular { .. }) => return Self::ScalarType::zero(),
            Err(SLuError::BackendFailure { info }) => {
                panic!("validated LU backend failed with info code {info}")
            }
        };
        let mut diagonal_product = Self::ScalarType::one();
        for i in 0..N {
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
        let mut out = Self::ScalarType::zero();
        for i in 0..N {
            out += self[(i, i)];
        }
        out
    }
}
//}}}
