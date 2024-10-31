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
use std::ops::Add;
//}}}
//{{{ dep imports 
use topohedral_tracing::*;
//}}}
//--------------------------------------------------------------------------------------------------
pub struct AddExpr<A, B, T, const N: usize, const M: usize>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display,
{
    a: A,
    b: B,
    _marker: std::marker::PhantomData<T>,
}


impl<T, const N: usize, const M: usize, A, B> Expression<T, N, M>  for AddExpr<A, B, T, N, M>    
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
    type Output = AddExpr<SMatrix<T, N, M>, SMatrix<T, N, M>, T, N, M>;

    fn add(self, rhs: Self) -> Self::Output {
        AddExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<A, B, T, const N: usize, const M: usize> Add<SMatrix<T, N, M>> for AddExpr<A, B, T, N, M>
    where [(); N*M]:,
    T: Field + Default + Copy + fmt::Display,
    A: Expression<T, N, M>,
    B: Expression<T, N, M>,
{
    type Output = AddExpr<Self, SMatrix<T, N, M>, T, N, M>;  

    fn add(self, rhs: SMatrix<T, N, M>) -> Self::Output {
        AddExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<A, B, T, const N: usize, const M: usize> Add<AddExpr<A, B, T, N, M>> for SMatrix<T, N, M> 
where
    [(); N*M]:, 
    T: Field + Default + Copy + fmt::Display,
    A: Expression<T, N, M>,
    B: Expression<T, N, M>,
{
    type Output = AddExpr<Self, AddExpr<A, B, T, N, M>, T, N, M>;

    fn add(self, rhs: AddExpr<A, B, T, N, M>) -> Self::Output {
        AddExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<A, B, C, D, T, const N: usize, const M: usize> Add<AddExpr<C, D, T, N, M>> for AddExpr<A, B, T, N, M>
where
    [(); N*M]:, 
    T: Field + Default + Copy + fmt::Display,
    A: Expression<T, N, M>,
    B: Expression<T, N, M>,
    C: Expression<T, N, M>,
    D: Expression<T, N, M>,
{
    type Output = AddExpr<Self, AddExpr<C, D, T, N, M>, T, N, M>;   

    fn add(self, rhs: AddExpr<C, D, T, N, M>) -> Self::Output {
        AddExpr {
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
    fn test_matrix_add() {
        let matrix1 = SMatrix::<i32, 2, 2>::from_value(1);
        let matrix2 = SMatrix::<i32, 2, 2>::from_value(10);
        let matrix3 = SMatrix::<i32, 2, 2>::from_value(100);
        let matrix4 = SMatrix::<i32, 2, 2>::from_value(1000);
        let matrix5 = SMatrix::<i32, 2, 2>::from_value(10000);
        let matrix6 = SMatrix::<i32, 2, 2>::from_value(100000);
        let matrix7 = ((matrix1 + matrix2) + matrix3 + (matrix4 + matrix5) + matrix6).eval();

        //{{{ trace
        trace!("\n{}", matrix7);
        //}}}
    }
  
}
//}}}