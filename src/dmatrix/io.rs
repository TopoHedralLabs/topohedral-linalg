//! Display formatting for [`DMatrix`].
//!
//! Implements the [`std::fmt::Display`] trait for [`DMatrix<T>`], producing a human-readable,
//! row-by-row representation of the matrix. Each element is formatted using scientific notation
//! to ensure consistent column widths across magnitudes. The implementation iterates in row-major
//! order (transposing the underlying column-major layout) so that the printed output matches the
//! conventional mathematical layout.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::common::MatrixElementDisplay;
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
    T: Copy + MatrixElementDisplay,
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
                write!(f, " ")?;
                self[(i, j)].fmt_matrix_element(f)?;
                write!(f, " ")?;
            }
            writeln!(f, " |")?;
        }
        Ok(())
    }
}

//}}}
