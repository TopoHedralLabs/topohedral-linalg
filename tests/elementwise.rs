#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![feature(impl_trait_in_assoc_type)]

//{{{ mod: smatrix_tests
mod smatrix_tests
{
    use approx::assert_relative_eq;
    use topohedral_linalg::smatrix::{EvaluateSMatrix, SMatrix};

    //{{{ collection: mixed tests
    #[test]
    #[allow(clippy::op_ref)]
    pub fn test_all()
    {
        let aval = 1.0;
        let a = SMatrix::<f64, 10, 10>::from_value(aval);

        let bval = 10.0;
        let b = SMatrix::<f64, 10, 10>::from_value(bval);

        let cval = 100.0;
        let c = SMatrix::<f64, 10, 10>::from_value(cval);

        let dval = 1000.0;
        let d = SMatrix::<f64, 10, 10>::from_value(dval);

        let eval = 10000.0;
        let e = SMatrix::<f64, 10, 10>::from_value(eval);

        let fval = 100000.0;
        let f = SMatrix::<f64, 10, 10>::from_value(fval);

        let gval = fval * (aval + bval) - (cval / (dval * 2.0)) + 1.0 * eval;
        let g: SMatrix<f64, 10, 10> = (&f * (&a + &b) - (&c / (&d * 2.0)) + 1.0 * &e).evals();

        for val in g
        {
            assert_relative_eq!(val, gval, max_relative = 1e-10);
        }
    }
    //}}}
    //{{{ collection: addition tests

    #[test]
    fn test_add()
    {
        let matrix1 = SMatrix::<i32, 2, 2>::from_value(1);

        let matrix2 = SMatrix::<i32, 2, 2>::from_value(10);

        let matrix3 = matrix1 + matrix2;

        for val in &matrix3
        {
            assert_eq!(*val, 11);
        }
    }

    #[test]
    fn test_add_scalar()
    {
        let matrix1 = SMatrix::<i32, 2, 2>::from_value(1);

        let scalar = 10;

        let matrix2 = matrix1 + scalar;

        for val in &matrix2
        {
            assert_eq!(*val, 11);
        }

        let matrix3 = scalar + matrix2;

        for val in &matrix3
        {
            assert_eq!(*val, 21);
        }
    }

    #[test]
    fn test_add_assign()
    {
        let mut matrix1 = SMatrix::<i32, 2, 2>::from_value(1);

        let scalar = 10;

        matrix1 += scalar;

        for val in &matrix1
        {
            assert_eq!(*val, 11);
        }

        let matrix2 = SMatrix::<i32, 2, 2>::from_value(10);

        matrix1 += matrix2;

        for val in &matrix1
        {
            assert_eq!(*val, 21);
        }
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn test_add_lazy()
    {
        let matrix1 = SMatrix::<i32, 2, 2>::from_value(1);

        let matrix2 = SMatrix::<i32, 2, 2>::from_value(10);

        let matrix3 = SMatrix::<i32, 2, 2>::from_value(100);

        let matrix4 = SMatrix::<i32, 2, 2>::from_value(1000);

        let matrix5 = SMatrix::<i32, 2, 2>::from_value(10000);

        let matrix6 = SMatrix::<i32, 2, 2>::from_value(100000);

        let matrix7 = SMatrix::<i32, 2, 2>::from_value(1000000);

        let matrix8: SMatrix<i32, 2, 2> =
            (&matrix7 + (&matrix4 + &matrix5) + (&matrix1 + &matrix2 + &matrix3) + &matrix6)
                .evals();

        let exp_value: i32 = 1000000 + (1000 + 10000) + (1 + 10 + 100) + 100000;

        for val in &matrix8
        {
            assert_eq!(*val, exp_value);
        }
    }

    #[test]
    fn test_add_scalar_lazy()
    {
        let matrix1 = SMatrix::<i32, 2, 2>::from_value(10);

        let matrix2 = SMatrix::<i32, 2, 2>::from_value(100);

        let matrix4: SMatrix<i32, 2, 2> =
            (4i32 + (2i32 + &matrix1) + (&matrix2 + 3i32) + 5i32).evals();

        let exp_val = 4 + (2 + 10) + (100 + 3) + 5;

        for val in &matrix4
        {
            assert_eq!(*val, exp_val);
        }
    }
    //}}}
    //{{{ collection: division tests
    #[test]
    fn test_div()
    {
        let matrix1 = SMatrix::<f64, 2, 2>::from_value(10.0);

        let matrix2 = SMatrix::<f64, 2, 2>::from_value(2.0);

        let matrix3 = matrix1 / matrix2;

        for val in &matrix3
        {
            assert_eq!(*val, 5.0);
        }
    }

    #[test]
    fn test_div_scalar()
    {
        let matrix1 = SMatrix::<f64, 2, 2>::from_value(10.0);

        let scalar = 2.0;

        let matrix2 = matrix1 / scalar;

        for val in &matrix2
        {
            assert_eq!(*val, 5.0);
        }

        let matrix3 = 100.0 / matrix2;

        for val in &matrix3
        {
            assert_eq!(*val, 20.0);
        }
    }

    #[test]
    fn test_div_assign()
    {
        let mut matrix1 = SMatrix::<f64, 2, 2>::from_value(10.0);

        let scalar = 2.0;

        matrix1 /= scalar;

        for val in &matrix1
        {
            assert_eq!(*val, 5.0);
        }

        let matrix2 = SMatrix::<f64, 2, 2>::from_value(5.0);

        matrix1 /= matrix2;

        for val in &matrix1
        {
            assert_eq!(*val, 1.0);
        }
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn test_div_lazy()
    {
        let matrix1 = SMatrix::<f64, 2, 2>::from_value(1.0);

        let matrix2 = SMatrix::<f64, 2, 2>::from_value(10.0);

        let matrix3 = SMatrix::<f64, 2, 2>::from_value(100.0);

        let matrix4 = SMatrix::<f64, 2, 2>::from_value(1000.0);

        let matrix5 = SMatrix::<f64, 2, 2>::from_value(10000.0);

        let matrix6 = SMatrix::<f64, 2, 2>::from_value(100000.0);

        let matrix7 = SMatrix::<f64, 2, 2>::from_value(1000000.0);

        let matrix8: SMatrix<f64, 2, 2> =
            (&matrix7 / (&matrix4 / &matrix5) / (&matrix1 / &matrix2 / &matrix3) / &matrix6)
                .evals();

        let exp_value: f64 = 1000000.0 / (1000.0 / 10000.0) / (1.0 / 10.0 / 100.0) / 100000.0;

        for val in &matrix8
        {
            assert_eq!(*val, exp_value);
        }
    }

    #[test]
    fn test_div_scalar_lazy()
    {
        let matrix1 = SMatrix::<f64, 2, 2>::from_value(10.0);

        let matrix2 = SMatrix::<f64, 2, 2>::from_value(100.0);

        let matrix4: SMatrix<f64, 2, 2> = (4.0 / (2.0 / &matrix1) / (&matrix2 / 3.0) / 5.0).evals();

        let exp_val = 4.0 / (2.0 / 10.0) / (100.0 / 3.0) / 5.0;

        for val in &matrix4
        {
            assert_eq!(*val, exp_val);
        }
    }
    //}}}
    //{{{ collection: subtraction tests

    #[test]
    fn test_sub()
    {
        let matrix1 = SMatrix::<f64, 2, 2>::from_value(10.0);

        let matrix2 = SMatrix::<f64, 2, 2>::from_value(3.0);

        let matrix3 = matrix1 - matrix2;

        for val in &matrix3
        {
            assert_eq!(*val, 7.0);
        }
    }

    #[test]
    fn test_sub_scalar()
    {
        let matrix1 = SMatrix::<f64, 2, 2>::from_value(10.0);

        let scalar = 3.0;

        let matrix2 = matrix1 - scalar;

        for val in &matrix2
        {
            assert_eq!(*val, 7.0);
        }

        let matrix3 = 15.0 - matrix2;

        for val in &matrix3
        {
            assert_eq!(*val, 8.0);
        }
    }

    #[test]
    fn test_sub_assign()
    {
        let mut matrix1 = SMatrix::<f64, 2, 2>::from_value(10.0);

        let scalar = 3.0;

        matrix1 -= scalar;

        for val in &matrix1
        {
            assert_eq!(*val, 7.0);
        }

        let matrix2 = SMatrix::<f64, 2, 2>::from_value(2.0);

        matrix1 -= matrix2;

        for val in &matrix1
        {
            assert_eq!(*val, 5.0);
        }
    }
    #[test]
    #[allow(clippy::op_ref)]
    fn test_sub_lazy()
    {
        let matrix1 = SMatrix::<i32, 2, 2>::from_value(1);

        let matrix2 = SMatrix::<i32, 2, 2>::from_value(10);

        let matrix3 = SMatrix::<i32, 2, 2>::from_value(100);

        let matrix4 = SMatrix::<i32, 2, 2>::from_value(1000);

        let matrix5 = SMatrix::<i32, 2, 2>::from_value(10000);

        let matrix6 = SMatrix::<i32, 2, 2>::from_value(100000);

        let matrix7: SMatrix<i32, 2, 2> =
            ((&matrix4 - &matrix5) - (&matrix1 - &matrix2 - &matrix3) - &matrix6).evals();

        let exp_value: i32 = (1000 - 10000) - (1 - 10 - 100) - 100000;

        for val in &matrix7
        {
            assert_eq!(*val, exp_value);
        }
    }

    #[test]
    fn test_sub_scalar_lazy()
    {
        let matrix1 = SMatrix::<i32, 2, 2>::from_value(10);

        let matrix2 = SMatrix::<i32, 2, 2>::from_value(100);

        let matrix4: SMatrix<i32, 2, 2> =
            (4i32 - (2i32 - &matrix1) - (&matrix2 - 3i32) - 5i32).evals();

        let exp_val = 4 - (2 - 10) - (100 - 3) - 5;

        for val in &matrix4
        {
            assert_eq!(*val, exp_val);
        }
    }
    //}}}
    //{{{ collection: multiplication tests
    #[test]
    #[allow(clippy::op_ref)]
    fn test_mul()
    {
        let matrix1 = SMatrix::<f64, 2, 2>::from_value(10.0);

        let matrix2 = SMatrix::<f64, 2, 2>::from_value(2.0);

        let matrix3 = matrix1 * matrix2;

        for val in &matrix3
        {
            assert_eq!(*val, 20.0);
        }
    }

    #[test]
    fn test_mul_scalar()
    {
        let matrix1 = SMatrix::<f64, 2, 2>::from_value(10.0);

        let scalar = 2.0;

        let matrix2 = matrix1 * scalar;

        for val in &matrix2
        {
            assert_eq!(*val, 20.0);
        }

        let matrix3 = 3.0 * matrix2;

        for val in &matrix3
        {
            assert_eq!(*val, 60.0);
        }
    }

    #[test]
    fn test_mul_assign()
    {
        let mut matrix1 = SMatrix::<f64, 2, 2>::from_value(10.0);

        let scalar = 2.0;

        matrix1 *= scalar;

        for val in &matrix1
        {
            assert_eq!(*val, 20.0);
        }

        let matrix2 = SMatrix::<f64, 2, 2>::from_value(3.0);

        matrix1 *= matrix2;

        for val in &matrix1
        {
            assert_eq!(*val, 60.0);
        }
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn test_mul_lazy()
    {
        let matrix1 = SMatrix::<f64, 2, 2>::from_value(1.0);

        let matrix2 = SMatrix::<f64, 2, 2>::from_value(10.0);

        let matrix3 = SMatrix::<f64, 2, 2>::from_value(100.0);

        let matrix4 = SMatrix::<f64, 2, 2>::from_value(1000.0);

        let matrix5 = SMatrix::<f64, 2, 2>::from_value(10000.0);

        let matrix6 = SMatrix::<f64, 2, 2>::from_value(100000.0);

        let matrix7: SMatrix<f64, 2, 2> =
            ((&matrix4 * &matrix5) * (&matrix1 * &matrix2 * &matrix3) * &matrix6).evals();

        let exp_value: f64 = (1000.0 * 10000.0) * (1.0 * 10.0 * 100.0) * 100000.0;

        for val in &matrix7
        {
            assert_eq!(*val, exp_value);
        }
    }

    #[test]
    fn test_mul_scalar_lazy()
    {
        let matrix1 = SMatrix::<i32, 2, 2>::from_value(10);

        let matrix2 = SMatrix::<i32, 2, 2>::from_value(100);

        let matrix4: SMatrix<i32, 2, 2> =
            (4i32 * (2i32 * &matrix1) * (&matrix2 * 3i32) * 5i32).evals();

        let exp_val = 4 * (2 * 10) * (100 * 3) * 5;

        for val in &matrix4
        {
            assert_eq!(*val, exp_val);
        }
    }
    //}}}
    //{{{ collection: negation tests
    #[test]
    fn test_neg()
    {
        let a = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 2, 3, 4]);
        let b = -a;

        for i in 0..4
        {
            assert_eq!(b[i], -a[i]);
        }
    }
    //}}}
}
//}}}
//{{{ mod: dmatrix_tests
mod dmatrix_tests
{
    use topohedral_linalg::dmatrix::{DMatrix, EvaluateDMatrix};

    //{{{ collection: addition tests
    #[test]
    fn test_add()
    {
        let matrix1 = DMatrix::<i32>::from_value(1, 2, 2);
        let matrix2 = DMatrix::<i32>::from_value(10, 2, 2);
        let matrix3 = matrix1.clone() + matrix2.clone();
        for val in &matrix3
        {
            assert_eq!(*val, 11);
        }
    }

    #[test]
    fn test_add_scalar()
    {
        let matrix1 = DMatrix::<i32>::from_value(1, 2, 2);
        let scalar = 10;
        let matrix2 = matrix1.clone() + scalar;
        for val in &matrix2
        {
            assert_eq!(*val, 11);
        }
        let matrix3 = matrix2 + scalar;
        for val in &matrix3
        {
            assert_eq!(*val, 21);
        }
    }

    #[test]
    fn test_add_assign()
    {
        let mut matrix1 = DMatrix::<i32>::from_value(1, 2, 2);
        let scalar = 10;
        matrix1 += scalar;
        for val in &matrix1
        {
            assert_eq!(*val, 11);
        }
        let matrix2 = DMatrix::<i32>::from_value(10, 2, 2);
        matrix1 += matrix2;
        for val in &matrix1
        {
            assert_eq!(*val, 21);
        }
    }

    #[test]
    fn test_add_lazy()
    {
        let matrix1 = DMatrix::<i32>::from_value(1, 2, 2);

        let matrix2 = DMatrix::<i32>::from_value(10, 2, 2);

        let matrix3 = DMatrix::<i32>::from_value(100, 2, 2);

        let matrix4 = DMatrix::<i32>::from_value(1000, 2, 2);

        let matrix5 = DMatrix::<i32>::from_value(10000, 2, 2);

        let matrix6 = DMatrix::<i32>::from_value(100000, 2, 2);

        let matrix7 = DMatrix::<i32>::from_value(1000000, 2, 2);

        let matrix8: DMatrix<i32> =
            (&matrix7 + (&matrix4 + &matrix5) + (&matrix1 + &matrix2 + &matrix3) + &matrix6)
                .evald();

        let exp_value: i32 = 1000000 + (1000 + 10000) + (1 + 10 + 100) + 100000;

        for val in &matrix8
        {
            assert_eq!(*val, exp_value);
        }
    }
    #[test]
    fn test_add_scalar_lazy()
    {
        let matrix1 = DMatrix::<i32>::from_value(10, 2, 2);

        let matrix2 = DMatrix::<i32>::from_value(100, 2, 2);

        let matrix4: DMatrix<i32> = (4 + (2 + &matrix1) + (&matrix2 + 3) + 5).evald();

        let exp_val = 4 + (2 + 10) + (100 + 3) + 5;

        for val in &matrix4
        {
            assert_eq!(*val, exp_val);
        }
    }
    //}}}
    //{{{ collection: division tests
    #[test]
    fn test_div()
    {
        let matrix1 = DMatrix::<f64>::from_value(1.0, 2, 2);
        let scalar = 10.0;
        let matrix2 = matrix1 / scalar;
        for val in &matrix2
        {
            assert_eq!(*val, 1.0 / 10.0);
        }
    }

    #[test]
    fn test_div_scalar()
    {
        let matrix1 = DMatrix::<f64>::from_value(10.0, 2, 2);
        let scalar = 100.0;
        let matrix2 = matrix1.clone() / scalar;
        for val in &matrix2
        {
            assert_eq!(*val, 10.0 / 100.0);
        }
        let matrix3 = matrix2 / scalar;
        for val in &matrix3
        {
            assert_eq!(*val, 10.0 / (100.0 * 100.0));
        }
    }

    #[test]
    fn test_div_assign()
    {
        let mut matrix1 = DMatrix::<f64>::from_value(10.0, 2, 2);
        let scalar = 100.0;
        matrix1 /= scalar;
        for val in &matrix1
        {
            assert_eq!(*val, 10.0 / 100.0);
        }
        let matrix2 = DMatrix::<f64>::from_value(100.0, 2, 2);
        matrix1 /= matrix2;
        for val in &matrix1
        {
            assert_eq!(*val, 10.0 / (100.0 * 100.0));
        }
    }

    #[test]
    fn test_div_lazy()
    {
        let matrix1 = DMatrix::<f64>::from_value(1.0, 2, 2);
        let matrix2 = DMatrix::<f64>::from_value(10.0, 2, 2);
        let matrix3 = DMatrix::<f64>::from_value(100.0, 2, 2);
        let matrix4 = DMatrix::<f64>::from_value(1000.0, 2, 2);
        let matrix5 = DMatrix::<f64>::from_value(10000.0, 2, 2);
        let matrix6 = DMatrix::<f64>::from_value(100000.0, 2, 2);
        let matrix7 = DMatrix::<f64>::from_value(1000000.0, 2, 2);

        let matrix8: DMatrix<f64> =
            (&matrix7 / (&matrix4 / &matrix5) / (&matrix1 / &matrix2 / &matrix3) / &matrix6)
                .evald();

        let exp_value: f64 = 1000000.0 / (1000.0 / 10000.0) / (1.0 / 10.0 / 100.0) / 100000.0;

        for val in &matrix8
        {
            assert_eq!(*val, exp_value);
        }
    }

    #[test]
    fn test_div_scalar_lazy()
    {
        let matrix1 = DMatrix::<f64>::from_value(10.0, 2, 2);
        let matrix2 = DMatrix::<f64>::from_value(100.0, 2, 2);
        let matrix4: DMatrix<f64> = (4.0 / (2.0 / &matrix1) / (&matrix2 / 3.0) / 5.0).evald();

        let exp_val = 4.0 / (2.0 / 10.0) / (100.0 / 3.0) / 5.0;

        for val in &matrix4
        {
            assert_eq!(*val, exp_val);
        }
    }
    //}}}
    //{{{ collection: subtraction tests
    #[test]
    fn test_sub()
    {
        let matrix1 = DMatrix::<f64>::from_value(10.0, 2, 2);
        let matrix2 = DMatrix::<f64>::from_value(3.0, 2, 2);
        let matrix3 = matrix1.clone() - matrix2.clone();
        for val in &matrix3
        {
            assert_eq!(*val, 7.0);
        }
    }

    #[test]
    fn test_sub_scalar()
    {
        let matrix1 = DMatrix::<f64>::from_value(10.0, 2, 2);
        let scalar = 3.0;
        let matrix2 = matrix1.clone() - scalar;
        for val in &matrix2
        {
            assert_eq!(*val, 7.0);
        }
        let matrix3 = matrix2 - scalar;
        for val in &matrix3
        {
            assert_eq!(*val, 4.0);
        }
    }

    #[test]
    fn test_sub_assign()
    {
        let mut matrix1 = DMatrix::<f64>::from_value(10.0, 2, 2);
        let scalar = 3.0;
        matrix1 -= scalar;
        for val in &matrix1
        {
            assert_eq!(*val, 7.0);
        }
        let matrix2 = DMatrix::<f64>::from_value(2.0, 2, 2);
        matrix1 -= matrix2;
        for val in &matrix1
        {
            assert_eq!(*val, 5.0);
        }
    }

    #[test]
    fn test_sub_lazy()
    {
        let matrix1 = DMatrix::<f64>::from_value(1.0, 2, 2);
        let matrix2 = DMatrix::<f64>::from_value(10.0, 2, 2);
        let matrix3 = DMatrix::<f64>::from_value(100.0, 2, 2);
        let matrix4 = DMatrix::<f64>::from_value(1000.0, 2, 2);
        let matrix5 = DMatrix::<f64>::from_value(10000.0, 2, 2);
        let matrix6 = DMatrix::<f64>::from_value(100000.0, 2, 2);
        let matrix7: DMatrix<f64> =
            ((&matrix4 - &matrix5) - (&matrix1 - &matrix2 - &matrix3) - &matrix6).evald();

        let exp_value: f64 = (1000.0 - 10000.0) - (1.0 - 10.0 - 100.0) - 100000.0;

        for val in &matrix7
        {
            assert_eq!(*val, exp_value);
        }
    }

    #[test]
    fn test_sub_scalar_lazy()
    {
        let matrix1 = DMatrix::<f64>::from_value(10.0, 2, 2);
        let matrix2 = DMatrix::<f64>::from_value(100.0, 2, 2);
        let matrix4: DMatrix<f64> = (4.0 - (2.0 - &matrix1) - (&matrix2 - 3.0) - 5.0).evald();

        let exp_val = 4.0 - (2.0 - 10.0) - (100.0 - 3.0) - 5.0;

        for val in &matrix4
        {
            assert_eq!(*val, exp_val);
        }
    }
    //}}}
    //{{{ collection: multiplication tests
    #[test]
    fn test_mul()
    {
        let matrix1 = DMatrix::<f64>::from_value(10.0, 2, 2);
        let matrix2 = DMatrix::<f64>::from_value(3.0, 2, 2);
        let matrix3 = matrix1.clone() * matrix2.clone();
        for val in &matrix3
        {
            assert_eq!(*val, 30.0);
        }
    }

    #[test]
    fn test_mul_scalar()
    {
        let matrix1 = DMatrix::<f64>::from_value(10.0, 2, 2);
        let scalar = 3.0;
        let matrix2 = matrix1.clone() * scalar;
        for val in &matrix2
        {
            assert_eq!(*val, 30.0);
        }
        let matrix3 = matrix2 * scalar;
        for val in &matrix3
        {
            assert_eq!(*val, 90.0);
        }
    }

    #[test]
    fn test_mul_assign()
    {
        let mut matrix1 = DMatrix::<f64>::from_value(10.0, 2, 2);
        let scalar = 3.0;
        matrix1 *= scalar;
        for val in &matrix1
        {
            assert_eq!(*val, 30.0);
        }
        let matrix2 = DMatrix::<f64>::from_value(2.0, 2, 2);
        matrix1 *= matrix2;
        for val in &matrix1
        {
            assert_eq!(*val, 60.0);
        }
    }
    #[test]
    fn test_mul_lazy()
    {
        let matrix1 = DMatrix::<f64>::from_value(1.0, 2, 2);
        let matrix2 = DMatrix::<f64>::from_value(10.0, 2, 2);
        let matrix3 = DMatrix::<f64>::from_value(100.0, 2, 2);
        let matrix4 = DMatrix::<f64>::from_value(1000.0, 2, 2);
        let matrix5 = DMatrix::<f64>::from_value(10000.0, 2, 2);
        let matrix6 = DMatrix::<f64>::from_value(100000.0, 2, 2);
        let matrix7: DMatrix<f64> =
            ((&matrix4 * &matrix5) * (&matrix1 * &matrix2 * &matrix3) * &matrix6).evald();

        let exp_value: f64 = (1000.0 * 10000.0) * (1.0 * 10.0 * 100.0) * 100000.0;

        for val in &matrix7
        {
            assert_eq!(*val, exp_value);
        }
    }

    #[test]
    fn test_mul_scalar_lazy()
    {
        let matrix1 = DMatrix::<f64>::from_value(10.0, 2, 2);
        let matrix2 = DMatrix::<f64>::from_value(100.0, 2, 2);
        let matrix4: DMatrix<f64> = (4.0 * (2.0 * &matrix1) * (&matrix2 * 3.0) * 5.0).evald();

        let exp_val = 4.0 * (2.0 * 10.0) * (100.0 * 3.0) * 5.0;

        for val in &matrix4
        {
            assert_eq!(*val, exp_val);
        }
    }
    //}}}
    //{{{ collection: negation tests
    #[test]
    fn test_neg()
    {
        let a = DMatrix::<i32>::from_col_slice(&[1, 2, 3, 4], 2, 2);
        let b = -a.clone();
        for i in 0..4
        {
            assert_eq!(b[i], -a[i]);
        }
    }
    //}}}
}
//}}}
