//! Provides Schur decomposition of a matrix.
//!
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
pub enum SchurError {
    #[error("Error in Schur, exited with code {0}")]
    LapackError(i32),
}

pub struct SSchur<T, const N: usize, const M: usize>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display,
{
    pub q: SMatrix<T, N, M>,
    pub t: SMatrix<T, N, M>,
}

trait Gees: Copy {
    fn gees(
        jobvs: u8,
        sort: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        sdim: &mut i32,
        wr: &mut [Self],
        wi: &mut [Self],
        vs: &mut [Self],
        ldvs: i32,
        work: &mut [Self],
        lwork: i32,
        bwork: &mut [i32],
    ) -> i32;
}

impl Gees for f64 {
    #[inline]
    fn gees(
        jobvs: u8,
        sort: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        sdim: &mut i32,
        wr: &mut [Self],
        wi: &mut [Self],
        vs: &mut [Self],
        ldvs: i32,
        work: &mut [Self],
        lwork: i32,
        bwork: &mut [i32],

    ) -> i32 {
        let mut info = 0;
        unsafe {
            lapack::dgees(
                jobvs,
                sort,
                None,
                n,
                a,
                lda,
                sdim,
                wr,
                wi,
                vs,
                ldvs,
                work,
                lwork,
                bwork,
                &mut info,
            );
        }
        info
    }
}

impl Gees for f32 {
    #[inline]
    fn gees(
        jobvs: u8,
        sort: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        sdim: &mut i32,
        wr: &mut [Self],
        wi: &mut [Self],
        vs: &mut [Self],
        ldvs: i32,
        work: &mut [Self],
        lwork: i32,
        bwork: &mut [i32],
    ) -> i32 {
        let mut info = 0;
        unsafe {
            lapack::sgees(
                jobvs,
                sort,
                None,
                n,
                a,
                lda,
                sdim,
                wr,
                wi,
                vs,
                ldvs,
                work,
                lwork,
                bwork,
                &mut info,
            );
        }
        info
    }
}

#[allow(private_bounds)]
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    [(); N * 5]:,
    [(); N * M]:,
    T: One + Zero + Gees + Field + Default + Copy + fmt::Display + AsI32,
{
    pub fn schur(&self) -> Result<SSchur<T, N, M>, SchurError> {
        let mut a = self.clone();
        let mut vs = SMatrix::<T, N, M>::default();
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
            return Err(SchurError::LapackError(info));
        }

        Ok(SSchur { q: vs, t: a })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{matrix::matrix_op::matmul::MatMul, SMatrixConstructors};
    use approx::assert_relative_eq;

}
