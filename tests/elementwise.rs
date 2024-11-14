
#![feature(generic_const_exprs)]
#![feature(impl_trait_in_assoc_type)]

use topohedral_linalg::{Evaluate, SMatrix};

#[test]
pub fn elementwise_matrix() {
    let a = SMatrix::<f64, 10, 10>::from_value(1.0);
    let b = SMatrix::<f64, 10, 10>::from_value(10.0);
    let c = SMatrix::<f64, 10, 10>::from_value(100.0);
    let d = SMatrix::<f64, 10, 10>::from_value(1000.0);
    let e = SMatrix::<f64, 10, 10>::from_value(10000.0);
    let f = SMatrix::<f64, 10, 10>::from_value(100000.0);
    let g: SMatrix<f64, 10, 10> = (&f * (&a + &b) - (&c / &d) + &e).eval();

    println!("{}", g)
}
