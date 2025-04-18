//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::{MatrixOps, Field, Float, One, Zero};
use crate::blaslapack::getrf::Getrf;
use super::SMatrix;
use super::lu;
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

impl<T, const N: usize, const M: usize> MatrixOps for SMatrix<T, N, M>
where
    [(); N * M]:,
    [(); M * N]:,
    T: Field + Zero + One + Copy,
{
    type ScalarType = T;
    type TransposeType = SMatrix<T, M, N>;

    fn size() -> (usize, usize) {
        (N, M)
    }

    fn transpose(&self) -> Self::TransposeType {

        let mut transposed = SMatrix::<T, M, N>::zeros();

        for i in 0..N {
            for j in 0..M {
                transposed[(j, i)] = self[(i, j)];
            }
        }
        transposed
    }

    fn determinant(&self) 
    -> Self::ScalarType
    where 
        Self::ScalarType: Getrf + Float,
    {
        if N != M {
            panic!("Determinant is only defined for square matrices");
        }
        let lu::Return {l, u, p, num_swaps} = self.lu().unwrap();
        (-Self::ScalarType::one()).powi(num_swaps as i32) * u.trace()
    }

    fn trace(&self) -> Self::ScalarType {

        let mut out = Self::ScalarType::one();
        for i in 0..N {
            out *= self[(i, i)];
        }
        out
    }
}