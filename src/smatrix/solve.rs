//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use super::SMatrix;
use crate::common::{Field, One, Zero};
use crate::blaslapack::gesv::Gesv;
use crate::blaslapack::common::AsI32;
//}}}
//{{{ std imports 
use std::fmt;
//}}}
//{{{ dep imports 
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error in Solve, argument {0} is invalid")]
    InvalidArgument(i32),
    #[error("Error in Solve, matrix is singular")]
    SingularMatrix(i32),
}

#[allow(private_bounds)]
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    [(); N * M]:,
    T: One + Zero + Gesv + Field + Default + Copy + fmt::Display + AsI32,
{
    pub fn solve(&self, b: &SMatrix<T, N, M>) -> Result<SMatrix<T, N, M>, Error> {
        let mut a = self.clone();
        let mut x = b.clone();
        let mut ipiv = vec![0; N];

        let info = T::gesv(
            N as i32,
            M as i32,
            &mut a.data,
            N as i32,
            &mut ipiv,
            &mut x.data,
            N as i32,
        );

        match info {
            0 => Ok(x),
            i if i < 0 => Err(Error::InvalidArgument(i)),
            i => Err(Error::SingularMatrix(i)),
        }
    }
}