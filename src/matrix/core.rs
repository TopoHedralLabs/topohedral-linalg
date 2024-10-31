//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::*;
//}}}
//{{{ std imports 
use std::fmt;
//}}}
//{{{ dep imports 
use topohedral_tracing::*;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ struct: SMatrix
#[derive(Clone)]
pub struct SMatrix<T, const N: usize, const M: usize> 
    where [(); N*M]:, 
    T: Field + Default + Copy + fmt::Display, {
    pub(crate) data: [T; N*M],
}
//}}}
//{{{ impl: Default for SMatrix
impl<T, const N: usize, const M: usize> Default for SMatrix<T, N, M>    
    where [(); N*M]:, 
    T: Field + Default + Copy + fmt::Display, 
    {

    fn default() -> Self {
        Self {
            data: [T::default(); N*M],
        }
    }   
    }
//}}}
//{{{ impl: SMatrix
impl <T, const N: usize, const M: usize> SMatrix<T, N, M>
    where [(); N*M]:,
    T: Field + Default + Copy + fmt::Display, 
{
    pub fn from_value(value: T) -> Self {

        //{{{ trace
        info!("Initializing SMatrix<T, N, M> from value {}", value);
        //}}}
        Self {
            data: [value; N*M],
        }
    }

    pub fn from_slice(slice: &[T]) -> Self {
        assert_eq!(slice.len(), N*M);
        //{{{ trace
        info!("Initializing SMatrix<T, N, M> from slice");
        //}}}
        let mut out = Self::default();

        for j in 0..M {
            for i in 0..N {
                out.data[j * N + i] = slice[i * M + j];
            }
        }
        out
    }   
}
//}}}
//{{{ impl fmt::Display for SMatrix 
impl<T, const N: usize, const M: usize> fmt::Display for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let max_width = self.data
            .iter()
            .map(|x| format!("{}", x).len())
            .max()
            .unwrap_or(0);

        writeln!(f, "")?;
        for j in 0..M {
            write!(f, "|")?;
            for i in 0..N {
                write!(f, " {:>width$}", self.data[i * M + j], width = max_width)?;
            }
            writeln!(f, " |")?;
        }
        Ok(())
    }
}
//}}}

pub trait Expression<T, const N: usize, const M: usize> 
    where [(); N*M]:,
    T: Field + Default + Copy + fmt::Display,
{
    fn eval(&self) -> SMatrix<T, N, M>;  
}



//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{

    use super::*;   

    #[test]
    fn test_matrix_default() {
        let matrix = SMatrix::<i32, 2, 2>::default();

        assert_eq!(matrix.data, [0, 0, 0, 0]);
    }
    #[test]
    fn test_matrix_from_val() {
        let matrix = SMatrix::<i32, 2, 2>::from_value(10);
        assert_eq!(matrix.data, [10, 10, 10, 10]);  
    }

    #[test]
    fn test_matrix_from_slice() {
        let matrix = SMatrix::<i32, 2, 2>::from_slice(&[1, 10, 
                                                        100, 1000]);
        assert_eq!(matrix.data, [1, 100, 10, 1000]);  
    }
}
//}}}