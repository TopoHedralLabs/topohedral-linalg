//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::expr::*;
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------


#[derive(Clone)]
pub struct SMatrix<T, const N: usize, const M: usize> 
    where [(); N*M]:, 
    T: Field + Default + Copy,
{
    data: [T; N*M],
}

impl<T, const N: usize, const M: usize> Default for SMatrix<T, N, M>    
    where [(); N*M]:, 
    T: Field + Default + Copy,
    {

    fn default() -> Self {
        Self {
            data: [T::default(); N*M],
        }
    }   
    }

impl <T, const N: usize, const M: usize> SMatrix<T, N, M>
    where [(); N*M]:,
    T: Field + Default + Copy,
{
    pub fn from_value(value: T) -> Self {
        Self {
            data: [value; N*M],
        }
    }
}


impl<T, const N: usize, const M: usize> Expression for SMatrix<T, N, M> 
    where [(); N*M]:, 
    T: Field + Default + Copy,
{
    type Output = SMatrix<T, N, M>;

    fn eval(&self) -> Self::Output {
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
  
}
//}}}