use crate::blaslapack::common::AsI32;
use crate::blaslapack::geev::{self, Geev};
use crate::common::{Complex, Field, One, Zero};

pub(crate) struct EigRaw<T>
{
    pub vl:      Vec<T>,
    pub vr:      Vec<T>,
    pub eigvals: Vec<Complex<T>>,
}

/// Shared GEEV logic. Consumes the cloned matrix data and returns raw eigenvector/eigenvalue buffers.
pub(crate) fn eig_raw<T>(
    mut a_data: Vec<T>,
    n: usize,
) -> Result<EigRaw<T>, geev::Error>
where
    T: One + Zero + Geev + Field + Default + Copy + AsI32,
{
    let mut vl = vec![T::zero(); n * n];
    let mut vr = vec![T::zero(); n * n];
    let mut wr = vec![T::zero(); n];
    let mut wi = vec![T::zero(); n];

    let mut work = vec![T::zero(); 1];
    T::geev(
        b'V',
        b'V',
        n as i32,
        &mut a_data,
        n as i32,
        &mut wr,
        &mut wi,
        &mut vl,
        n as i32,
        &mut vr,
        n as i32,
        &mut work,
        -1,
    )?;

    let lwork = work[0].as_i32();
    let mut work = vec![T::zero(); lwork as usize];
    T::geev(
        b'V',
        b'V',
        n as i32,
        &mut a_data,
        n as i32,
        &mut wr,
        &mut wi,
        &mut vl,
        n as i32,
        &mut vr,
        n as i32,
        &mut work,
        lwork,
    )?;

    let eigvals: Vec<Complex<T>> = wr
        .iter()
        .zip(wi.iter())
        .take(n)
        .map(|(&r, &i)| Complex::new(r, i))
        .collect();

    Ok(EigRaw { vl, vr, eigvals })
}
