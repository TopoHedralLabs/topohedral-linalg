use crate::blaslapack::gees::{self, Gees};
use crate::common::{Field, One, Zero};

pub(crate) struct SchurRaw<T>
{
    pub q_data: Vec<T>,
    pub t_data: Vec<T>,
}

/// Shared GEES logic. Consumes the cloned matrix data and returns raw Q/T buffers.
pub(crate) fn schur_raw<T>(
    mut a_data: Vec<T>,
    n: usize,
    m: usize,
) -> Result<SchurRaw<T>, gees::Error>
where
    T: One + Zero + Gees + Field + Default + Copy,
{
    let mut vs = vec![T::zero(); n * m];
    let mut wr = vec![T::zero(); n];
    let mut wi = vec![T::zero(); n];
    let mut sdim = 0;
    let mut work = vec![T::zero(); n * 5];
    let lwork = (n * 5) as i32;
    let mut bwork = vec![0; n];

    T::gees(
        b'V',
        b'N',
        n as i32,
        &mut a_data,
        n as i32,
        &mut sdim,
        &mut wr,
        &mut wi,
        &mut vs,
        n as i32,
        &mut work,
        lwork,
        &mut bwork,
    )?;

    Ok(SchurRaw { q_data: vs, t_data: a_data })
}
