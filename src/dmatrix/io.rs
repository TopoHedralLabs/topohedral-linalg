//! Serialization and display formatting for [`DMatrix`].
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
use serde::{de, Deserialize, Deserializer};
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl Deserialize for DMatrix
impl<'de, T> Deserialize<'de> for DMatrix<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Representation<T> {
            data: Vec<T>,
            nrows: usize,
            ncols: usize,
        }

        let representation = Representation::deserialize(deserializer)?;
        let expected = representation
            .nrows
            .checked_mul(representation.ncols)
            .ok_or_else(|| de::Error::custom("matrix dimensions overflow usize"))?;
        if representation.data.len() != expected {
            return Err(de::Error::custom(
                "matrix data length does not match its dimensions",
            ));
        }

        Ok(DMatrix {
            data: representation.data,
            nrows: representation.nrows,
            ncols: representation.ncols,
        })
    }
}
//}}}
//{{{ impl fmt::Display for SMatrix
impl<T> fmt::Display for DMatrix<T>
where
    T: MatrixElementDisplay,
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        for i in 0..self.nrows {
            write!(f, "|")?;
            for j in 0..self.ncols {
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
