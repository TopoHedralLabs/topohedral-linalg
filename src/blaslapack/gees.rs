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

pub trait Gees: Copy {
    fn gees(
        jobvs: u8,
        sort: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        sdim: &mut i32,
        wr: &mut [Self],
        wi: &mut [Self],
        vs: &mut [Self],
        ldvs: i32,
        work: &mut [Self],
        lwork: i32,
        bwork: &mut [i32],
    ) -> i32;
}

impl Gees for f64 {
    #[inline]
    fn gees(
        jobvs: u8,
        sort: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        sdim: &mut i32,
        wr: &mut [Self],
        wi: &mut [Self],
        vs: &mut [Self],
        ldvs: i32,
        work: &mut [Self],
        lwork: i32,
        bwork: &mut [i32],

    ) -> i32 {
        let mut info = 0;
        unsafe {
            lapack::dgees(
                jobvs,
                sort,
                None,
                n,
                a,
                lda,
                sdim,
                wr,
                wi,
                vs,
                ldvs,
                work,
                lwork,
                bwork,
                &mut info,
            );
        }
        info
    }
}

impl Gees for f32 {
    #[inline]
    fn gees(
        jobvs: u8,
        sort: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        sdim: &mut i32,
        wr: &mut [Self],
        wi: &mut [Self],
        vs: &mut [Self],
        ldvs: i32,
        work: &mut [Self],
        lwork: i32,
        bwork: &mut [i32],
    ) -> i32 {
        let mut info = 0;
        unsafe {
            lapack::sgees(
                jobvs,
                sort,
                None,
                n,
                a,
                lda,
                sdim,
                wr,
                wi,
                vs,
                ldvs,
                work,
                lwork,
                bwork,
                &mut info,
            );
        }
        info
    }
}