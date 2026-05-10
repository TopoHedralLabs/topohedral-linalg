pub(crate) mod eig;
pub(crate) mod lu;
pub(crate) mod matmul;
pub(crate) mod qr;
pub(crate) mod schur;
pub(crate) mod solve;
pub(crate) mod symeig;

use crate::common::{Field, Shape};

/// Abstracts over matrix storage for generic LAPACK dispatch.
#[allow(dead_code)]
pub(crate) trait MatrixBuffer: Shape
{
    type Scalar: Field + Copy;
    fn as_slice(&self) -> &[Self::Scalar];
    fn as_mut_slice(&mut self) -> &mut [Self::Scalar];
}
