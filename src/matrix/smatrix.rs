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
use rand::distributions::uniform::{SampleBorrow, SampleUniform};
//}}}
//{{{ dep imports 
use topohedral_tracing::*;
use rand::distributions::{Distribution, Uniform};
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
//{{{ impl: Index for SMatrix
impl<T, const N: usize, const M: usize> Index<usize> for SMatrix<T, N, M>
    where [(); N*M]:, 
    T: Field + Default + Copy + fmt::Display, 
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
//}}}
//{{{ impl: IndexMut for SMatrix
impl<T, const N: usize, const M: usize> IndexMut<usize> for SMatrix<T, N, M>
    where [(); N*M]:, 
    T: Field + Default + Copy + fmt::Display, 
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
//}}}
//{{{ impl: IndexValue for SMatrix
impl <T, const N: usize, const M: usize> IndexValue<usize> for SMatrix<T, N, M>
    where [(); N*M]:,
    T: Field + Default + Copy + fmt::Display,
{
    type Output = T;

    #[inline]
    fn index_value(&self, index: usize) -> Self::Output {
        self.data[index]
    }
}
//}}}
//{{{ impl: IntoIterator for SMatrix
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
//}}}
//{{{ impl: IntoIterator for &a' SMatrix
impl<'a, T, const N: usize, const M: usize> IntoIterator for &'a SMatrix<T, N, M>
where
    [(); N*M]:,
    T: Field + Default + Copy + fmt::Display,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
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
    T: Field + Default + Copy + fmt::Display + SampleUniform + Sized, 
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

    pub fn from_uniform_random(low: T, high: T) -> Self {
        //{{{ trace
        info!("Initializing SMatrix<T, N, M> from uniform random distribution");
        //}}}
        let mut out = Self::default();

        let range = Uniform::<T>::new(low, high);
        let mut rng = rand::thread_rng();
        for i in 0..N*M {
            out.data[i] = range.sample(&mut rng);
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
//{{{ trait: Evaluate
pub trait Evaluate<T, const N: usize, const M: usize> 
    where [(); N*M]:,
    T: Field + Default + Copy + fmt::Display,
{
    fn eval(&self) -> SMatrix<T, N, M>;  
}
//}}}
//{{{ impl: Evaluate for SMatrix
impl<T, const N: usize, const M: usize> Evaluate<T, N, M> for SMatrix<T, N, M>    
    where [(); N*M]:,
    T: Field + Default + Copy + fmt::Display,
{

    fn eval(&self) -> SMatrix<T, N, M> {
        self.clone()
    }
}
//}}}
//{{{ impl: IndexValue for &'a SMatrix
impl<'a, T, const N: usize, const M: usize> IndexValue<usize> for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = T;

    #[inline]
    fn index_value(&self, index: usize) -> Self::Output {
        self.data[index]
    }
}
//}}}


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

    #[test]
    fn test_matrix_from_uniform_random() {
        let matrix = SMatrix::<f64, 2, 2>::from_uniform_random(-1100.0, 100.1);
    }
}
//}}}