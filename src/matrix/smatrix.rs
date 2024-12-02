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
use serde::ser::{Serialize, SerializeStruct, Serializer};
// use serde::de::{Deserialize, Deserializer}
use topohedral_tracing::*;
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
/// will be stored as:
/// ```ignore
/// 1 4 7 2 5 9 3 6 9
/// ```
/// In memory
#[derive(Clone)]
pub struct SMatrix<T, const N: usize, const M: usize>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display,
{
    /// The data of the matrix, stored in column-major order.
    pub(crate) data: [T; N * M],
    pub(crate) nrows: usize,
    pub(crate) ncols: usize,
}
//}}}
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


//{{{ collection: Index Pair Indexing
//{{{ impl: Index<(usize, usize)> for SMatrix
impl<T, const N: usize, const M: usize> Index<(usize, usize)> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + SampleUniform + One + Zero,
{
    type Output = T;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output
    {
        let lin_idx = Self::lin_index(index);
        &self.data[lin_idx]
    }
}
//}}}
//{{{ impl: IndexMut<(usize, usize)> for SMatrix
//}}}
impl<T, const N: usize, const M: usize> IndexMut<(usize, usize)> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + SampleUniform + One + Zero,
{
    fn index_mut(
        &mut self,
        index: (usize, usize),
    ) -> &mut Self::Output
    {
        let lin_idx = Self::lin_index(index);
        &mut self.data[lin_idx]
    }
}
//}}}
//{{{ collection: Single integer indexing
//{{{ impl: Index<usize> for SMatrix
impl<T, const N: usize, const M: usize> Index<usize> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display,
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
//{{{ impl: IndexValue<usize> for SMatrix
impl<T, const N: usize, const M: usize> IndexValue<usize> for SMatrix<T, N, M>
where
    [(); N * M]:,
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
//{{{ impl: IntoIterator for SMatrix
impl<T, const N: usize, const M: usize> IntoIterator for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display,
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
//}}}
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
//{{{ impl: Default for SMatrix
impl<T, const N: usize, const M: usize> Default for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display,
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

        for j in 0..M
        {
            write!(f, "|")?;

            for i in 0..N
            {
                write!(f, " {:>width$}", self.data[i * M + j], width = max_width)?;
            }

            writeln!(f, " |")?;
        }

        Ok(())
    }
}

//}}}
//{{{ collecton: Evaluation to SMatrix
//{{{ trait: Evaluate
pub trait Evaluate<T, const N: usize, const M: usize>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display,
{
    fn eval(&self) -> SMatrix<T, N, M>;
}

//}}}
//{{{ impl: Evaluate for SMatrix
impl<T, const N: usize, const M: usize> Evaluate<T, N, M> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display,
{
    fn eval(&self) -> SMatrix<T, N, M>
    {
        self.clone()
    }
}

//}}}
//{{{ impl: IndexValue for &'a SMatrix
impl<T, const N: usize, const M: usize> IndexValue<usize> for &SMatrix<T, N, M>
where
    [(); N * M]:,
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
//{{{ impl: SMatrix
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + SampleUniform + Sized + One + Zero,
{
    //{{{ fun: from_value
    pub fn from_value(value: T) -> Self
    {
        //{{{ trace
        info!("Initializing SMatrix<T, N, M> from value {}", value);

        //}}}
        Self {
            data: [value; N * M],
            nrows: N,
            ncols: M,
        }
    }
    //}}}
    //{{{ fun: from_slice
    pub fn from_slice(slice: &[T]) -> Self
    {
        assert_eq!(slice.len(), N * M);

        //{{{ trace
        info!("Initializing SMatrix<T, N, M> from slice");

        //}}}
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
    //{{{ fun: from_uniform_random
    pub fn from_uniform_random(
        low: T,
        high: T,
    ) -> Self
    {
        //{{{ trace
        info!("Initializing SMatrix<T, N, M> from uniform random distribution");

        //}}}
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
    pub fn identity() -> Self
    {
        //{{{ trace
        info!("Initialising identity matrix");
        //}}}

        let mut out = Self::default();
        let n = N;
        let m = M;
        let l = n.min(N);

        for i in 0..l
        {
            out[(i, i)] = T::one()
        }
        out
    }
    //}}}

    #[inline]
    fn lin_index(idx: (usize, usize)) -> usize
    {
        idx.0 + idx.1 * N
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
        let matrix = SMatrix::<i32, 2, 2>::from_slice(&[1, 10, 100, 1000]);

        assert_eq!(matrix.data, [1, 100, 10, 1000]);
    }

    #[test]

    fn test_matrix_from_uniform_random()
    {
        let matrix = SMatrix::<f64, 2, 2>::from_uniform_random(-1100.0, 100.1);
    }

    #[test]
    fn test_serialization()
    {
        // setup
        // let orig_dir = std::env::current_dir().unwrap();
        // let tmp_dir = orig_dir.join("tmp");
        // std::fs::create_dir_all(&tmp_dir).unwrap();
        // std::env::set_current_dir(&tmp_dir).unwrap();


        let matrix = SMatrix::<i32, 2, 2>::from_slice(&[1, 10, 100, 1000]);
        let matrix_json = serde_json::to_string_pretty(&matrix).unwrap();

        println!("{}", matrix_json);


        // std::fs::write(path, contents)

        // teardown
        // std::env::set_current_dir(orig_dir).unwrap();
        // std::fs::remove_dir_all(&tmp_dir).unwrap();  
    }
}

//}}}
