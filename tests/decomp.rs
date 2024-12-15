#![feature(generic_const_exprs)]
#![feature(impl_trait_in_assoc_type)]

use topohedral_linalg::*;

#[test]

pub fn elementwise_matrix()
{

    let a = SMatrix::<f64, 10, 10>::from_uniform_random(-10000.0, 10000.0);
    let qr = a.qr();
    let lu = a.lu();
    let schur = a.schur();

}