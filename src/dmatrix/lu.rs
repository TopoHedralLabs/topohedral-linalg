//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::blaslapack::getrf::Getrf;
use crate::blaslapack::getrf;
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
pub enum Error
{
    #[error("Error in lu(), exited with error:\n{0}")]
    GetrfError(#[from] getrf::Error),
}
//}}}
//{{{ struct: Return
/// Represents the LU decomposition of a matrix.
///
/// The LU decomposition is a factorization of a matrix into the product of a lower triangular matrix
/// and an upper triangular matrix. This struct stores the L, U, and permutation matrices resulting
/// from the LU decomposition.
#[derive(Debug)]
pub struct Return<T>
where
    T: Field + Copy,
{
    pub l: DMatrix<T>,
    pub u: DMatrix<T>,
    pub p: DMatrix<T>,
    pub num_swaps: usize,
}
//}}}
//{{{ impl DMatrix<T>
#[allow(private_bounds)]
impl<T> DMatrix<T>
where
    T: One + Zero + Getrf + Field + Copy,
{
    pub fn lu(&self) -> Result<Return<T>, Error>
    {
        let n = self.nrows;
        let m = self.ncols;

        //{{{ com: call getrf and check for errors
        let mut a = self.clone();
        let mut ipiv = vec![0; n.min(m)];
        T::getrf(n as i32, m as i32, &mut a.data, n as i32, &mut ipiv)?;
        //}}}
        //{{{ com: Extract L and U matrices from the factorized matrix
        let mut l = DMatrix::<T>::zeros(n, m);
        let mut u = DMatrix::<T>::zeros(n, m);

        for i in 0..n
        {
            for j in 0..m
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
        let mut p = DMatrix::<T>::identity(n, m);
        let mut num_swaps = 0;
        for (k, &pivot) in ipiv.iter().enumerate()
        {
            let pivot = (pivot - 1) as usize;
            if k != pivot
            {
                for j in 0..m
                {
                    p.data.swap(k + j * n, pivot + j * n);
                    num_swaps += 1;
                }
            }
        }
        //}}}
        Ok(Return { l, u, p, num_swaps })
    }
}
//}}}
