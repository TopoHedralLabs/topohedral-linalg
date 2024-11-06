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


//{{{ impl: IndexValue for BinopExpr
impl<A, B, T> IndexValue<usize> for BinopExpr<A, B, T>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = T;
    fn index_value(&self, index: usize) -> Self::Output {
        //{{{ trace
        debug!("Calling BinopExpr::index_value with index = {}", index);
        debug!("a.index_value(index) = {}", self.a.index_value(index)); 
        debug!("b.index_value(index) = {}", self.b.index_value(index)); 
        //}}}
        self.a.index_value(index) + self.b.index_value(index)
    }
}
//}}}
//{{{ impl: Evaluate for BinopExpr
impl <A, B, T, const N: usize, const M: usize> Evaluate<T, N, M>  for BinopExpr<A, B, T>
where
    [(); N * M]:,
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    fn eval(&self) -> SMatrix<T, N, M> {

        //{{{ trace
        debug!("Calling BinopExpr::eval()");
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
    type Output = BinopExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T>;

    fn add(self, rhs: Self) -> Self::Output {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Add<&' SMatrix> for BinopExpr
impl<'a, A, B, T, const N: usize, const M: usize> Add<&'a SMatrix<T, N, M>> for BinopExpr<A, B, T>
where
    [(); N * M]:,
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = BinopExpr<Self, &'a SMatrix<T, N, M>, T>;

    fn add(self, rhs: &'a SMatrix<T, N, M>) -> Self::Output {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Add<BinopExpr> for &'a SMatrix
impl<'a, A, B, T, const N: usize, const M: usize> Add<BinopExpr<A, B, T>> for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = BinopExpr<Self, BinopExpr<A, B, T>, T>;

    fn add(self, rhs: BinopExpr<A, B, T>) -> Self::Output {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Add for BinopExpr
impl<A, B, C, D, T> Add<BinopExpr<A, B, T>> for BinopExpr<C, D, T>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    C: IndexValue<usize, Output = T>,
    D: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = BinopExpr<BinopExpr<C, D, T>, BinopExpr<A, B,T>, T>;

    fn add(self, rhs: BinopExpr<A, B, T>) -> Self::Output {
        BinopExpr {
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
        let matrix6 = SMatrix::<i32, 2, 2>::from_value(100000);
        let mut matrix7 = SMatrix::<i32, 2, 2>::default();
        matrix7 = ((&matrix4 + &matrix5) + (&matrix1 + &matrix2 + &matrix3) + &matrix6).eval();


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