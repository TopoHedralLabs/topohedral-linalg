//! Provides eigenvalue decomposition of a matrix.
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
pub enum EigError {
    #[error("Error in eigenvalue computation, argument {0} is invalid")]
    InvalidArgument(i32),
    #[error("Error in eigenvalue computation, exited with code {0}")]
    LapackError(i32),
}

/// Represents the eigenvalue decomposition of a square matrix of size `N`.
/// 
/// The eigenvalue decomposition of a matrix `A` is a factorization of the form `A = PDP^-1`,
/// where `P` is the matrix of right eigenvectors, `D` is the diagonal matrix of eigenvalues,
/// and `P^-1` is the matrix of left eigenvectors.
///
/// This struct contains the left and right eigenvectors, as well as the eigenvalues, of the
/// decomposition.
#[derive(Debug)]
pub struct SEig<T, const N: usize>
where
    [(); N * N]:,
    T: Field + Default + Copy + fmt::Display,
{
    pub left_eigvecs: SMatrix<T, N, N>,   
    pub right_eigvecs: SMatrix<T, N, N>,
    pub eigvals: [Complex<T>; N],
}

trait Geev: Copy {
    fn geev(
        jobvl: u8,
        jobvr: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        wr: &mut [Self],
        wi: &mut [Self],
        vl: &mut [Self],
        ldvl: i32,
        vr: &mut [Self],
        ldvr: i32,
        work: &mut [Self],
        lwork: i32,
    ) -> i32;
}

impl Geev for f64 {
    #[inline]
    fn geev(
        jobvl: u8,
        jobvr: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        wr: &mut [Self],
        wi: &mut [Self],
        vl: &mut [Self],
        ldvl: i32,
        vr: &mut [Self],
        ldvr: i32,
        work: &mut [Self],
        lwork: i32,
    ) -> i32 {
        let mut info = 0;
        unsafe {
            lapack::dgeev(
                jobvl,
                jobvr,
                n,
                a,
                lda,
                wr,
                wi,
                vl,
                ldvl,
                vr,
                ldvr,
                work,
                lwork,
                &mut info,
            );
        }
        info
    }
}

impl Geev for f32 {
    #[inline]
    fn geev(
        jobvl: u8,
        jobvr: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        wr: &mut [Self],
        wi: &mut [Self],
        vl: &mut [Self],
        ldvl: i32,
        vr: &mut [Self],
        ldvr: i32,
        work: &mut [Self],
        lwork: i32,
    ) -> i32 {
        let mut info = 0;
        unsafe {
            lapack::sgeev(
                jobvl,
                jobvr,
                n,
                a,
                lda,
                wr,
                wi,
                vl,
                ldvl,
                vr,
                ldvr,
                work,
                lwork,
                &mut info,
            );
        }
        info
    }
}

#[allow(private_bounds)]
impl<T, const N: usize> SMatrix<T, N, N>
where
    [(); N * N]:,
    T: One + Zero + Geev + Field + Default + Copy + fmt::Display + AsI32,
{
    pub fn eig(&self) -> Result<SEig<T, N>, EigError> {
        let mut a = self.clone();
        let mut vl = SMatrix::<T, N, N>::default();
        let mut vr = SMatrix::<T, N, N>::default();
        let mut wr = [T::zero(); N];
        let mut wi = [T::zero(); N];
        
        // Query optimal workspace
        let mut work = vec![T::zero(); 1];
        let info = T::geev(
            b'V' as u8,
            b'V' as u8,
            N as i32,
            &mut a.data,
            N as i32,
            &mut wr,
            &mut wi,
            &mut vl.data,
            N as i32,
            &mut vr.data,
            N as i32,
            &mut work,
            -1,
        );

        if info != 0 {
            return Err(EigError::InvalidArgument(info));
        }

        // Perform eigenvalue decomposition
        let lwork = work[0].as_i32();
        let mut work = vec![T::zero(); lwork as usize];
        let info = T::geev(
            b'V' as u8,
            b'V' as u8,
            N as i32,
            &mut a.data,
            N as i32,
            &mut wr,
            &mut wi,
            &mut vl.data,
            N as i32,
            &mut vr.data,
            N as i32,
            &mut work,
            lwork,
        );

        if info != 0 {
            return Err(EigError::LapackError(info));
        }

        let eigvals: [Complex<T>; N] = std::array::from_fn(|i| Complex::new(wr[i], wi[i]));

        Ok(SEig { left_eigvecs: vl, right_eigvecs: vr, eigvals: eigvals })    
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    use super::super::super::common::{SMatrixConstructors, DMatrixConstructors};

    #[test]
    fn test_eigenvalue_decomposition() {
        let a = SMatrix::<f64, 3, 3>::from_row_slice(&[
            1.0, 5.0, 0.0,
            2.0, 4.0, -1.0,
            0.0, 2.0, 3.0,
        ]);

        let eig = a.eig().unwrap();
        // Known eigenvalues for this matrix
        let expected_eigenvalues = vec![Complex64::new(-0.8595233886152194, 0.0), 
                                        Complex64::new(5.433664629783286, 0.0), 
                                        Complex64::new(3.42585875883193, 0.0)];

        for i in 0..3 {
            assert_relative_eq!(eig.eigvals[i].re, expected_eigenvalues[i].re, epsilon=1e-10);
            assert_relative_eq!(eig.eigvals[i].im, expected_eigenvalues[i].im, epsilon=1e-10);
        }


        // Known left eigenvectors for this matrix
        let expected_left_eigenvecotors = SMatrix::<f64, 3, 3>::from_row_slice(&[
            -0.7212203345550064,  -0.3850687990747861,  -0.3073880480179293,  
            0.6705630402249634, -0.8536329572455033,  -0.3728399943222721,  
            0.1737424476304467,  0.3507603088768719,  0.8755015285934659, 
        ]);

        for i in 0..3 {
            for j in 0..3 {
                assert_relative_eq!(eig.left_eigvecs[(i, j)], expected_left_eigenvecotors[(i, j)], epsilon=1e-10);
            }
        }
        
    }
}
