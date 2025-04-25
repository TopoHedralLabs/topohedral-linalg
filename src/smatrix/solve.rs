//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::blaslapack::common::AsI32;
use crate::blaslapack::gesv;
use crate::blaslapack::gesv::Gesv;
use crate::common::{Field, One, Zero};
//}}}
//{{{ std imports
use std::fmt;
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

#[derive(Error, Debug)]
pub enum Error
{
    #[error("Error in solve(), exited with error:\n{0}")]
    GesvError(#[from] gesv::Error),
}

#[allow(private_bounds)]
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Gesv + Field,
{
    pub fn solve(
        &self,
        b: &SMatrix<T, N, M>,
    ) -> Result<SMatrix<T, N, M>, Error>
    {
        let mut a = self.clone();
        let mut x = b.clone();
        let mut ipiv = vec![0; N];
        T::gesv(
            N as i32,
            M as i32,
            &mut a.data,
            N as i32,
            &mut ipiv,
            &mut x.data,
            N as i32,
        )?;
        Ok(x)
    }
}
