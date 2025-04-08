//! Provides QR decomposition of a matrix.
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

//{{{ enum: QRError
#[derive(Error, Debug)]
pub enum QRError {
    #[error("Error in QR, argument {0} is invalid")]
    InvalidArgument(i32),
    #[error("Error in QR decomposition, exited with code {0}")]
    LapackError(i32),
}
//}}}
//{{{ struct: SQR
pub struct SQR<T, const N: usize, const M: usize>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display,
{
    pub q: SMatrix<T, N, M>,
    pub r: SMatrix<T, N, M>,
}
//}}}
//{{{ trait: Geqrf
trait Geqrf: Copy {
    fn geqrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        tau: &mut [Self],
        work: &mut [Self],
        lwork: i32,
    ) -> i32;
    
    fn orgqr(
        m: i32,
        n: i32,
        k: i32,
        a: &mut [Self],
        lda: i32,
        tau: &[Self],
        work: &mut [Self],
        lwork: i32,
    ) -> i32;
}
//}}}
//{{{ impl: Geqrf for f64
impl Geqrf for f64 {
    #[inline]
    fn geqrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        tau: &mut [Self],
        work: &mut [Self],
        lwork: i32,
    ) -> i32 {
        let mut info = 0;
        unsafe {
            lapack::dgeqrf(m, n, a, lda, tau, work, lwork, &mut info);
        }
        info
    }

    #[inline]
    fn orgqr(
        m: i32,
        n: i32,
        k: i32,
        a: &mut [Self],
        lda: i32,
        tau: &[Self],
        work: &mut [Self],
        lwork: i32,
    ) -> i32 {
        let mut info = 0;
        unsafe {
            lapack::dorgqr(m, n, k, a, lda, tau, work, lwork, &mut info);
        }
        info
    }
}
//}}}
//{{{ impl: Geqrf for f32
impl Geqrf for f32 {
    #[inline]
    fn geqrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        tau: &mut [Self],
        work: &mut [Self],
        lwork: i32,
    ) -> i32 {
        let mut info = 0;
        unsafe {
            lapack::sgeqrf(m, n, a, lda, tau, work, lwork, &mut info);
        }
        info
    }

    #[inline]
    fn orgqr(
        m: i32,
        n: i32,
        k: i32,
        a: &mut [Self],
        lda: i32,
        tau: &[Self],
        work: &mut [Self],
        lwork: i32,
    ) -> i32 {
        let mut info = 0;
        unsafe {
            lapack::sorgqr(m, n, k, a, lda, tau, work, lwork, &mut info);
        }
        info
    }
}
//}}}
//{{{ impl: SMatrix<T, N, M>
#[allow(private_bounds)]
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    [(); N * M]:,
    T: One + Zero + Geqrf + Field + Default + Copy + fmt::Display + AsI32,
{
    pub fn qr(&self) -> Result<SQR<T, N, M>, QRError> {
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
            return Err(QRError::InvalidArgument(info));
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
            return Err(QRError::LapackError(info));
        }
        
        // Extract R matrix (upper triangular part)
        let mut r = SMatrix::<T, N, M>::default();
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
            return Err(QRError::LapackError(info));
        }
        
        let q = a;
        
        Ok(SQR { q, r })
    }
}
//}}}

//{{{ mod: tests
#[cfg(test)]
mod tests {

    use crate::{matrix::matrix_op::matmul::MatMul, SMatrixConstructors};
    use super::*;
    use approx::{assert_relative_eq, assert_abs_diff_eq};

}
//}}}
