//! This module implements all of the supported element-wise matrix operations.
//!
//! Operations include:
//!
//! - Addition [`addop`]
//! - Subtraction [`subop`]
//! - Multiplication [`mulop`]
//! - Division [`divop`]
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

mod addop;
mod common;
mod divop;
mod mulop;
mod negop;
mod subop;

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]

mod tests
{

    use super::super::dmatrix::{DMatrix, EvaluateDMatrix};
    use super::super::smatrix::{EvaluateSMatrix, SMatrix};

    #[test]

    fn test_add_sub_smatrix()
    {
        let a = SMatrix::<f64, 2, 2>::from_value(1.0);

        let b = SMatrix::<f64, 2, 2>::from_value(10.0);

        let c = SMatrix::<f64, 2, 2>::from_value(100.0);

        let d = SMatrix::<f64, 2, 2>::from_value(1000.0);

        let e = SMatrix::<f64, 2, 2>::from_value(10000.0);

        let f = SMatrix::<f64, 2, 2>::from_value(100000.0);

        let g: SMatrix<f64, 2, 2> = (&f + (&a + &b) - (&c - &d) + &e).evals();

        let exp_val = 100000.0 + (1.0 + 10.0) - (100.0 - 1000.0) + 10000.0;

        for val in &g
        {
            assert_eq!(*val, exp_val);
        }
    }

    #[test]

    fn test_mul_div_smatrix()
    {
        let a = SMatrix::<f64, 2, 2>::from_value(1.0);

        let b = SMatrix::<f64, 2, 2>::from_value(10.0);

        let c = SMatrix::<f64, 2, 2>::from_value(100.0);

        let d = SMatrix::<f64, 2, 2>::from_value(1000.0);

        let e = SMatrix::<f64, 2, 2>::from_value(10000.0);

        let f = SMatrix::<f64, 2, 2>::from_value(100000.0);

        let g: SMatrix<f64, 2, 2> = (&f * (&a * &b) / (&c / &d) * &e).evals();

        let exp_val = 100000.0 * (1.0 * 10.0) / (100.0 / 1000.0) * 10000.0;

        for val in &g
        {
            assert_eq!(*val, exp_val);
        }
    }

    #[test]
    fn test_add_sub_div_mul_smatrix()
    {
        let a = SMatrix::<f64, 2, 2>::from_value(1.0);

        let b = SMatrix::<f64, 2, 2>::from_value(10.0);

        let c = SMatrix::<f64, 2, 2>::from_value(100.0);

        let d = SMatrix::<f64, 2, 2>::from_value(1000.0);

        let e = SMatrix::<f64, 2, 2>::from_value(10000.0);

        let f = SMatrix::<f64, 2, 2>::from_value(100000.0);

        let g: SMatrix<f64, 2, 2> = (&f * (&a + &b) - (&c / &d) + &e).evals();

        let exp_val = 100000.0 * (1.0 + 10.0) - (100.0 / 1000.0) + 10000.0;

        for val in &g
        {
            assert_eq!(*val, exp_val);
        }
    }

    #[test]

    fn test_add_sub_scalar_smatrix()
    {
        let a = SMatrix::<f64, 2, 2>::from_value(1.0);

        let b = SMatrix::<f64, 2, 2>::from_value(10.0);

        let c = SMatrix::<f64, 2, 2>::from_value(100.0);

        let d = SMatrix::<f64, 2, 2>::from_value(1000.0);

        let e = SMatrix::<f64, 2, 2>::from_value(10000.0);

        let f = SMatrix::<f64, 2, 2>::from_value(100000.0);

        let g: SMatrix<f64, 2, 2> =
            (1.0 + &f - 3.4 + (5.0 - &a + 3.2 + &b) - (&c - &d) + &e + 8.9).evals();

        let exp_val =
            1.0 + 100000.0 - 3.4 + (5.0 - 1.0 + 3.2 + 10.0) - (100.0 - 1000.0) + 10000.0 + 8.9;

        for val in &g
        {
            assert_eq!(*val, exp_val);
        }
    }

    #[test]
    fn test_add_sub_dmatrix()
    {
        let a = DMatrix::<f64>::from_value(2, 2, 1.0);
        let b = DMatrix::<f64>::from_value(2, 2, 10.0);
        let c = DMatrix::<f64>::from_value(2, 2, 100.0);
        let d = DMatrix::<f64>::from_value(2, 2, 1000.0);
        let e = DMatrix::<f64>::from_value(2, 2, 10000.0);
        let f = DMatrix::<f64>::from_value(2, 2, 100000.0);

        let g: DMatrix<f64> = (&f + (&a + &b) - (&c - &d) + &e).evald();
        let exp_val = 100000.0 + (1.0 + 10.0) - (100.0 - 1000.0) + 10000.0;

        for val in &g.data
        {
            assert_eq!(*val, exp_val);
        }
    }

    #[test]
    fn test_mul_div_dmatrix()
    {
        let a = DMatrix::<f64>::from_value(2, 2, 1.0);
        let b = DMatrix::<f64>::from_value(2, 2, 10.0);
        let c = DMatrix::<f64>::from_value(2, 2, 100.0);
        let d = DMatrix::<f64>::from_value(2, 2, 1000.0);
        let e = DMatrix::<f64>::from_value(2, 2, 10000.0);
        let f = DMatrix::<f64>::from_value(2, 2, 100000.0);

        let g: DMatrix<f64> = (&f * (&a * &b) / (&c / &d) * &e).evald();
        let exp_val = 100000.0 * (1.0 * 10.0) / (100.0 / 1000.0) * 10000.0;

        for val in &g.data
        {
            assert_eq!(*val, exp_val);
        }
    }

    #[test]
    fn test_add_sub_div_mul_dmatrix()
    {
        let a = DMatrix::<f64>::from_value(2, 2, 1.0);
        let b = DMatrix::<f64>::from_value(2, 2, 10.0);
        let c = DMatrix::<f64>::from_value(2, 2, 100.0);
        let d = DMatrix::<f64>::from_value(2, 2, 1000.0);
        let e = DMatrix::<f64>::from_value(2, 2, 10000.0);
        let f = DMatrix::<f64>::from_value(2, 2, 100000.0);

        let g: DMatrix<f64> = (&f * (&a + &b) - (&c / &d) + &e).evald();
        let exp_val = 100000.0 * (1.0 + 10.0) - (100.0 / 1000.0) + 10000.0;

        for val in &g.data
        {
            assert_eq!(*val, exp_val);
        }
    }

    #[test]
    fn test_add_sub_scalar_dmatrix()
    {
        let a = DMatrix::<f64>::from_value(2, 2, 1.0);
        let b = DMatrix::<f64>::from_value(2, 2, 10.0);
        let c = DMatrix::<f64>::from_value(2, 2, 100.0);
        let d = DMatrix::<f64>::from_value(2, 2, 1000.0);
        let e = DMatrix::<f64>::from_value(2, 2, 10000.0);
        let f = DMatrix::<f64>::from_value(2, 2, 100000.0);

        let g: DMatrix<f64> =
            (1.0 + &f - 3.4 + (5.0 - &a + 3.2 + &b) - (&c - &d) + &e + 8.9).evald();
        let exp_val =
            1.0 + 100000.0 - 3.4 + (5.0 - 1.0 + 3.2 + 10.0) - (100.0 - 1000.0) + 10000.0 + 8.9;

        for val in &g.data
        {
            assert_eq!(*val, exp_val);
        }
    }
}

//}}}
