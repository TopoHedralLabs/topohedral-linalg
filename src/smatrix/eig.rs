//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use super::SMatrix;
use crate::blaslapack::geev::Geev;
use crate::blaslapack::common::AsI32;
use crate::common::{One, Zero, Field, Complex};
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error in eigenvalue computation, argument {0} is invalid")]
    InvalidArgument(i32),
    #[error("Error in eigenvalue computation, exited with code {0}")]
    LapackError(i32),
}

//{{{ struct: EigResult 
/// Represents the eigenvalue decomposition of a square matrix of size `N`.
/// 
/// The eigenvalue decomposition of a matrix `A` is a factorization of the form `A = PDP^-1`,
/// where `P` is the matrix of right eigenvectors, `D` is the diagonal matrix of eigenvalues,
/// and `P^-1` is the matrix of left eigenvectors.
///
/// This struct contains the left and right eigenvectors, as well as the eigenvalues, of the
/// decomposition.
#[derive(Debug)]
pub struct Return<T, const N: usize>
where
    [(); N * N]:,
    T: Field + Default + Copy
{
    pub left_eigvecs: SMatrix<T, N, N>,   
    pub right_eigvecs: SMatrix<T, N, N>,
    pub eigvals: [Complex<T>; N],
}
//}}}

#[allow(private_bounds)]
impl<T, const N: usize> SMatrix<T, N, N>
where
    [(); N * N]:,
    T: One + Zero + Geev + Field + Default + Copy + AsI32,
{
    pub fn eig(&self) -> Result<Return<T, N>, Error> {
        let mut a = self.clone();
        let mut vl = SMatrix::<T, N, N>::zeros();
        let mut vr = SMatrix::<T, N, N>::zeros();
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
            return Err(Error::InvalidArgument(info));
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
            return Err(Error::LapackError(info));
        }

        let eigvals: [Complex<T>; N] = std::array::from_fn(|i| Complex::new(wr[i], wi[i]));

        Ok(Return { left_eigvecs: vl, right_eigvecs: vr, eigvals: eigvals })    
    }
}