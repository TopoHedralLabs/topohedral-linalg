//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::common::AsI32;
use crate::blaslapack::geqrf;
use crate::blaslapack::geqrf::Geqrf;
use crate::blaslapack::orgqr;
use crate::blaslapack::orgqr::Orgqr;
use crate::common::{Field, One, Zero};
use crate::smatrix::SMatrix;
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: Error
#[derive(Error, Debug)]
pub enum Error
{
    #[error("Error in qr(), exited with error:\n{0}")]
    GetrfError(#[from] geqrf::Error),
    #[error("Error in qr(), exited with error:\n{0}")]
    OrgqrError(#[from] orgqr::Error),
}
//}}}
//{{{ struct: Return
pub struct Return<T, const N: usize, const M: usize>
where
    [(); N * M]:,
    T: Field + Copy,
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
    pub fn qr(&self) -> Result<Return<T, N, M>, Error>
    {
        let mut a = *self;
        let k = N.min(M);
        let mut tau = vec![T::zero(); k];

        // Query optimal workspace
        let mut work = vec![T::zero(); 1];
        T::geqrf(
            N as i32,
            M as i32,
            &mut a.data,
            N as i32,
            &mut tau,
            &mut work,
            -1,
        )?;

        // Perform QR factorization
        let lwork = work[0].as_i32();
        let mut work = vec![T::zero(); lwork as usize];
        T::geqrf(
            N as i32,
            M as i32,
            &mut a.data,
            N as i32,
            &mut tau,
            &mut work,
            lwork,
        )?;

        // Extract R matrix (upper triangular part)
        let mut r = SMatrix::<T, N, M>::zeros();
        for i in 0..N
        {
            for j in i..M
            {
                r[(i, j)] = a[(i, j)];
            }
        }

        // Generate Q matrix
        T::orgqr(
            N as i32,
            N.min(M) as i32,
            k as i32,
            &mut a.data,
            N as i32,
            &tau,
            &mut work,
            lwork,
        )?;
        let q = a;
        Ok(Return { q, r })
    }
}
//}}}
