//! Short Description of module
//!
//! Longer description of module

//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::common::*;
//}}}
//{{{ std imports
use rand::distributions::uniform::SampleUniform;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Index, IndexMut};
//}}}
//{{{ dep imports
use rand::distributions::{Distribution, Uniform};
use std::marker::PhantomData;
//}}}
//--------------------------------------------------------------------------------------------------


//{{{ struct: DMatrix
/// A fixed-size $N \times M$ matrix type that stores its elements in a static, contiguous array.
///
/// The `DMatrix` struct represents a 2D matrix with a fixed size, where the dimensions
/// are specified as generic parameters `N` and `M`. The elements of the matrix are
/// stored in a contiguous array, which allows for efficient access and manipulation.
///
/// The matrix is stored in column-major order, which means a matrix is stored column by column
/// in memory. So, for example, the matrix:
/// ```ignore
/// 1 2 3
/// 4 5 6
/// 7 8 9
/// ```
/// will be stored in memory as:
/// ```ignore
/// 1 4 7 2 5 9 3 6 9
/// ```
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DMatrix<T>
where
    T: Field + Default + Copy,
{
    /// The data of the matrix, stored in column-major order.
    pub(crate) data: Vec<T>,
    pub(crate) nrows: usize,
    pub(crate) ncols: usize,
}
//}}}
//{{{ collection: Index Pair Indexing
//{{{ impl: Index<(usize, usize)> for DMatrix
impl<T> Index<(usize, usize)> for DMatrix<T>
where
    
    T: Field + Default + Copy,
{
    type Output = T;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output
    {
        let lin_idx = lin_index(index, self.nrows);
        &self.data[lin_idx]
    }
}
//}}}
//{{{ impl: IndexMut<(usize, usize)> for DMatrix
impl<T> IndexMut<(usize, usize)> for DMatrix<T>
where
    
    T: Field + Default + Copy,
{
    fn index_mut(
        &mut self,
        index: (usize, usize),
    ) -> &mut Self::Output
    {
        let lin_idx = lin_index(index, self.nrows);
        &mut self.data[lin_idx]
    }
}
//}}}
//}}}
//{{{ collection: Single integer indexing
//{{{ impl: Index<usize> for DMatrix
impl<T> Index<usize> for DMatrix<T>
where
    
    T: Field + Default + Copy
{
    type Output = T;

    fn index(
        &self,
        index: usize,
    ) -> &Self::Output
    {
        &self.data[index]
    }
}

//}}}
//{{{ impl: IndexMut<usize> for DMatrix
impl<T> IndexMut<usize> for DMatrix<T>
where
    
    T: Field + Default + Copy + fmt::Display,
{
    fn index_mut(
        &mut self,
        index: usize,
    ) -> &mut Self::Output
    {
        &mut self.data[index]
    }
}

//}}}
//{{{ impl: IndexValue<usize> for DMatrix
impl<T> IndexValue<usize> for DMatrix<T>
where
    
    T: Field + Default + Copy + fmt::Display,
{
    type Output = T;

    #[inline]

    fn index_value(
        &self,
        index: usize,
    ) -> Self::Output
    {
        self.data[index]
    }
}

//}}}
//}}}
//{{{ collection: into iterator conversion
//{{{ impl: IntoIterator for DMatrix
impl<T> IntoIterator for DMatrix<T>
where
    T: Field + Default + Copy + fmt::Display,
{
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.data.into_iter()
    }
}

//}}}
//{{{ impl: IntoIterator for &a' DMatrix
impl<'a, T > IntoIterator for &'a DMatrix<T>
where
    T: Field + Default + Copy + fmt::Display,
{
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.data.iter()
    }
}
//}}}
//}}}
//{{{ collection: Miscellataneous
//{{{ impl fmt::Display for DMatrix
impl<T> fmt::Display for DMatrix<T>
where
    
    T: Field + Default + Copy + fmt::Display,
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result
    {
        let max_width = self
            .data
            .iter()
            .map(|x| format!("{}", x).len())
            .max()
            .unwrap_or(0);

        for i in 0..self.nrows {

            write!(f, "|")?;
            for j in 0..self.ncols {
                write!(f, " {:>width$}", self[(i, j)], width = max_width)?;
            }
            writeln!(f, " |")?;
        }

        Ok(())
    }
}

//}}}
//}}}
//{{{ collecton: Evaluation to DMatrix
//{{{ trait: Evaluate
pub trait EvaluateDMatrix<T>
where
    
    T: Field + Default + Copy,
{
    fn evald(&self) -> DMatrix<T>;
}

//}}}
//{{{ impl: Evaluate for DMatrix
impl<T> EvaluateDMatrix<T> for DMatrix<T>
where
    
    T: Field + Default + Copy + fmt::Display,
{
    fn evald(&self) -> DMatrix<T>
    {
        self.clone()
    }
}

//}}}
//{{{ impl: IndexValue for &'a DMatrix
impl<T> IndexValue<usize> for &DMatrix<T>
where
    
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = T;

    #[inline]

    fn index_value(
        &self,
        index: usize,
    ) -> Self::Output
    {
        self.data[index]
    }
}

//}}}
//}}}
//{{{ impl: DMatrix
impl<T> DMatrix<T>
where
    
    T: Field + Default + Copy,
{
    //{{{ collection: constructors
    //{{{ fun: from_value
    /// Creates a new `DMatrix` with all elements initialized to the given `value`.
    ///
    /// This is a convenience constructor that initializes an `DMatrix` with a constant value.
    /// The resulting matrix will have dimensions `N x M`.
    pub fn from_value(nrows: usize, ncols: usize, value: T) -> Self
    {
        Self {
            data: vec![value; nrows*ncols],
            nrows: nrows,
            ncols: ncols,
        }
    }
    //}}}
    //{{{ fun: from_slice_row
    /// Creates a new `DMatrix` from a slice of `T`.
    ///
    /// The length of the slice must be equal to `N * M`, where `N` and `M` are the
    /// dimensions of the matrix. The elements of the slice are supplied in row-major order so
    /// that statmmemnts like:
    /// ```ignore
    /// DMatrix::from_slice(&[1, 2, 3,
    ///                       4, 5, 6]);   
    /// ```
    /// will result in a 2x3 matrix with the values:
    ///
    /// $$
    /// \begin{bmatrix}
    /// 1 & 2 & 3 \\\\
    /// 4 & 5 & 6
    /// \end{bmatrix}
    /// $$
    /// but stored in column-major order.
    ///
    /// # Panics
    ///
    /// This function will panic if the length of the slice is not equal to `N * M`.
    pub fn from_slice_row(nrows: usize, ncols: usize, slice: &[T]) -> Self
    {
        assert_eq!(slice.len(), nrows * ncols);

        let mut out = Self {
            data: vec![T::default(); nrows * ncols],
            nrows: nrows, 
            ncols: ncols,
        };

        for j in 0..ncols
        {
            for i in 0..nrows
            {
                out.data[j * nrows + i] = slice[i * ncols + j];
            }
        }

        out
    }
    //}}}
    //{{{ fun: from_slice_col   
    pub fn from_slice_col(nrows: usize, ncols: usize, slice: &[T]) -> Self
    {
        assert_eq!(slice.len(), nrows * ncols);
        let mut out = Self {
            data: vec![T::default(); nrows * ncols],
            nrows: nrows, 
            ncols: ncols,
        };
        out.data.copy_from_slice(slice);    
        out
    }
    //}}}
    //{{{ fun: from_uniform_random
    /// Creates a new `DMatrix` with elements initialized to random values within the given range.
    ///
    /// The `low` and `high` parameters specify the inclusive range of the random values.
    /// The matrix is initialized using a uniform random distribution.
    pub fn from_uniform_random(
        nrows: usize, 
        ncols: usize, 
        low: T,
        high: T,
    ) -> Self
    where
        T: SampleUniform,
    {
        let mut out = Self {
            data: vec![T::default(); nrows * ncols],
            nrows: nrows, 
            ncols: ncols,
        };

        let range = Uniform::<T>::new(low, high);

        let mut rng = rand::thread_rng();

        for i in 0..nrows*ncols
        {
            out.data[i] = range.sample(&mut rng);
        }

        out
    }
    //}}}
    //{{{ fun: identity
    /// Creates a new `DMatrix` initialized as the identity matrix.
    ///
    /// The identity matrix is a square matrix with 1s on the main diagonal and 0s elsewhere.
    /// The dimensions of the identity matrix are determined by the generic parameters `N` and `M`.
    pub fn identity(nrows: usize, ncols: usize) -> Self
    where
        T: One + Zero,
    {
        let mut out = Self {
            data: vec![T::zero(); nrows * ncols],
            nrows: nrows, 
            ncols: ncols,
        };
        let l = nrows.min(ncols);
        for i in 0..l
        {
            out[(i, i)] = T::one()
        }
        out
    }
    //}}}
    //{{{ fun: ones
    /// Creates a new `DMatrix` initialized with all elements set to 1.
    ///
    /// The dimensions of the matrix are determined by the generic parameters `N` and `M`.
    pub fn ones(nrows: usize, ncols: usize) -> Self
    where
        T: One,
    {
        let out = Self::from_value(nrows, ncols, T::one());
        out
    }
    //}}}
    //{{{ fun: zeros
    /// Creates a new `DMatrix` initialized with all elements set to 0.
    ///
    /// The dimensions of the matrix are determined by the generic parameters `N` and `M`.
    pub fn zeros(nrows: usize, ncols: usize) -> Self
    where
        T: Zero,
    {
        let out = Self::from_value(nrows, ncols, T::zero());
        out
    }
    //}}}
    //}}}
    //{{{ collection: converters
    //{{{ fun: as_slice
    pub fn as_slice(&self) -> &[T]
    {
        self.data.as_slice()
    }
    //..............................................................................
    //}}}
    //{{{ fun: transpose
    /// Transposes the matrix, returning a new matrix with the rows and columns swapped.
    pub fn transpose(&self) -> DMatrix<T>
    where 
        T: Zero
    {
        let mut out = DMatrix::<T>::zeros(self.ncols, self.nrows);

        for i in 0..self.ncols
        {
            for j in 0..self.nrows
            {
                out[(i, j)] = self[(j, i)];
            }
        }
        out
    }
    //}}}   
    //}}}
}

//}}}
//{{{ fun: lin_index
#[inline]
fn lin_index(
    idx: (usize, usize),
    n: usize,
) -> usize
{
    idx.0 + idx.1 * n
}
//}}}


//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]

mod tests
{

    use super::*;


    #[test]
    fn test_matrix_zeros()
    {
        let matrix = DMatrix::<i32>::zeros(2,2);

        assert_eq!(matrix.data, [0, 0, 0, 0]);
    }

    #[test]

    fn test_matrix_from_val()
    {
        let matrix = DMatrix::<i32>::from_value(1,4,10);

        assert_eq!(matrix.data, [10, 10, 10, 10]);
    }

    #[test]

    fn test_matrix_from_slice()
    {
        let matrix = DMatrix::<i32>::from_slice_row(2,2,&[1, 10, 100, 1000]);

        assert_eq!(matrix.data, [1, 100, 10, 1000]);
    }

    #[test]

    fn test_matrix_from_uniform_random()
    {
        let matrix = DMatrix::<f64>::from_uniform_random(4,4, -1100.0, 100.1);
    }

    #[test]
    fn test_matrix_indexing()
    {
        let matrix = DMatrix::<i32>::from_slice_row(2,2, &[1, 10, 100, 1000]);
        assert_eq!(matrix[(0, 0)], 1);
        assert_eq!(matrix[(0, 1)], 10);
        assert_eq!(matrix[(1, 0)], 100);
        assert_eq!(matrix[(1, 1)], 1000);
    }

    #[test]
    fn test_serde()
    {
        let matrix = DMatrix::<i32>::from_slice_row(2, 2, &[1, 10, 100, 1000]);
        let matrix_json = serde_json::to_string_pretty(&matrix).unwrap();
        let matrix2: DMatrix<i32> = serde_json::from_str(&matrix_json).unwrap();

        for i in 0..4
        {
            assert_eq!(matrix.data[i], matrix2.data[i]);
        }
    }

    #[test]
    fn test_matrix_transpose()
    {
        let matrix = DMatrix::<i32>::from_slice_row(2, 3, &[1, 2, 3, 4, 5, 6]);
        let transposed = matrix.transpose();
        assert_eq!(transposed[(0, 0)], 1);
        assert_eq!(transposed[(0, 1)], 4);
        assert_eq!(transposed[(1, 0)], 2);
        assert_eq!(transposed[(1, 1)], 5);
        assert_eq!(transposed[(2, 0)], 3);
        assert_eq!(transposed[(2, 1)], 6);
    }
}

//}}}
