//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::dmatrix::*;
use crate::common::*;
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ type: DCVector
/// A dynamically-sized column vector type.
pub type DCVector<T> = DMatrix<T>;
//}}}
//{{{ type: DRVector
/// A dynamically-sized row vector type. 
pub type DRVector<T> = DMatrix<T>;
//}}}

//{{{ impl: DCVector
impl<T> DCVector<T>
where
    T: Field + Default + Copy,
{
    /// Creates a new vector from a slice
    pub fn from_slice(data: &[T]) -> Self 
    where 
        T: Zero
    {
        let mut out = Self::default();
        out.data.resize(data.len(), T::zero());
        out.ncols = 1;
        out.nrows = data.len(); 
        out.data.copy_from_slice(data);
        out
    }

    pub fn norm(&self) -> T 
    where 
        T: Zero
    {
        let mut out = T::zero();
        for i in 0..self.nrows {
            out += self[i] * self[i]
        }
        out
    }

    pub fn dot(&self, other: &Self) -> T
    where
        T: Field + Zero
    {
        assert_eq!(self.nrows, other.nrows, "Vector dimensions must match");
        let mut out = T::zero();
        for i in 0..self.nrows {
            out += self[i] * other[i]
        }
        out
    }

    /// Returns the number of elements in the vector
    pub fn len(&self) -> usize {
        self.nrows
    }
}
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dvector_add() {
        let a = DCVector::<f64>::from_slice(&[1.0, 2.0, 3.0]);
        let b = DCVector::<f64>::from_slice(&[1.0, 2.0, 3.0]);
        let c: DCVector<f64> = (&a + &b).evald();
        assert_eq!(c.as_slice(), &[2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_dvector_sub() {
        let a = DCVector::<f64>::from_slice(&[5.0, 7.0, 9.0]);
        let b = DCVector::<f64>::from_slice(&[1.0, 2.0, 3.0]);
        let c: DCVector<f64> = (&a - &b).evald();
        assert_eq!(c.as_slice(), &[4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_dvector_mul() {
        let a = DCVector::<f64>::from_slice(&[2.0, 3.0, 4.0]);
        let b = DCVector::<f64>::from_slice(&[3.0, 4.0, 5.0]);
        let c: DCVector<f64> = (&a * &b).evald();
        assert_eq!(c.as_slice(), &[6.0, 12.0, 20.0]);
    }

    #[test]
    fn test_dvector_div() {
        let a = DCVector::<f64>::from_slice(&[6.0, 15.0, 24.0]);
        let b = DCVector::<f64>::from_slice(&[2.0, 3.0, 4.0]);
        let c: DCVector<f64> = (&a / &b).evald();
        assert_eq!(c.as_slice(), &[3.0, 5.0, 6.0]);
    }
}
//}}}
