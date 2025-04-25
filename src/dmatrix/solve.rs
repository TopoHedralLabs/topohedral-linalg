//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::blaslapack::gesv;
use crate::blaslapack::gesv::Gesv;
use crate::common::Field;
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
    #[error("Error in solve(), exited with error:\n{0}")]
    GesvError(#[from] gesv::Error),
}

#[allow(private_bounds)]
impl<T> DMatrix<T>
where
    T: Gesv + Field,
{
    pub fn solve(
        &self,
        b: &DMatrix<T>,
    ) -> Result<DMatrix<T>, Error>
    {
        let n = self.nrows;
        let m = self.ncols;
        let mut a = self.clone();
        let mut x = b.clone();
        let mut ipiv = vec![0; n];
        T::gesv(
            n as i32,
            m as i32,
            &mut a.data,
            n as i32,
            &mut ipiv,
            &mut x.data,
            m as i32,
        )?;
        Ok(x)
    }
}
