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
    #[error("Error in geev, exited with code {0}")]
    LapackError(i32),
}

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
    ) -> Result<(), Error>;
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
    ) -> Result<(), Error> {
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

        if info != 0 {
            return Err(Error::LapackError(info));
        }
        Ok(())
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
    ) -> Result<(), Error> {
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
        if info != 0 {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}