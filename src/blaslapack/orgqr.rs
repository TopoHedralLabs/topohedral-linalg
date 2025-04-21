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
    #[error("Error in orgqr, exited with code {0}")]
    LapackError(i32),
}

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
    ) -> Result<(), Error>;
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
    ) -> Result<(), Error> {
        let mut info = 0;
        unsafe {
            lapack::dorgqr(m, n, k, a, lda, tau, work, lwork, &mut info);
        }
        if info != 0 {
            return Err(Error::LapackError(info));
        }
        Ok(())
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
    ) -> Result<(), Error> {
        let mut info = 0;
        unsafe {
            lapack::sorgqr(m, n, k, a, lda, tau, work, lwork, &mut info);
        }
        if info != 0 {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
//}}}