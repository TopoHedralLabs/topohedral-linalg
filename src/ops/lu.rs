use crate::blaslapack::getrf::{self, Getrf};
use crate::common::{Field, One, Zero};

pub(crate) struct LuRaw<T>
{
    pub l_data:    Vec<T>,
    pub u_data:    Vec<T>,
    pub p_data:    Vec<T>,
    pub num_swaps: usize,
}

/// Shared GETRF logic. Consumes the cloned matrix data and returns raw L/U/P buffers.
pub(crate) fn lu_raw<T>(
    mut a_data: Vec<T>,
    n: usize,
    m: usize,
) -> Result<LuRaw<T>, getrf::Error>
where
    T: One + Zero + Getrf + Field + Copy,
{
    let mut ipiv = vec![0; n.min(m)];
    T::getrf(n as i32, m as i32, &mut a_data, n as i32, &mut ipiv)?;

    // Extract L and U from the factored matrix (column-major: element (i,j) at i + j*n)
    let mut l_data = vec![T::zero(); n * m];
    let mut u_data = vec![T::zero(); n * m];
    for i in 0..n
    {
        for j in 0..m
        {
            let idx = i + j * n;
            if i > j
            {
                l_data[idx] = a_data[idx];
            }
            else if i == j
            {
                l_data[idx] = T::one();
                u_data[idx] = a_data[idx];
            }
            else
            {
                u_data[idx] = a_data[idx];
            }
        }
    }

    // Build identity permutation matrix and apply row swaps from ipiv
    let mut p_data = vec![T::zero(); n * m];
    for i in 0..n.min(m)
    {
        p_data[i + i * n] = T::one();
    }
    let mut num_swaps = 0;
    for (k, &pivot) in ipiv.iter().enumerate()
    {
        let pivot = (pivot - 1) as usize;
        if k != pivot
        {
            for j in 0..m
            {
                p_data.swap(k + j * n, pivot + j * n);
                num_swaps += 1;
            }
        }
    }

    Ok(LuRaw { l_data, u_data, p_data, num_swaps })
}
