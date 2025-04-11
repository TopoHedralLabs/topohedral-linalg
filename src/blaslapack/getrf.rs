//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ trait: Getrf
/// Trait for types that support LU factorization.
trait Getrf: Copy
{
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
//}}}
//{{{ impl: Getrf for f64
impl Getrf for f64
{
    #[inline]
    fn getrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        ipiv: &mut [i32],
    ) -> i32
    {
        let mut info = 0;
        unsafe {
            lapack::dgetrf(m, n, a, lda, ipiv, &mut info);
        }
        info
    }
}
//}}}
//{{{ impl: Getrf for f32
impl Getrf for f32
{
    #[inline]
    fn getrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        ipiv: &mut [i32],
    ) -> i32
    {
        let mut info = 0;
        unsafe {
            lapack::sgetrf(m, n, a, lda, ipiv, &mut info);
        }
        info
    }
}
//}}}