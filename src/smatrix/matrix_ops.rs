//! Core matrix operations for [`SMatrix`]: shape, transpose, trace, and determinant.
//!
//! Implements the [`Shape`] and [`MatrixOps`] traits for [`SMatrix<T, N, M>`]. `transpose()`
//! produces an `SMatrix<T, M, N>` with rows and columns exchanged, encoded at the type level
//! through swapped const-generic parameters. `trace()` sums diagonal elements; `determinant()`
//! is derived from the LU factorisation.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::blaslapack::SLuReturn;
use crate::blaslapack::Getrf;
use crate::common::Shape;
use crate::common::{Field, MatrixOps, One, Zero};
use crate::float::Float;
use crate::smatrix::SMatrix;
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl: Shape for SMatrix
impl<T, const N: usize, const M: usize> Shape for SMatrix<T, N, M>
where
    T: Copy,
{
    fn ncols(&self) -> usize
    {
        M
    }

    fn nrows(&self) -> usize
    {
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

    fn transpose(&self) -> Self::TransposeType
    {
        let mut transposed = SMatrix::<T, M, N>::zeros();

        for i in 0..N
        {
            for j in 0..M
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
        if N != M
        {
            panic!("Determinant is only defined for square matrices");
        }
        let SLuReturn {
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
        for i in 0..N
        {
            out *= self[(i, i)];
        }
        out
    }
}
//}}}
