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
pub enum Error
{
    #[error("Error in geqrf, exited with code {0}")]
    LapackError(i32),
}

//{{{ trait: Geqrf
pub trait Geqrf: Copy
{
    fn geqrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        tau: &mut [Self],
        work: &mut [Self],
        lwork: i32,
    ) -> Result<(), Error>;
}
//}}}
//{{{ impl: Geqrf for f64
impl Geqrf for f64
{
    #[inline]
    fn geqrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        tau: &mut [Self],
        work: &mut [Self],
        lwork: i32,
    ) -> Result<(), Error>
    {
        let mut info = 0;
        unsafe {
            lapack::dgeqrf(m, n, a, lda, tau, work, lwork, &mut info);
        }
        if info != 0
        {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
//}}}
//{{{ impl: Geqrf for f32
impl Geqrf for f32
{
    #[inline]
    fn geqrf(
        m: i32,
        n: i32,
        a: &mut [Self],
        lda: i32,
        tau: &mut [Self],
        work: &mut [Self],
        lwork: i32,
    ) -> Result<(), Error>
    {
        let mut info = 0;
        unsafe {
            lapack::sgeqrf(m, n, a, lda, tau, work, lwork, &mut info);
        }
        if info != 0
        {
            return Err(Error::LapackError(info));
        }
        Ok(())
    }
}
//}}}
