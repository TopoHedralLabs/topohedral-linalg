//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::{Field, One, Zero};
use crate::blaslapack::geqrf;
use crate::blaslapack::geqrf::Geqrf;
use crate::blaslapack::orgqr::Orgqr;
use crate::blaslapack::orgqr;
use crate::blaslapack::common::AsI32;
use super::DMatrix;
//}}}
//{{{ std imports 
use std::ops::{Index, IndexMut};
//}}}
//{{{ dep imports 
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: Error
#[derive(Error, Debug)]
pub enum Error {
    #[error("Error in qr(), exited with error:\n{0}")]
    GetrfError(#[from] geqrf::Error),
    #[error("Error in qr(), exited with error:\n{0}")]
    OrgqrError(#[from] orgqr::Error),
}
//}}}
//{{{ struct: Return
pub struct Return<T>
where
    T: Field + Copy 
{
    pub q: DMatrix<T>,
    pub r: DMatrix<T>,
}
//}}}
//{{{ impl: SMatrix<T, N, M>
impl<T> DMatrix<T>
where
    T: One + Zero + Geqrf + Orgqr + Field + Copy + AsI32,
{
    pub fn qr(&self) -> Result<Return<T>, Error> {

        let n = self.nrows;
        let m = self.ncols;
        let mut a = self.clone();
        let k = self.nrows.min(self.ncols);
        let mut tau = vec![T::zero(); k];
        
        // Query optimal workspace
        let mut work = vec![T::zero(); 1];
        T::geqrf(
            n as i32,
            m as i32,
            &mut a.data,
            n as i32,
            &mut tau,
            &mut work,
            -1,
        )?;
        
        // Perform QR factorization
        let lwork = work[0].as_i32();
        let mut work = vec![T::zero(); lwork as usize];
        T::geqrf(
            n as i32,
            m as i32,
            &mut a.data,
            n as i32,
            &mut tau,
            &mut work,
            lwork,
        )?;
        
        // Extract R matrix (upper triangular part)
        let mut r = DMatrix::<T>::zeros(n, m);
        for i in 0..n {
            for j in i..m {
                r[(i, j)] = a[(i, j)];
            }
        }
        
        // Generate Q matrix
        T::orgqr(
            n as i32,
            n.min(m) as i32,
            k as i32,
            &mut a.data,
            n as i32,
            &tau,
            &mut work,
            lwork,
        )?;
        let q = a;
        Ok(Return { q, r })
    }
}
//}}}