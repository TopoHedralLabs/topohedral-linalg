//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::blaslapack::common::AsI32;
use crate::blaslapack::geev::{self, Geev};
use crate::common::{Complex, Field, One, Zero};
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

#[derive(Error, Debug)]
pub enum Error
{
    #[error("Error in eig(), exited with error:\n{0}")]
    GeevError(#[from] geev::Error),
}

//{{{ struct: Return
/// Represents the eigenvalue decomposition of a square matrix of size `N`.
///
/// The eigenvalue decomposition of a matrix `A` is a factorization of the form `A = PDP^-1`,
/// where `P` is the matrix of right eigenvectors, `D` is the diagonal matrix of eigenvalues,
/// and `P^-1` is the matrix of left eigenvectors.
///
/// This struct contains the left and right eigenvectors, as well as the eigenvalues, of the
/// decomposition.
#[derive(Debug)]
pub struct Return<T>
where
    T: Field + Default + Copy,
{
    pub left_eigvecs: DMatrix<T>,
    pub right_eigvecs: DMatrix<T>,
    pub eigvals: Vec<Complex<T>>,
}
//}}}

#[allow(private_bounds)]
impl<T> DMatrix<T>
where
    T: One + Zero + Geev + Field + Default + Copy + AsI32,
{
    pub fn eig(&self) -> Result<Return<T>, Error>
    {
        let n = self.nrows;
        let mut a = self.clone();
        let mut vl = DMatrix::<T>::zeros(n, n);
        let mut vr = DMatrix::<T>::zeros(n, n);
        let nelem = self.nrows * self.ncols;
        let mut wr = vec![T::zero(); nelem];
        let mut wi = vec![T::zero(); nelem];

        // Query optimal workspace
        let mut work = vec![T::zero(); 1];
        T::geev(
            b'V',
            b'V',
            n as i32,
            &mut a.data,
            n as i32,
            &mut wr,
            &mut wi,
            &mut vl.data,
            n as i32,
            &mut vr.data,
            n as i32,
            &mut work,
            -1,
        )?;

        // Perform eigenvalue decomposition
        let lwork = work[0].as_i32();
        let mut work = vec![T::zero(); lwork as usize];
        T::geev(
            b'V',
            b'V',
            n as i32,
            &mut a.data,
            n as i32,
            &mut wr,
            &mut wi,
            &mut vl.data,
            n as i32,
            &mut vr.data,
            n as i32,
            &mut work,
            lwork,
        )?;

        let eigvals: Vec<Complex<T>> = wr
            .iter()
            .zip(wi.iter())
            .map(|(&wr_val, &wi_val)| Complex::new(wr_val, wi_val))
            .take(n)
            .collect();

        Ok(Return {
            left_eigvecs: vl,
            right_eigvecs: vr,
            eigvals,
        })
    }
}
