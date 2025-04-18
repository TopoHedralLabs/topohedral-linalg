//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use super::SMatrix;
use crate::blaslapack::getrf::Getrf;
use crate::blaslapack::common::AsI32;
use crate::common::{One, Zero, Field, Complex};
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
pub enum Error {
    #[error("Error in LU, argument {0} is invalid")]
    InvalidArgument(i32), 
    #[error("Error in LU, diagonal element is zero")]
    DiagonalZero,
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
    T: Field + Default + Copy
{
    pub l: SMatrix<T, N, M>,
    pub u: SMatrix<T, N, M>,
    pub p: SMatrix<T, N, M>,
}
//}}}
//{{{ impl SMatrix<T, N, M> 
#[allow(private_bounds)]
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    [(); N * M]:,
    T: One + Zero + Getrf + Field + Default + Copy
{
    pub fn lu(&self) -> Result<Return<T, N, M>, Error>
    {
        //{{{ com: call getrf and check for errors
        let mut a = self.clone();
        let mut ipiv = vec![0; N.min(M)];
        let info = T::getrf(N as i32, M as i32, &mut a.data, N as i32, &mut ipiv);
        if info > 0
        {
            return Err(Error::InvalidArgument(info));
        }
        else if info < 0 
        {
            return Err(Error::DiagonalZero);
        }
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
                    l.data[i + j * N] = a.data[i + j * N];
                }
                else if i == j
                {
                    l.data[i + j * N] = T::one();
                    u.data[i + j * N] = a.data[i + j * N];
                }
                else
                {
                    u.data[i + j * N] = a.data[i + j * N];
                }
            }
        }
        //}}}
        //{{{ com: Create permutation matrix from ipiv
        let mut p = SMatrix::<T, N, M>::identity();
        for (k, &pivot) in ipiv.iter().enumerate()
        {
            let pivot = (pivot - 1) as usize;
            if k != pivot
            {
                for j in 0..M
                {
                    p.data.swap(k + j * N, pivot + j * N);
                }
            }
        }
        //}}}
        Ok(Return{ l, u, p })
    }
}
//}}}