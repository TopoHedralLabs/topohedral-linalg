use crate::blaslapack::common::AsI32;
use crate::blaslapack::syev::{self, Syev};
use crate::common::{Field, One, Zero};

pub(crate) struct SymEigRaw<T>
{
    /// Eigenvector matrix data (overwrites the input matrix after SYEV).
    pub eigvecs_data: Vec<T>,
    pub eigvals:      Vec<T>,
}

/// Shared SYEV logic. Consumes the cloned matrix data and returns raw eigenvec/eigenval buffers.
pub(crate) fn symeig_raw<T>(
    mut a_data: Vec<T>,
    n: usize,
) -> Result<SymEigRaw<T>, syev::Error>
where
    T: One + Zero + Syev + Field + Default + Copy + AsI32,
{
    let mut eigvals = vec![T::zero(); n];

    let mut work = vec![T::zero(); 1];
    T::syev(b'V', b'L', n as i32, &mut a_data, n as i32, &mut eigvals, &mut work, -1)?;

    let lwork = work[0].as_i32();
    let mut work = vec![T::zero(); lwork as usize];
    T::syev(b'V', b'L', n as i32, &mut a_data, n as i32, &mut eigvals, &mut work, lwork)?;

    Ok(SymEigRaw { eigvecs_data: a_data, eigvals })
}
