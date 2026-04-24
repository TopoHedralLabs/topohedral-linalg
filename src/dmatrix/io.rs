//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::common::Field;
//}}}
//{{{ std imports
use std::fmt;
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl fmt::Display for SMatrix
impl<T> fmt::Display for DMatrix<T>
where
    T: Field + Default + Copy + fmt::Display + fmt::LowerExp,
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result
    {
        for i in 0..self.nrows
        {
            write!(f, "|")?;
            for j in 0..self.ncols
            {
                write!(f, " {:1.4e} ", self[(i, j)],)?;
            }
            writeln!(f, " |")?;
        }
        Ok(())
    }
}

//}}}
