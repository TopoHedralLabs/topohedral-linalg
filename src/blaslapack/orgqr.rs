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

//{{{ trait: Orqr
pub trait Orgqr: Copy {
    
    fn orgqr(
        m: i32,
        n: i32,
        k: i32,
        a: &mut [Self],
        lda: i32,
        tau: &[Self],
        work: &mut [Self],
        lwork: i32,
    ) -> i32;
}
//}}}
//{{{ impl: Orqr for f64
impl Orgqr for f64 {

    #[inline]
    fn orgqr(
        m: i32,
        n: i32,
        k: i32,
        a: &mut [Self],
        lda: i32,
        tau: &[Self],
        work: &mut [Self],
        lwork: i32,
    ) -> i32 {
        let mut info = 0;
        unsafe {
            lapack::dorgqr(m, n, k, a, lda, tau, work, lwork, &mut info);
        }
        info
    }
}
//}}}
//{{{ impl: Orqr for f32
impl Orgqr for f32 {

    #[inline]
    fn orgqr(
        m: i32,
        n: i32,
        k: i32,
        a: &mut [Self],
        lda: i32,
        tau: &[Self],
        work: &mut [Self],
        lwork: i32,
    ) -> i32 {
        let mut info = 0;
        unsafe {
            lapack::sorgqr(m, n, k, a, lda, tau, work, lwork, &mut info);
        }
        info
    }
}
//}}}