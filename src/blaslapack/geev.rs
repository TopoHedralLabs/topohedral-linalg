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

pub trait Geev: Copy {
    fn geev(
        jobvl: u8,
        jobvr: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        wr: &mut [Self],
        wi: &mut [Self],
        vl: &mut [Self],
        ldvl: i32,
        vr: &mut [Self],
        ldvr: i32,
        work: &mut [Self],
        lwork: i32,
    ) -> i32;
}

impl Geev for f64 {
    #[inline]
    fn geev(
        jobvl: u8,
        jobvr: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        wr: &mut [Self],
        wi: &mut [Self],
        vl: &mut [Self],
        ldvl: i32,
        vr: &mut [Self],
        ldvr: i32,
        work: &mut [Self],
        lwork: i32,
    ) -> i32 {
        let mut info = 0;
        unsafe {
            lapack::dgeev(
                jobvl,
                jobvr,
                n,
                a,
                lda,
                wr,
                wi,
                vl,
                ldvl,
                vr,
                ldvr,
                work,
                lwork,
                &mut info,
            );
        }
        info
    }
}

impl Geev for f32 {
    #[inline]
    fn geev(
        jobvl: u8,
        jobvr: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        wr: &mut [Self],
        wi: &mut [Self],
        vl: &mut [Self],
        ldvl: i32,
        vr: &mut [Self],
        ldvr: i32,
        work: &mut [Self],
        lwork: i32,
    ) -> i32 {
        let mut info = 0;
        unsafe {
            lapack::sgeev(
                jobvl,
                jobvr,
                n,
                a,
                lda,
                wr,
                wi,
                vl,
                ldvl,
                vr,
                ldvr,
                work,
                lwork,
                &mut info,
            );
        }
        info
    }
}