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

//{{{ trait: Geqrf
trait Geqrf: Copy {
    fn geqrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        tau: &mut [Self],
        work: &mut [Self],
        lwork: i32,
    ) -> i32;
}
//}}}
//{{{ impl: Geqrf for f64
impl Geqrf for f64 {
    #[inline]
    fn geqrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        tau: &mut [Self],
        work: &mut [Self],
        lwork: i32,
    ) -> i32 {
        let mut info = 0;
        unsafe {
            lapack::dgeqrf(m, n, a, lda, tau, work, lwork, &mut info);
        }
        info
    }
}
//}}}
//{{{ impl: Geqrf for f32
impl Geqrf for f32 {
    #[inline]
    fn geqrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        tau: &mut [Self],
        work: &mut [Self],
        lwork: i32,
    ) -> i32 {
        let mut info = 0;
        unsafe {
            lapack::sgeqrf(m, n, a, lda, tau, work, lwork, &mut info);
        }
        info
    }
}
//}}}
