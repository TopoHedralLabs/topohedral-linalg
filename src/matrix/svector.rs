//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::*;
use super::smatrix::*;
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ type: SCVector
/// A statically-sized column vector type.
type SCVector<T, const N: usize> = SMatrix<T, N, 1>;
//}}}
//{{{ impl: SCVector
impl<T, const N: usize> SCVector<T, N> 
where 
    [(); N * 1]:, 
    T: Field + Default + Copy,
{
    /// Creates a new vector from an array
    pub fn from_slice(data: &[T; N]) -> Self 
    where
        T: Copy + Default
    {
        let mut out = Self::default();
        out.data.copy_from_slice(data);
        out
    }

    /// Returns the number of elements in the vector
    pub fn len(&self) -> usize {
        N
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
    fn test_svector_add()
    {
        let a = SCVector::<f64, 3>::from_slice(&[1.0, 2.0, 3.0]);
        let b = SCVector::<f64, 3>::from_slice(&[1.0, 2.0, 3.0]);
        let c: SCVector::<f64, 3> = (&a + &b).eval();

    }

}
//}}}