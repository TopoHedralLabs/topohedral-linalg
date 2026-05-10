use crate::blaslapack::common::AsI32;
use crate::blaslapack::geqrf::{self, Geqrf};
use crate::blaslapack::orgqr::{self, Orgqr};
use crate::common::{Field, One, Zero};

pub(crate) struct QrRaw<T>
{
    pub q_data: Vec<T>,
    pub r_data: Vec<T>,
}

#[derive(Debug)]
pub(crate) enum Error
{
    Geqrf(geqrf::Error),
    Orgqr(orgqr::Error),
}

impl From<geqrf::Error> for Error
{
    fn from(e: geqrf::Error) -> Self
    {
        Error::Geqrf(e)
    }
}

impl From<orgqr::Error> for Error
{
    fn from(e: orgqr::Error) -> Self
    {
        Error::Orgqr(e)
    }
}

/// Shared GEQRF + ORGQR logic. Consumes the cloned matrix data and returns raw Q/R buffers.
pub(crate) fn qr_raw<T>(
    mut a_data: Vec<T>,
    n: usize,
    m: usize,
) -> Result<QrRaw<T>, Error>
where
    T: One + Zero + Geqrf + Orgqr + Field + Copy + AsI32,
{
    let k = n.min(m);
    let mut tau = vec![T::zero(); k];

    let mut work = vec![T::zero(); 1];
    T::geqrf(
        n as i32,
        m as i32,
        &mut a_data,
        n as i32,
        &mut tau,
        &mut work,
        -1,
    )?;

    let lwork = work[0].as_i32();
    let mut work = vec![T::zero(); lwork as usize];
    T::geqrf(
        n as i32,
        m as i32,
        &mut a_data,
        n as i32,
        &mut tau,
        &mut work,
        lwork,
    )?;

    // Extract R from upper triangular part of a_data (column-major: element (i,j) at i + j*n)
    let mut r_data = vec![T::zero(); n * m];
    for i in 0..n
    {
        for j in i..m
        {
            r_data[i + j * n] = a_data[i + j * n];
        }
    }

    // Expand Q in-place in a_data
    T::orgqr(
        n as i32,
        n.min(m) as i32,
        k as i32,
        &mut a_data,
        n as i32,
        &tau,
        &mut work,
        lwork,
    )?;

    Ok(QrRaw { q_data: a_data, r_data })
}
