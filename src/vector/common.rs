//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::common::{Field, Float, One, Zero};
//}}}
//{{{ std imports
use core::ops::{Index, IndexMut};
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

pub struct Assert<const check: bool>;
pub trait IsTrue {}
impl IsTrue for Assert<true> {}


pub trait SVectorConstructors<T, const N: usize>
where
    T: Field + Zero + One + Copy + Default,
{
    /// Creates a new vector with the given dimensions and initializes all elements to zero.
    fn zeros() -> Self;

    /// Creates a new vector with the given dimensions and initializes all elements to one.
    fn ones() -> Self;

    /// Creates a new vector with the given dimensions and initializes all elements to the specified value.
    fn from_value(value: f64) -> Self;

    /// Creates a new vector from a slice of values.
    fn from_slice(values: &[T]) -> Self;
}

pub trait DVectorConstructors<T>
{
    /// Creates a new vector with the given dimensions and initializes all elements to zero.
    fn zeros(nelem: usize) -> Self;

    /// Creates a new vector with the given dimensions and initializes all elements to one.
    fn ones(nelem: usize) -> Self;

    /// Creates a new vector with the given dimensions and initializes all elements to the specified value.
    fn from_value(nelem: usize, value: f64) -> Self;

    /// Creates a new vector from a slice of values.
    fn from_slice(values: &[T]) -> Self;
}

pub trait VectorOps:
    Index<usize, Output = Self::ScalarType> + IndexMut<usize, Output = Self::ScalarType> + Sized + Clone
{
    type ScalarType: Field + Zero + One + Copy + Default;

    fn len(&self) -> usize;

    /// Computes the norm (magnitude) of the vector.
    ///
    /// # Returns
    ///
    /// The norm of the vector as a value of type `Self::T`.
    ///
    fn norm(&self) -> Self::ScalarType
    {
        let mut out = Self::ScalarType::zero();

        for i in 0..self.len()
        {
            out += self[i] * self[i]
        }
        out
    }

    fn dot(
        &self,
        other: &Self,
    ) -> Self::ScalarType
    {
        let mut out = Self::ScalarType::zero();
        for i in 0..self.len()
        {
            out += self[i] * other[i]
        }
        out
    }

    fn normalize(&self) -> Self
    {
        let norm = self.norm();
        let mut out = self.clone();
        if norm != Self::ScalarType::zero()
        {
            for i in 0..self.len()
            {
                out[i] /= norm;
            }
        }
        out
    }
    fn cross(
        &self,
        other: &Self,
    ) -> Self
    {
        if self.len() != 3
        {
            panic!("Cross product is only defined for 2D and 3D vectors");
        }

        let mut out = other.clone();
        out[0] = self[1] * other[2] - self[2] * other[1];
        out[1] = self[2] * other[0] - self[0] * other[2];
        out[2] = self[0] * other[1] - self[1] * other[0];
        out
    }
}

pub trait FloatVectorOps: VectorOps
where
    Self::ScalarType: Float + Zero + One + Copy + Default,
{
    fn angle(
        &self,
        other: &Self,
    ) -> Self::ScalarType
    {
        if self.norm() < Self::ScalarType::small() || other.norm() < Self::ScalarType::small()
        {
            panic!("Cannot compute angle with zero vector");
        }

        let a = self.normalize();
        let b = other.normalize();
        let dot = (a.dot(&b)).clamp(-Self::ScalarType::one(), Self::ScalarType::one());
        dot.acos()
    }
}
