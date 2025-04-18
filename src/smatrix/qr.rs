//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::{Field, One, Zero};
use crate::blaslapack::geqrf::Geqrf;
use crate::blaslapack::orgqr::Orgqr;
use crate::blaslapack::common::AsI32;
use crate::smatrix::SMatrix;
//}}}
//{{{ std imports 
use std::ops::{Index, IndexMut};
//}}}
//{{{ dep imports 
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: QRError
#[derive(Error, Debug)]
pub enum Error {
    #[error("Error in QR, argument {0} is invalid")]
    InvalidArgument(i32),
    #[error("Error in QR decomposition, exited with code {0}")]
    LapackError(i32),
}
//}}}
//{{{ struct: Return
pub struct Return<T, const N: usize, const M: usize>
where
    [(); N * M]:,
    T: Field + Copy 
{
    pub q: SMatrix<T, N, M>,
    pub r: SMatrix<T, N, M>,
}
//}}}
//{{{ impl: SMatrix<T, N, M>
#[allow(private_bounds)]
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    [(); N * M]:,
    T: One + Zero + Geqrf + Orgqr + Field + Copy + AsI32,
{
    pub fn qr(&self) -> Result<Return<T, N, M>, Error> {
        let mut a = self.clone();
        let k = N.min(M);
        let mut tau = vec![T::zero(); k];
        
        // Query optimal workspace
        let mut work = vec![T::zero(); 1];
        let info = T::geqrf(
            N as i32,
            M as i32,
            &mut a.data,
            N as i32,
            &mut tau,
            &mut work,
            -1,
        );
        
        if info != 0 {
            return Err(Error::InvalidArgument(info));
        }
        
        // Perform QR factorization
        let lwork = work[0].as_i32();
        let mut work = vec![T::zero(); lwork as usize];
        let info = T::geqrf(
            N as i32,
            M as i32,
            &mut a.data,
            N as i32,
            &mut tau,
            &mut work,
            lwork,
        );
        
        if info != 0 {
            return Err(Error::LapackError(info));
        }
        
        // Extract R matrix (upper triangular part)
        let mut r = SMatrix::<T, N, M>::zeros();
        for i in 0..N {
            for j in i..M {
                r[(i, j)] = a[(i, j)];
            }
        }
        
        // Generate Q matrix
        let info = T::orgqr(
            N as i32,
            N.min(M) as i32,
            k as i32,
            &mut a.data,
            N as i32,
            &tau,
            &mut work,
            lwork,
        );
        
        if info != 0 {
            return Err(Error::LapackError(info));
        }
        
        let q = a;
        
        Ok(Return { q, r })
    }
}
//}}}