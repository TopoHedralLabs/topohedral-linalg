//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::*;
//}}}
//{{{ std imports 
use std::fmt;
use std::ops::{Index, IndexMut};
//}}}
//{{{ dep imports 
use topohedral_tracing::*;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ struct: SMatrix
#[derive(Clone)]
pub struct SMatrix<T, const N: usize, const M: usize> 
    where [(); N*M]:, 
    T: Field + Default + Copy + fmt::Display, 
{
    pub(crate) data: [T; N*M],
}
//}}}
impl<T, const N: usize, const M: usize> Index<usize> for SMatrix<T, N, M>
    where [(); N*M]:, 
    T: Field + Default + Copy + fmt::Display, 
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl<T, const N: usize, const M: usize> IndexMut<usize> for SMatrix<T, N, M>
    where [(); N*M]:, 
    T: Field + Default + Copy + fmt::Display, 
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl <T, const N: usize, const M: usize> IndexValue<usize> for SMatrix<T, N, M>
    where [(); N*M]:,
    T: Field + Default + Copy + fmt::Display,
{
    type Output = T;
    fn index_value(&self, index: usize) -> Self::Output {
        self.data[index]
    }
}

impl<T, const N: usize, const M: usize> IntoIterator for SMatrix<T, N, M>
where
    [(); N*M]:,
    T: Field + Default + Copy + fmt::Display,
{
    type Item = T;
    type IntoIter = std::array::IntoIter<T, {N*M}>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
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

pub trait Evaluate<T, const N: usize, const M: usize> 
    where [(); N*M]:,
    T: Field + Default + Copy + fmt::Display,
{
    fn eval(&self) -> SMatrix<T, N, M>;  
}

impl<T, const N: usize, const M: usize> Evaluate<T, N, M> for SMatrix<T, N, M>    
    where [(); N*M]:,
    T: Field + Default + Copy + fmt::Display,
{

    fn eval(&self) -> SMatrix<T, N, M> {
        self.clone()
    }
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