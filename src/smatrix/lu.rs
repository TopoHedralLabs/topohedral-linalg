//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::blaslapack::getrf;
use crate::blaslapack::getrf::Getrf;
use crate::common::{Field, One, Zero};
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: LUError
/// Errors that can occur during LU decomposition.
///
/// The `LUError` enum represents the different types of errors that can occur during the LU decomposition
/// of a matrix. The `InvalidArgument` variant indicates that one of the arguments passed to the LU
/// decomposition function was invalid, while the `DiagonalZero` variant indicates that the diagonal
/// element of the matrix became zero during the decomposition, which is not allowed.
#[derive(Error, Debug)]
pub enum Error
{
    #[error("Error in lu(), exited with error:\n{0}")]
    GetrfError(#[from] getrf::Error),
}
//}}}
//{{{ sturct: Return
/// Represents the LU decomposition of a matrix.
///
/// The LU decomposition is a factorization of a matrix into the product of a lower triangular matrix
/// and an upper triangular matrix. This struct stores the L, U, and permutation matrices resulting
/// from the LU decomposition.
pub struct Return<T, const N: usize, const M: usize>
where
    [(); N * M]:,
    T: Field + Copy,
{
    pub l: SMatrix<T, N, M>,
    pub u: SMatrix<T, N, M>,
    pub p: SMatrix<T, N, M>,
    pub num_swaps: usize,
}
//}}}
//{{{ impl SMatrix<T, N, M>
#[allow(private_bounds)]
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    [(); N * M]:,
    T: One + Zero + Getrf + Field + Copy,
{
    pub fn lu(&self) -> Result<Return<T, N, M>, Error>
    {
        //{{{ com: call getrf and check for errors
        let mut a = *self;
        let mut ipiv = vec![0; N.min(M)];
        T::getrf(N as i32, M as i32, &mut a.data, N as i32, &mut ipiv)?;
        //}}}
        //{{{ com:  Extract L and U matrices from the factorized matrix
        let mut l = SMatrix::<T, N, M>::zeros();
        let mut u = SMatrix::<T, N, M>::zeros();

        for i in 0..N
        {
            for j in 0..M
            {
                if i > j
                {
                    l[(i, j)] = a[(i, j)];
                }
                else if i == j
                {
                    l[(i, j)] = T::one();
                    u[(i, j)] = a[(i, j)];
                }
                else
                {
                    u[(i, j)] = a[(i, j)];
                }
            }
        }
        //}}}
        //{{{ com: Create permutation matrix from ipiv
        let mut p = SMatrix::<T, N, M>::identity();
        let mut num_swaps = 0;
        for (k, &pivot) in ipiv.iter().enumerate()
        {
            let pivot = (pivot - 1) as usize;
            if k != pivot
            {
                for j in 0..M
                {
                    p.data.swap(k + j * N, pivot + j * N);
                    num_swaps += 1;
                }
            }
        }
        //}}}
        Ok(Return { l, u, p, num_swaps })
    }
}
//}}}
