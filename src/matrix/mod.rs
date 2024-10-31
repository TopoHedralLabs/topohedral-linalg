//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------


//{{{ crate imports 
use crate::expr::*;
//}}}
//{{{ std imports 
use std::{ops::Add, process::Output};
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
    data: [T; N*M],
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



impl<T, const N: usize, const M: usize, A, B> Expression<T, N, M>  for AddExpr<A, B>    
    where [(); N*M]:,
    T: Field + Default + Copy + fmt::Display,
    A: Expression<T, N, M>,
    B: Expression<T, N, M>,
{
    fn eval(&self) -> SMatrix<T, N, M> {
        //{{{ trace
        info!("\nExpresson for AddExpr<A, B> is being evaluated");
        //}}}
        let left = self.a.eval();   
        let right = self.b.eval();  
        //{{{ trace
        info!("\nleft = {} right = {}", left, right);
        //}}}
        let mut out = SMatrix::<T, N, M>::default();
        for i in 0..N*M
        {
            out.data[i] = left.data[i] + right.data[i];
        }
        out
    }
}


impl<T, const N: usize, const M: usize> Expression<T, N, M> for SMatrix<T, N, M>    
    where [(); N*M]:,
    T: Field + Default + Copy + fmt::Display,
{

    fn eval(&self) -> SMatrix<T, N, M> {
        self.clone()
    }
}

impl<T, const N: usize, const M: usize> Add for SMatrix<T, N, M>
    where [(); N*M]:,
    T: Field + Default + Copy + fmt::Display,
{
    type Output = AddExpr<SMatrix<T, N, M>, SMatrix<T, N, M>>;// impl Expression<Output = SMatrix<T, N, M>>;

    fn add(self, rhs: Self) -> Self::Output {
        AddExpr {
            a: self,
            b: rhs,
        }
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


    #[test]
    fn test_matrix_add() {
        let matrix1 = SMatrix::<i32, 2, 2>::from_value(10);
        let matrix2 = SMatrix::<i32, 2, 2>::from_value(10);
        let matrix3 = SMatrix::<i32, 2, 2>::from_value(20);
        let matrix4 = ((matrix1 + matrix2) + matrix3).eval();

        //{{{ trace
        trace!("\n{}", matrix4);
        //}}}
    }
  
}
//}}}