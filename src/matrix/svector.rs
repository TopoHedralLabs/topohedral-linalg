//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::smatrix::*;
use crate::common::*;
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ type: SCVector
/// A statically-sized column vector type.
pub type SCVector<T, const N: usize> = SMatrix<T, N, 1>;
//}}}
//{{{ type: SRVector
/// A statically-sized row vector type. 
pub type SRVector<T, const N: usize> = SMatrix<T, 1, N>;
//}}}
//{{{ impl: SCector
impl<T, const N: usize> SCVector<T, N>
where
    [(); N * 1]:,
    T: Field + Default + Copy,
{
    /// Creates a new vector from an array
    pub fn from_slice(data: &[T; N]) -> Self
    where
        T: Copy + Default,
    {
        let mut out = Self::default();
        out.data.copy_from_slice(data);
        out
    }

    pub fn norm(&self) -> T 
    where 
        T: Zero
    {

        let mut out = T::zero();

        for i in 0..N {
            out += self[i] * self[i]
        }

        // out = T::sq
        out
    }


    pub fn dot(&self, other: &Self) -> T
    where
    T: Field + Zero
    {
        let mut out = T::zero();
        for i in 0..N {
            out += self[i] * other[i]
        }
        out
    }

    /// Returns the number of elements in the vector
    pub fn len(&self) -> usize
    {
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
        let c: SCVector<f64, 3> = (&a + &b).evals();
    }
    #[test]
    fn test_svector_sub()
    {
        let a = SCVector::<f64, 3>::from_slice(&[5.0, 7.0, 9.0]);
        let b = SCVector::<f64, 3>::from_slice(&[1.0, 2.0, 3.0]);
        let c: SCVector<f64, 3> = (&a - &b).evals();

        assert_eq!(c.as_slice(), &[4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_svector_mul()
    {
        let a = SCVector::<f64, 3>::from_slice(&[2.0, 3.0, 4.0]);
        let b = SCVector::<f64, 3>::from_slice(&[3.0, 4.0, 5.0]);
        let c: SCVector<f64, 3> = (&a * &b).evals();

        assert_eq!(c.as_slice(), &[6.0, 12.0, 20.0]);
    }

    #[test]
    fn test_svector_div()
    {
        let a = SCVector::<f64, 3>::from_slice(&[6.0, 15.0, 24.0]);
        let b = SCVector::<f64, 3>::from_slice(&[2.0, 3.0, 4.0]);
        let c: SCVector<f64, 3> = (&a / &b).evals();

        assert_eq!(c.as_slice(), &[3.0, 5.0, 6.0]);
    }
}
//}}}
