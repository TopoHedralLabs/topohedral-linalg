//! This module implements all of the supported matrix operations.
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

pub mod addop;
pub mod common;
pub mod divop;
pub mod mulop;
pub mod subop;


//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{
  use super::{addop, divop, mulop,subop, common};
  use super::super::smatrix::{SMatrix, Evaluate};


  #[test]
  fn test_add_sub()
  {
    let a = SMatrix::<f64, 2, 2>::from_value(1.0);
    let b = SMatrix::<f64, 2, 2>::from_value(10.0);
    let c = SMatrix::<f64, 2, 2>::from_value(100.0);
    let d = SMatrix::<f64, 2, 2>::from_value(1000.0);
    let e = SMatrix::<f64, 2, 2>::from_value(10000.0);
    let f = SMatrix::<f64, 2, 2>::from_value(100000.0);
    let g: SMatrix<f64, 2,2> = (&f + (&a + &b) - (&c - &d) + &e).eval();
    let exp_val = 100000.0 + (1.0 + 10.0) - (100.0 - 1000.0) + 10000.0;

    for val in &g {
      assert_eq!(*val, exp_val);
    }
  }

  #[test]
  fn test_mul_div()
  {
    let a = SMatrix::<f64, 2, 2>::from_value(1.0);
    let b = SMatrix::<f64, 2, 2>::from_value(10.0);
    let c = SMatrix::<f64, 2, 2>::from_value(100.0);
    let d = SMatrix::<f64, 2, 2>::from_value(1000.0);
    let e = SMatrix::<f64, 2, 2>::from_value(10000.0);
    let f = SMatrix::<f64, 2, 2>::from_value(100000.0);
    let g: SMatrix<f64, 2,2> = (&f * (&a *  &b) / (&c / &d) * &e).eval();

    let exp_val = 100000.0 * (1.0 * 10.0) / (100.0 / 1000.0) * 10000.0;

    for val in &g {
      assert_eq!(*val, exp_val);
    }
  }

  #[test]
  fn test_add_sub_div_mul()
  {
    let a = SMatrix::<f64, 2, 2>::from_value(1.0);
    let b = SMatrix::<f64, 2, 2>::from_value(10.0);
    let c = SMatrix::<f64, 2, 2>::from_value(100.0);
    let d = SMatrix::<f64, 2, 2>::from_value(1000.0);
    let e = SMatrix::<f64, 2, 2>::from_value(10000.0);
    let f = SMatrix::<f64, 2, 2>::from_value(100000.0);
    let g: SMatrix<f64, 2,2> = (&f * (&a +  &b) - (&c / &d) + &e).eval();

    let exp_val = 100000.0 * (1.0 + 10.0) - (100.0 / 1000.0) + 10000.0;
    for val in &g {
      assert_eq!(*val, exp_val);
    } 
  }
}
//}}}