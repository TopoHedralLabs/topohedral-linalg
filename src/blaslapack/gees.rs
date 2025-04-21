//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error in gees, exited with code {0}")]
    LapackError(i32),
}

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
    ) -> Result<(), Error>;
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

    ) -> Result<(), Error> {
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
        if info != 0 {
            return Err(Error::LapackError(info));
        }
        Ok(())
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
    ) ->  Result<(), Error> {
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
        if info != 0 {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}