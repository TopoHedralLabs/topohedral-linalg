use crate::blaslapack::gesv::{self, Gesv};
use crate::common::Field;

/// Shared GESV logic. Returns the solution data (overwrites the rhs).
pub(crate) fn solve_raw<T>(
    mut a_data: Vec<T>,
    mut b_data: Vec<T>,
    n: usize,
    nrhs: usize,
) -> Result<Vec<T>, gesv::Error>
where
    T: Gesv + Field,
{
    let mut ipiv = vec![0; n];
    T::gesv(
        n as i32,
        nrhs as i32,
        &mut a_data,
        n as i32,
        &mut ipiv,
        &mut b_data,
        nrhs as i32,
    )?;
    Ok(b_data)
}
