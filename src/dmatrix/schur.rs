//! Schur decomposition of a [`DMatrix`] via LAPACK `dgees`/`sgees`.
//!
//! Provides the `schur()` method on [`DMatrix<T>`], computing the Schur decomposition A = Q T Q^H
//! where Q is orthogonal and T is quasi-upper-triangular (block upper-triangular with 1×1 and 2×2
//! diagonal blocks for real inputs). The factorisation is computed by the [`Gees`] LAPACK driver.
//! Results are returned in a `Return<T>` struct containing Q and T; LAPACK errors propagate as a
//! typed `Error`.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::blaslapack::gees;
use crate::blaslapack::gees::Gees;
use crate::common::{Field, One, Zero};
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: Error
#[derive(Error, Debug)]
pub enum Error
{
    #[error("Error in schur(), exited with error:\n{0}")]
    /// LAPACK `gees` failed to compute the Schur decomposition.
    GeesfError(#[from] gees::Error),
}
//}}}

//{{{ struct: Return
/// Represents the Schur decomposition of a square matrix.
///
/// The decomposition satisfies `A = Q T Q^H`, where `Q` is orthogonal and `T` is
/// quasi-upper-triangular (block upper-triangular with 1×1 and 2×2 diagonal blocks for real inputs).
pub struct Return<T>
where
    T: Field + Copy,
{
    /// Orthogonal Schur vector matrix Q.
    pub q: DMatrix<T>,
    /// Quasi-upper-triangular Schur form T.
    pub t: DMatrix<T>,
}
//}}}

//{{{ impl DMatrix<T>
#[allow(private_bounds)]
impl<T> DMatrix<T>
where
    T: One + Zero + Gees + Field + Default + Copy,
{
    /// Computes the Schur decomposition of the matrix.
    ///
    /// Factors `self` into `Q` and `T` such that `A = Q T Q^H`, where `Q` is orthogonal and `T`
    /// is quasi-upper-triangular.
    ///
    /// # Errors
    ///
    /// Returns [`Error::GeesfError`] if the LAPACK `gees` routine fails.
    pub fn schur(&self) -> Result<Return<T>, Error>
    {
        let n = self.nrows;
        let m = self.ncols;
        let mut a = self.clone();
        let mut vs = DMatrix::<T>::zeros(n, m);
        let mut wr = vec![T::zero(); n];
        let mut wi = vec![T::zero(); n];
        let mut sdim = 0;
        let mut work = vec![T::zero(); n * 5];
        let lwork = (n * 5) as i32;
        let mut bwork = vec![0; n];
        T::gees(
            b'V',
            b'N',
            n as i32,
            &mut a.data,
            n as i32,
            &mut sdim,
            &mut wr,
            &mut wi,
            &mut vs.data,
            n as i32,
            &mut work,
            lwork,
            &mut bwork,
        )?;
        Ok(Return { q: vs, t: a })
    }
}
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{}
//}}}
