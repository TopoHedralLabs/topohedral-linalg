//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::*;
use super::core::*;
//}}}
//{{{ std imports 
use std::fmt;
use std::ops::{Add, Index};
//}}}
//{{{ dep imports 
use topohedral_tracing::*;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ struct: AddExpr
pub struct AddExpr<A, B, T>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    a: A,
    b: B,
    _marker: std::marker::PhantomData<T>,
}
//}}}
//{{{ impl: IndexValue for AddExpr
impl<A, B, T> IndexValue<usize> for AddExpr<A, B, T>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = T;
    fn index_value(&self, index: usize) -> Self::Output {
        //{{{ trace
        debug!("Calling AddExpr::index_value with index = {}", index);
        debug!("a.index_value(index) = {}", self.a.index_value(index)); 
        debug!("b.index_value(index) = {}", self.b.index_value(index)); 
        //}}}
        self.a.index_value(index) + self.b.index_value(index)
    }
}
//}}}
//{{{ impl: Evaluate for AddExpr
impl <A, B, T, const N: usize, const M: usize> Evaluate<T, N, M>  for AddExpr<A, B, T>
where
    [(); N * M]:,
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    fn eval(&self) -> SMatrix<T, N, M> {

        //{{{ trace
        debug!("Calling AddExpr::eval()");
        //}}}
        let mut out = SMatrix::<T, N, M>::default();

        for i in 0..N*M
        {
            out.data[i] = self.index_value(i);
        }
        out
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
    fn index_value(&self, index: usize) -> Self::Output {
        self.data[index]
    }
}
//}}}
//{{{ impl: Add for &'a SMatrix
impl<'a, T, const N: usize, const M: usize> Add for &'a SMatrix<T, N, M>
where 
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = AddExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T>;

    fn add(self, rhs: Self) -> Self::Output {
        AddExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Add<&' SMatrix> for AddExpr
impl<'a, A, B, T, const N: usize, const M: usize> Add<&'a SMatrix<T, N, M>> for AddExpr<A, B, T>
where
    [(); N * M]:,
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = AddExpr<Self, &'a SMatrix<T, N, M>, T>;

    fn add(self, rhs: &'a SMatrix<T, N, M>) -> Self::Output {
        AddExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Add<AddExpr> for &'a SMatrix
impl<'a, A, B, T, const N: usize, const M: usize> Add<AddExpr<A, B, T>> for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = AddExpr<Self, AddExpr<A, B, T>, T>;

    fn add(self, rhs: AddExpr<A, B, T>) -> Self::Output {
        AddExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Add for AddExpr
impl<A, B, T> Add for AddExpr<A, B, T>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = AddExpr<Self, Self, T>;
    fn add(self, rhs: Self) -> Self::Output {
        AddExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
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
    fn test_matrix_add() {
        let matrix1 = SMatrix::<i32, 2, 2>::from_value(1);
        let matrix2 = SMatrix::<i32, 2, 2>::from_value(10);
        let matrix3 = SMatrix::<i32, 2, 2>::from_value(100);
        let matrix4 = SMatrix::<i32, 2, 2>::from_value(1000);
        let matrix5 = SMatrix::<i32, 2, 2>::from_value(10000);
        // let matrix6 = SMatrix::<i32, 2, 2>::from_value(100000);
        let mut matrix7 = SMatrix::<i32, 2, 2>::default();
        matrix7 = ((&matrix4 + &matrix5) + (&matrix1 + &matrix2  + &matrix3)).eval();


        //{{{ trace
        trace!("\n{}", matrix1);
        trace!("\n{}", matrix2);
        trace!("\n{}", matrix3);
        trace!("\n{}", matrix4);
        trace!("\n{}", matrix7);
        // trace!("\n{}", matrix7);
        //}}}
    }
  
}
//}}}