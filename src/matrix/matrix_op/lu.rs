//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

use rand::distributions::uniform::SampleUniform;

//{{{ crate imports 
use crate::matrix::smatrix::*;
use crate::common::*;
//}}}
//{{{ std imports 
use std::fmt;
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

pub struct SLU<T, const N: usize, const M: usize>
where 
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display,
{
    pub l: SMatrix<T, N, M>,
    pub u: SMatrix<T, N, M>,
    pub p: SMatrix<T, N, M>,
}

pub trait Getrf: Copy {
    /// Performs LU factorization of a general M-by-N matrix A using partial pivoting
    /// with row interchanges.
    ///
    /// The factorization has the form:
    /// A = P * L * U
    ///
    /// where P is a permutation matrix, L is lower triangular with unit diagonal
    /// elements, and U is upper triangular.
    fn getrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        ipiv: &mut [i32],
    ) -> i32;
}

impl Getrf for f64 {
    #[inline]
    fn getrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        ipiv: &mut [i32],
    ) -> i32 {
        let mut info = 0;
        unsafe {
            lapack::dgetrf(
                m,
                n,
                a,
                lda,
                ipiv,
                &mut info
            );
        }
        info
    }
}

impl Getrf for f32 {
    #[inline]
    fn getrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        ipiv: &mut [i32],
    ) -> i32 {
        let mut info = 0;
        unsafe {
            lapack::sgetrf(
                m,
                n,
                a,
                lda,
                ipiv,
                &mut info
            );
        }
        info
    }
}

impl<T, const N: usize, const M: usize> SMatrix<T, N, M> 
where
    [(); N * M]:,
    T: One + Zero + Getrf + Field + Default + Copy + fmt::Display + SampleUniform,
{
    pub fn lu(&self) -> SLU<T, N, M> {
        let mut a = self.clone();
        let mut ipiv = vec![0; N.min(M)];
        
        let info = T::getrf(
            N as i32,
            M as i32,
            &mut a.data,
            N as i32,
            &mut ipiv
        );

        if info != 0 {
            panic!("LU decomposition failed with error code {}", info);
        }

        // Extract L and U matrices from the factorized matrix
        let mut l = SMatrix::<T, N, M>::default();
        let mut u = SMatrix::<T, N, M>::default();
        
        for i in 0..N {
            for j in 0..M {
                if i > j {
                    l.data[i + j * N] = a.data[i + j * N];
                } else if i == j {
                    l.data[i + j * N] = T::one();
                    u.data[i + j * N] = a.data[i + j * N];
                } else {
                    u.data[i + j * N] = a.data[i + j * N];
                }
            }
        }

        // Create permutation matrix from ipiv
        let mut p = SMatrix::<T, N, M>::identity();
        for (k, &pivot) in ipiv.iter().enumerate() {
            let pivot = (pivot - 1) as usize;
            if k != pivot {
                for j in 0..M {
                    p.data.swap(k + j * N, pivot + j * N);
                }
            }
        }

        SLU { l, u, p }
    }
}


//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{
    
    // #[test]
    // fn 
}
//}}}
