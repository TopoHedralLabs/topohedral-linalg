
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
use std::ops::Sub;
//}}}
//{{{ dep imports 
use topohedral_tracing::*;
//}}}
//--------------------------------------------------------------------------------------------------
pub struct SubExpr<A, B, T, const N: usize, const M: usize>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display,
{
    a: A,
    b: B,
    _marker: std::marker::PhantomData<T>,
}


impl<T, const N: usize, const M: usize, A, B> Evaluate<T, N, M>  for SubExpr<A, B, T, N, M>    
    where [(); N*M]:,
    T: Field + Default + Copy + fmt::Display,
    A: Evaluate<T, N, M>,
    B: Evaluate<T, N, M>,
{
    fn eval(&self) -> SMatrix<T, N, M> {
        //{{{ trace
        info!("\nExpresson for SubExpr<A, B> is being evaluated");
        //}}}
        let left = self.a.eval();   
        let right = self.b.eval();  
        //{{{ trace
        info!("\nleft = {} right = {}", left, right);
        //}}}
        let mut out = SMatrix::<T, N, M>::default();
        for i in 0..N*M
        {
            out.data[i] = left.data[i] - right.data[i];
        }
        out
    }
}



impl<T, const N: usize, const M: usize> Sub for SMatrix<T, N, M>
    where [(); N*M]:,
    T: Field + Default + Copy + fmt::Display,
{
    type Output = SubExpr<SMatrix<T, N, M>, SMatrix<T, N, M>, T, N, M>;

    fn sub(self, rhs: Self) -> Self::Output {
        SubExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<A, B, T, const N: usize, const M: usize> Sub<SMatrix<T, N, M>> for SubExpr<A, B, T, N, M>
    where [(); N*M]:,
    T: Field + Default + Copy + fmt::Display,
    A: Evaluate<T, N, M>,
    B: Evaluate<T, N, M>,
{
    type Output = SubExpr<Self, SMatrix<T, N, M>, T, N, M>;  

    fn sub(self, rhs: SMatrix<T, N, M>) -> Self::Output {
        SubExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<A, B, T, const N: usize, const M: usize> Sub<SubExpr<A, B, T, N, M>> for SMatrix<T, N, M> 
where
    [(); N*M]:, 
    T: Field + Default + Copy + fmt::Display,
    A: Evaluate<T, N, M>,
    B: Evaluate<T, N, M>,
{
    type Output = SubExpr<Self, SubExpr<A, B, T, N, M>, T, N, M>;

    fn sub(self, rhs: SubExpr<A, B, T, N, M>) -> Self::Output {
        SubExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<A, B, C, D, T, const N: usize, const M: usize> Sub<SubExpr<C, D, T, N, M>> for SubExpr<A, B, T, N, M>
where
    [(); N*M]:, 
    T: Field + Default + Copy + fmt::Display,
    A: Evaluate<T, N, M>,
    B: Evaluate<T, N, M>,
    C: Evaluate<T, N, M>,
    D: Evaluate<T, N, M>,
{
    type Output = SubExpr<Self, SubExpr<C, D, T, N, M>, T, N, M>;   

    fn sub(self, rhs: SubExpr<C, D, T, N, M>) -> Self::Output {
        SubExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
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
    fn test_matrix_sub() {
        let matrix1 = SMatrix::<i32, 2, 2>::from_value(1);
        let matrix2 = SMatrix::<i32, 2, 2>::from_value(10);
        let matrix3 = SMatrix::<i32, 2, 2>::from_value(100);
        let matrix4 = SMatrix::<i32, 2, 2>::from_value(1000);
        let matrix5 = SMatrix::<i32, 2, 2>::from_value(10000);
        let matrix6 = SMatrix::<i32, 2, 2>::from_value(100000);
        let matrix7 = ((matrix1 - matrix2) - matrix3 - (matrix4 - matrix5) - matrix6).eval();

        let exp_value: i32 = (1 - 10) - 100 - (1000 - 10000) - 100000;
        for val in matrix7 {
            assert_eq!(val, exp_value);
        }

        //{{{ trace
        // trace!("\n{}", matrix7);
        //}}}
    }
  
}
//}}}