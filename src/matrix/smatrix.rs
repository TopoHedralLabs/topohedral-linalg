//! Short Description of module
//!
//! Longer description of module

//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::common::*;
//}}}
//{{{ std imports
use rand::distributions::uniform::SampleUniform;
use std::fmt;
use std::ops::{Index, IndexMut};
//}}}
//{{{ dep imports
use rand::distributions::{Distribution, Uniform};
use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::marker::PhantomData;
//}}}
//--------------------------------------------------------------------------------------------------


//{{{ struct: SMatrix
/// A fixed-size $N \times M$ matrix type that stores its elements in a static, contiguous array.
///
/// The `SMatrix` struct represents a 2D matrix with a fixed size, where the dimensions
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
#[derive(Copy, Clone, Debug)]
pub struct SMatrix<T, const N: usize, const M: usize>
where
    [(); N * M]:,
    T: Field + Default + Copy,
{
    /// The data of the matrix, stored in column-major order.
    pub(crate) data: [T; N * M],
    pub(crate) nrows: usize,
    pub(crate) ncols: usize,
}
//}}}
//{{{ collection: Serialization/Deserializaton
//{{{ impl Serialize for SMatrix
impl<T, const N: usize, const M: usize> Serialize for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + Serialize,
    [T; N * M]: Serialize,
{
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMatrix", 3)?;
        state.serialize_field("data", &self.data)?;
        state.serialize_field("nrows", &self.nrows)?;
        state.serialize_field("ncols", &self.ncols)?;
        state.end()
    }
}
//}}}
//{{{ impl Deserialize for SMatrix

impl<'de, T, const N: usize, const M: usize> Deserialize<'de> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + Deserialize<'de>,
    [T; N * M]: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum DeField
        {
            Data,
            Nrows,
            Ncols,
        }

        struct SMatrixVisitor<T, const N: usize, const M: usize>(PhantomData<T>);

        impl<'de, T, const N: usize, const M: usize> Visitor<'de> for SMatrixVisitor<T, N, M>
        where
            [(); N * M]:,
            T: Field + Default + Copy + fmt::Display + Deserialize<'de>,
            [T; N * M]: Deserialize<'de>,
        {
            type Value = SMatrix<T, N, M>;

            fn expecting(
                &self,
                formatter: &mut fmt::Formatter,
            ) -> fmt::Result
            {
                formatter.write_str("struct SMatrix")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> Result<SMatrix<T, N, M>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut data: Option<[T; N * M]> = None;
                let mut nrows: Option<usize> = None;
                let mut ncols: Option<usize> = None;

                while let Some(key) = map.next_key()?
                {
                    match key
                    {
                        DeField::Data =>
                        {
                            if data.is_some()
                            {
                                return Err(de::Error::duplicate_field("data"));
                            }
                            data = Some(map.next_value()?);
                        }
                        DeField::Nrows =>
                        {
                            if nrows.is_some()
                            {
                                return Err(de::Error::duplicate_field("nrows"));
                            }
                            nrows = Some(map.next_value()?);
                        }
                        DeField::Ncols =>
                        {
                            if ncols.is_some()
                            {
                                return Err(de::Error::duplicate_field("ncols"));
                            }
                            ncols = Some(map.next_value()?);
                        }
                    }
                }

                let data = data.ok_or_else(|| de::Error::missing_field("data"))?;
                let nrows = nrows.ok_or_else(|| de::Error::missing_field("nrows"))?;
                let ncols = ncols.ok_or_else(|| de::Error::missing_field("ncols"))?;

                Ok(SMatrix { data, nrows, ncols })
            }
        }

        impl<'de> Deserialize<'de> for DeField
        {
            fn deserialize<D>(deserializer: D) -> Result<DeField, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor
                {
                    type Value = DeField;

                    fn expecting(
                        &self,
                        formatter: &mut fmt::Formatter,
                    ) -> fmt::Result
                    {
                        formatter.write_str("`data` or `nrows` or `ncols`")
                    }

                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> Result<DeField, E>
                    where
                        E: de::Error,
                    {
                        match value
                        {
                            "data" => Ok(DeField::Data),
                            "nrows" => Ok(DeField::Nrows),
                            "ncols" => Ok(DeField::Ncols),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        const FIELDS: &[&str] = &["data", "nrows", "ncols"];
        deserializer.deserialize_struct("SMatrix", FIELDS, SMatrixVisitor(PhantomData))
    }
}
//}}}
//}}}
//{{{ collection: Index Pair Indexing
//{{{ impl: Index<(usize, usize)> for SMatrix
impl<T, const N: usize, const M: usize> Index<(usize, usize)> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy,
{
    type Output = T;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output
    {
        let lin_idx = lin_index(index, N);
        &self.data[lin_idx]
    }
}
//}}}
//{{{ impl: IndexMut<(usize, usize)> for SMatrix
impl<T, const N: usize, const M: usize> IndexMut<(usize, usize)> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy,
{
    fn index_mut(
        &mut self,
        index: (usize, usize),
    ) -> &mut Self::Output
    {
        let lin_idx = lin_index(index, N);
        &mut self.data[lin_idx]
    }
}
//}}}
//}}}
//{{{ collection: Single integer indexing
//{{{ impl: Index<usize> for SMatrix
impl<T, const N: usize, const M: usize> Index<usize> for SMatrix<T, N, M>
where
    [(); N * M]:,
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
//{{{ impl: IndexMut<usize> for SMatrix
impl<T, const N: usize, const M: usize> IndexMut<usize> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy
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
//{{{ impl: IndexValue<usize> for SMatrix
impl<T, const N: usize, const M: usize> IndexValue<usize> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy
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
//{{{ impl: IntoIterator for SMatrix
impl<T, const N: usize, const M: usize> IntoIterator for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy
{
    type Item = T;

    type IntoIter = std::array::IntoIter<T, { N * M }>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.data.into_iter()
    }
}

//}}}
//{{{ impl: IntoIterator for &a' SMatrix
impl<'a, T, const N: usize, const M: usize> IntoIterator for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
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
//{{{ impl: Default for SMatrix
impl<T, const N: usize, const M: usize> Default for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy,
{
    fn default() -> Self
    {
        Self {
            data: [T::default(); N * M],
            nrows: N,
            ncols: M,
        }
    }
}

//}}}
//{{{ impl fmt::Display for SMatrix
impl<T, const N: usize, const M: usize> fmt::Display for SMatrix<T, N, M>
where
    [(); N * M]:,
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

        for i in 0..N {

            write!(f, "|")?;
            for j in 0..M {
                write!(f, " {:>width$}", self[(i, j)], width = max_width)?;
            }
            writeln!(f, " |")?;
        }

        Ok(())
    }
}

//}}}
//}}}
//{{{ collecton: Evaluation to SMatrix
//{{{ trait: EvaluateSMatrix
pub trait EvaluateSMatrix<T, const N: usize, const M: usize>
where
    [(); N * M]:,
    T: Field + Default + Copy,
{
    fn evals(&self) -> SMatrix<T, N, M>;
}

//}}}
//{{{ impl: Evaluate for SMatrix
impl<T, const N: usize, const M: usize> EvaluateSMatrix<T, N, M> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy
{
    fn evals(&self) -> SMatrix<T, N, M>
    {
        self.clone()
    }
}

//}}}
//{{{ impl: IndexValue for &'a SMatrix
impl<T, const N: usize, const M: usize> IndexValue<usize> for &SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy
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
//{{{ impl: SMatrix
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy,
{
    //{{{ collection: constructors
    //{{{ fun: from_value
    /// Creates a new `SMatrix` with all elements initialized to the given `value`.
    ///
    /// This is a convenience constructor that initializes an `SMatrix` with a constant value.
    /// The resulting matrix will have dimensions `N x M`.
    pub fn from_value(value: T) -> Self
    {
        Self {
            data: [value; N * M],
            nrows: N,
            ncols: M,
        }
    }
    //}}}
    //{{{ fun: from_slice_row
    /// Creates a new `SMatrix` from a slice of `T`.
    ///
    /// The length of the slice must be equal to `N * M`, where `N` and `M` are the
    /// dimensions of the matrix. The elements of the slice are supplied in row-major order so
    /// that statmmemnts like:
    /// ```ignore
    /// SMatrix::from_slice(&[1, 2, 3,
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
    pub fn from_slice_row(slice: &[T]) -> Self
    {
        assert_eq!(slice.len(), N * M);

        let mut out = Self::default();

        for j in 0..M
        {
            for i in 0..N
            {
                out.data[j * N + i] = slice[i * M + j];
            }
        }

        out
    }
    //}}}
    //{{{ fun: from_slice_col   
    pub fn from_slice_col(slice: &[T]) -> Self
    {
        assert_eq!(slice.len(), N * M);
        let mut out = Self::default();
        out.data.copy_from_slice(slice);    
        out
    }
    //}}}
    //{{{ fun: from_uniform_random
    /// Creates a new `SMatrix` with elements initialized to random values within the given range.
    ///
    /// The `low` and `high` parameters specify the inclusive range of the random values.
    /// The matrix is initialized using a uniform random distribution.
    pub fn from_uniform_random(
        low: T,
        high: T,
    ) -> Self
    where
        T: SampleUniform,
    {
        let mut out = Self::default();

        let range = Uniform::<T>::new(low, high);

        let mut rng = rand::thread_rng();

        for i in 0..N * M
        {
            out.data[i] = range.sample(&mut rng);
        }

        out
    }
    //}}}
    //{{{ fun: identity
    /// Creates a new `SMatrix` initialized as the identity matrix.
    ///
    /// The identity matrix is a square matrix with 1s on the main diagonal and 0s elsewhere.
    /// The dimensions of the identity matrix are determined by the generic parameters `N` and `M`.
    pub fn identity() -> Self
    where
        T: One,
    {
        let mut out = Self::default();
        let l = N.min(M);
        for i in 0..l
        {
            out[(i, i)] = T::one()
        }
        out
    }
    //}}}
    //}}}
    //{{{ collection: converters
    //{{{ fun: copy_from
    pub fn copy_from(&mut self, other: &Self)
    {
        debug_assert!(self.nrows == other.nrows);
        debug_assert!(self.ncols == other.ncols);
        self.data.copy_from_slice(&other.data);
    }
    //..............................................................................
    //}}}
    //{{{ fun: as_slice
    pub fn as_slice(&self) -> &[T; N * M]
    {
        &self.data
    }
    //..............................................................................
    //}}}
    //{{{ fun: transpose
    /// Transposes the matrix, returning a new matrix with the rows and columns swapped.
    pub fn transpose(&self) -> SMatrix<T, M, N>
    where
        [(); M * N]:,
    {
        let mut out = SMatrix::<T, M, N>::default();

        for i in 0..M
        {
            for j in 0..N
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
//{{{ impl: Zero for SMatrix
impl<T, const N: usize, const M: usize> Zero for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + Zero,
{
    fn zero() -> Self
    {
        let out = Self::from_value(T::zero());
        out
    }
}
//}}}
//{{{ impl: One for SMatrix
impl<T, const N: usize, const M: usize> One for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + One,
{
    fn one() -> Self
    {
        let out = Self::from_value(T::one());
        out
    }
}
//}}}
//{{{ fun: lin_index
#[inline]
fn lin_index(
    idx: (usize, usize),
    N: usize,
) -> usize
{
    idx.0 + idx.1 * N
}
//}}}


//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]

mod tests
{

    use super::*;


    #[test]
    fn test_matrix_default()
    {
        let matrix = SMatrix::<i32, 2, 2>::default();

        assert_eq!(matrix.data, [0, 0, 0, 0]);
    }

    #[test]

    fn test_matrix_from_val()
    {
        let matrix = SMatrix::<i32, 2, 2>::from_value(10);

        assert_eq!(matrix.data, [10, 10, 10, 10]);
    }

    #[test]

    fn test_matrix_from_slice()
    {
        let matrix = SMatrix::<i32, 2, 2>::from_slice_row(&[1, 10, 100, 1000]);

        assert_eq!(matrix.data, [1, 100, 10, 1000]);
    }

    #[test]

    fn test_matrix_from_uniform_random()
    {
        let matrix = SMatrix::<f64, 2, 2>::from_uniform_random(-1100.0, 100.1);
    }

    #[test]
    fn test_matrix_indexing()
    {
        let matrix = SMatrix::<i32, 2, 2>::from_slice_row(&[1, 10, 100, 1000]);
        assert_eq!(matrix[(0, 0)], 1);
        assert_eq!(matrix[(0, 1)], 10);
        assert_eq!(matrix[(1, 0)], 100);
        assert_eq!(matrix[(1, 1)], 1000);
    }

    #[test]
    fn test_serde()
    {
        let matrix = SMatrix::<i32, 2, 2>::from_slice_row(&[1, 10, 100, 1000]);
        let matrix_json = serde_json::to_string_pretty(&matrix).unwrap();
        let matrix2: SMatrix<i32, 2, 2> = serde_json::from_str(&matrix_json).unwrap();

        for i in 0..4
        {
            assert_eq!(matrix.data[i], matrix2.data[i]);
        }
    }

    #[test]
    fn test_matrix_transpose()
    {
        let matrix = SMatrix::<i32, 2, 3>::from_slice_row(&[1, 2, 3, 4, 5, 6]);
        let transposed = matrix.transpose();
        assert_eq!(transposed[(0, 0)], 1);
        assert_eq!(transposed[(0, 1)], 4);
        assert_eq!(transposed[(1, 0)], 2);
        assert_eq!(transposed[(1, 1)], 5);
        assert_eq!(transposed[(2, 0)], 3);
        assert_eq!(transposed[(2, 1)], 6);
    }

    #[test]
    fn test_copy_from() 
    {
        let mut matrix = SMatrix::<i32, 2, 2>::default();
        let matrix2 = SMatrix::<i32, 2, 2>::from_slice_row(&[1, 2, 3, 4]);
        matrix.copy_from(&matrix2);
        assert_eq!(matrix.data, [1, 3, 2, 4]);
    }
}

//}}}
