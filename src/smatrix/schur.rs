//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::{One, Zero, Field, Float};
use super::SMatrix;
use crate::blaslapack::gees::Gees;
use crate::blaslapack::common::AsI32;
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error in Schur, exited with code {0}")]
    LapackError(i32),
}

pub struct Return<T, const N: usize, const M: usize>
where
    [(); N * M]:,
    T: Field + Copy
{
    pub q: SMatrix<T, N, M>,
    pub t: SMatrix<T, N, M>,
}

#[allow(private_bounds)]
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    [(); N * 5]:,
    [(); N * M]:,
    T: One + Zero + Gees + Field + Default + Copy + AsI32,
{
    pub fn schur(&self) -> Result<Return<T, N, M>, Error> {
        let mut a = self.clone();
        let mut vs = SMatrix::<T, N, M>::zeros();
        let mut wr = [T::zero(); N];
        let mut wi = [T::zero(); N];
        let mut sdim = 0;
        let mut work = [T::zero(); N * 5];
        let lwork = (N * 5) as i32;
        let mut bwork = [0; N];
        let info = T::gees(
            b'V' as u8,
            b'N' as u8,
            N as i32,
            &mut a.data,
            N as i32,
            &mut sdim,
            &mut wr,
            &mut wi,
            &mut vs.data,
            N as i32,
            &mut work,
            lwork,
            &mut bwork,
        );

        if info != 0 {
            return Err(Error::LapackError(info));
        }

        Ok(Return { q: vs, t: a })
    }
}


//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{
  
}
//}}}