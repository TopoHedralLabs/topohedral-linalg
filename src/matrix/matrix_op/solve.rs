//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::matrix::smatrix::*;
use crate::common::*;
use super::common::AsI32;
//}}}
//{{{ std imports 
use std::fmt;
//}}}
//{{{ dep imports 
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

#[derive(Error, Debug)]
pub enum SolveError {
    #[error("Error in Solve, argument {0} is invalid")]
    InvalidArgument(i32),
    #[error("Error in Solve, matrix is singular")]
    SingularMatrix(i32),
}

trait Gesv: Copy {
    fn gesv(
        n: i32,
        nrhs: i32,
        a: &mut [Self],
        lda: i32,
        ipiv: &mut [i32],
        b: &mut [Self],
        ldb: i32,
    ) -> i32;
}

impl Gesv for f64 {
    #[inline]
    fn gesv(
        n: i32,
        nrhs: i32,
        a: &mut [Self],
        lda: i32,
        ipiv: &mut [i32],
        b: &mut [Self],
        ldb: i32,
    ) -> i32 {
        let mut info = 0;
        unsafe {
            lapack::dgesv(n, nrhs, a, lda, ipiv, b, ldb, &mut info);
        }
        info
    }
}

impl Gesv for f32 {
    #[inline]
    fn gesv(
        n: i32,
        nrhs: i32,
        a: &mut [Self],
        lda: i32,
        ipiv: &mut [i32],
        b: &mut [Self],
        ldb: i32,
    ) -> i32 {
        let mut info = 0;
        unsafe {
            lapack::sgesv(n, nrhs, a, lda, ipiv, b, ldb, &mut info);
        }
        info
    }
}

#[allow(private_bounds)]
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    [(); N * M]:,
    T: One + Zero + Gesv + Field + Default + Copy + fmt::Display + AsI32,
{
    pub fn solve(&self, b: &SMatrix<T, N, M>) -> Result<SMatrix<T, N, M>, SolveError> {
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
            i if i < 0 => Err(SolveError::InvalidArgument(i)),
            i => Err(SolveError::SingularMatrix(i)),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{matrix::matrix_op::matmul::MatMul, SMatrixConstructors};
    use super::*;
    use approx::assert_relative_eq;

}
