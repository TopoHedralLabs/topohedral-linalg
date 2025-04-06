#![feature(generic_const_exprs)]
#![feature(impl_trait_in_assoc_type)]

//{{{ mod: smatrix_tests
mod smatrix_tests
{
    use topohedral_linalg::{SMatrix, EvaluateSMatrix};
    use approx::assert_relative_eq;

    //{{{ collection: mixed tests
    #[test]
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

        let matrix3 = SMatrix::<i32, 2, 2>::from_value(100);

        let matrix4 = SMatrix::<i32, 2, 2>::from_value(1000);

        let matrix5 = SMatrix::<i32, 2, 2>::from_value(10000);

        let matrix6 = SMatrix::<i32, 2, 2>::from_value(100000);

        let matrix7 = SMatrix::<i32, 2, 2>::from_value(1000000);

        let mut matrix8 = SMatrix::<i32, 2, 2>::default();

        matrix8 = (&matrix7 + (&matrix4 + &matrix5) + (&matrix1 + &matrix2 + &matrix3) + &matrix6)
            .evals();

        let exp_value: i32 = 1000000 + (1000 + 10000) + (1 + 10 + 100) + 100000;

        for val in &matrix8
        {
            assert_eq!(*val, exp_value);
        }
    }


    #[test]
    fn test_add_scalar()
    {
        let matrix1 = SMatrix::<i32, 2, 2>::from_value(10);

        let matrix2 = SMatrix::<i32, 2, 2>::from_value(100);

        let mut matrix4 = SMatrix::<i32, 2, 2>::default();

        matrix4 = (4i32 + (2i32 + &matrix1) + (&matrix2 + 3i32) + 5i32).evals();

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
        let matrix1 = SMatrix::<f64, 2, 2>::from_value(1.0);

        let matrix2 = SMatrix::<f64, 2, 2>::from_value(10.0);

        let matrix3 = SMatrix::<f64, 2, 2>::from_value(100.0);

        let matrix4 = SMatrix::<f64, 2, 2>::from_value(1000.0);

        let matrix5 = SMatrix::<f64, 2, 2>::from_value(10000.0);

        let matrix6 = SMatrix::<f64, 2, 2>::from_value(100000.0);

        let matrix7 = SMatrix::<f64, 2, 2>::from_value(1000000.0);

        let mut matrix8 = SMatrix::<f64, 2, 2>::default();

        matrix8 = (&matrix7 / (&matrix4 / &matrix5) / (&matrix1 / &matrix2 / &matrix3) / &matrix6)
            .evals();

        let exp_value: f64 = 1000000.0 / (1000.0 / 10000.0) / (1.0 / 10.0 / 100.0) / 100000.0;

        for val in &matrix8
        {
            assert_eq!(*val, exp_value);
        }
    }

    #[test]
    fn test_div_scalar()
    {
        let matrix1 = SMatrix::<f64, 2, 2>::from_value(10.0);

        let matrix2 = SMatrix::<f64, 2, 2>::from_value(100.0);

        let mut matrix4 = SMatrix::<f64, 2, 2>::default();

        matrix4 = (4.0 / (2.0 / &matrix1) / (&matrix2 / 3.0) / 5.0).evals();
    }
    //}}}
    //{{{ collection: subtraction tests

    #[test]
    fn test_sub()
    {
        let matrix1 = SMatrix::<i32, 2, 2>::from_value(1);

        let matrix2 = SMatrix::<i32, 2, 2>::from_value(10);

        let matrix3 = SMatrix::<i32, 2, 2>::from_value(100);

        let matrix4 = SMatrix::<i32, 2, 2>::from_value(1000);

        let matrix5 = SMatrix::<i32, 2, 2>::from_value(10000);

        let matrix6 = SMatrix::<i32, 2, 2>::from_value(100000);

        let mut matrix7 = SMatrix::<i32, 2, 2>::default();

        matrix7 = ((&matrix4 - &matrix5) - (&matrix1 - &matrix2 - &matrix3) - &matrix6).evals();

        let exp_value: i32 = (1000 - 10000) - (1 - 10 - 100) - 100000;

        for val in &matrix7
        {
            assert_eq!(*val, exp_value);
        }
    }

    #[test]
    fn test_sub_scalar()
    {
        let matrix1 = SMatrix::<i32, 2, 2>::from_value(10);

        let matrix2 = SMatrix::<i32, 2, 2>::from_value(100);

        let mut matrix4 = SMatrix::<i32, 2, 2>::default();

        matrix4 = (4i32 - (2i32 - &matrix1) - (&matrix2 - 3i32) - 5i32).evals();

        let exp_val = 4 - (2 - 10) - (100 - 3) - 5;

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
        let matrix1 = SMatrix::<f64, 2, 2>::from_value(1.0);

        let matrix2 = SMatrix::<f64, 2, 2>::from_value(10.0);

        let matrix3 = SMatrix::<f64, 2, 2>::from_value(100.0);

        let matrix4 = SMatrix::<f64, 2, 2>::from_value(1000.0);

        let matrix5 = SMatrix::<f64, 2, 2>::from_value(10000.0);

        let matrix6 = SMatrix::<f64, 2, 2>::from_value(100000.0);

        let mut matrix7 = SMatrix::<f64, 2, 2>::default();

        matrix7 = ((&matrix4 * &matrix5) * (&matrix1 * &matrix2 * &matrix3) * &matrix6).evals();

        let exp_value: f64 = (1000.0 * 10000.0) * (1.0 * 10.0 * 100.0) * 100000.0;

        for val in &matrix7
        {
            assert_eq!(*val, exp_value);
        }
    }

    #[test]
    fn test_mul_scalar()
    {
        let matrix1 = SMatrix::<i32, 2, 2>::from_value(10);

        let matrix2 = SMatrix::<i32, 2, 2>::from_value(100);

        let mut matrix4 = SMatrix::<i32, 2, 2>::default();

        matrix4 = (4i32 * (2i32 * &matrix1) * (&matrix2 * 3i32) * 5i32).evals();

        let exp_val = 4 * (2 * 10) * (100 * 3) * 5;

        for val in &matrix4
        {
            assert_eq!(*val, exp_val);
        }
    }
    //}}}
    //{{{ collection: negation tests
    #[test]
    fn test_neg() {

        let a  = SMatrix::<i32, 2, 2>::from_slice_row(&[ 1, 2, 3, 4]);
        let b = -a.clone(); 

        for i in 0..4 {
            assert_eq!(b[i], -a[i]);
        }
    }
    //}}}
}
//}}}
//{{{ mod: dmatrix_tests
mod dmatrix_tests 
{
    use topohedral_linalg::{DMatrix, EvaluateDMatrix};


    //{{{ collection: addition tests
    #[test]
    fn test_add()
    {
        let matrix1 = DMatrix::<i32>::from_value(2, 2, 1);

        let matrix2 = DMatrix::<i32>::from_value(2, 2, 10);

        let matrix3 = DMatrix::<i32>::from_value(2, 2, 100);

        let matrix4 = DMatrix::<i32>::from_value(2, 2, 1000);

        let matrix5 = DMatrix::<i32>::from_value(2, 2, 10000);

        let matrix6 = DMatrix::<i32>::from_value(2, 2, 100000);

        let matrix7 = DMatrix::<i32>::from_value(2, 2, 1000000);

        let mut matrix8 = DMatrix::<i32>::zeros(2, 2);

        matrix8 = (&matrix7 + (&matrix4 + &matrix5) + (&matrix1 + &matrix2 + &matrix3) + &matrix6)
            .evald();

        let exp_value: i32 = 1000000 + (1000 + 10000) + (1 + 10 + 100) + 100000;

        for val in &matrix8
        {
            assert_eq!(*val, exp_value);
        }
    }
    #[test]
    fn test_add_scalar()
    {
        let matrix1 = DMatrix::<i32>::from_value(2, 2, 10);

        let matrix2 = DMatrix::<i32>::from_value(2, 2, 100);

        let mut matrix4 = DMatrix::<i32>::zeros(2, 2);

        matrix4 = (4i32 + (2i32 + &matrix1) + (&matrix2 + 3i32) + 5i32).evald();

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
        let matrix1 = DMatrix::<f64>::from_value(2, 2, 1.0);
        let matrix2 = DMatrix::<f64>::from_value(2, 2, 10.0);
        let matrix3 = DMatrix::<f64>::from_value(2, 2, 100.0);
        let matrix4 = DMatrix::<f64>::from_value(2, 2, 1000.0);
        let matrix5 = DMatrix::<f64>::from_value(2, 2, 10000.0);
        let matrix6 = DMatrix::<f64>::from_value(2, 2, 100000.0);
        let matrix7 = DMatrix::<f64>::from_value(2, 2, 1000000.0);
        let mut matrix8 = DMatrix::<f64>::zeros(2, 2);

        matrix8 = (&matrix7 / (&matrix4 / &matrix5) / (&matrix1 / &matrix2 / &matrix3) / &matrix6)
            .evald();

        let exp_value: f64 = 1000000.0 / (1000.0 / 10000.0) / (1.0 / 10.0 / 100.0) / 100000.0;

        for val in &matrix8
        {
            assert_eq!(*val, exp_value);
        }
    }

    #[test]
    fn test_div_scalar()
    {
        let matrix1 = DMatrix::<f64>::from_value(2, 2, 10.0);
        let matrix2 = DMatrix::<f64>::from_value(2, 2, 100.0);
        let mut matrix4 = DMatrix::<f64>::zeros(2, 2);

        matrix4 = (4.0 / (2.0 / &matrix1) / (&matrix2 / 3.0) / 5.0).evald();

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
        let matrix1 = DMatrix::<f64>::from_value(2, 2, 1.0);
        let matrix2 = DMatrix::<f64>::from_value(2, 2, 10.0);
        let matrix3 = DMatrix::<f64>::from_value(2, 2, 100.0);
        let matrix4 = DMatrix::<f64>::from_value(2, 2, 1000.0);
        let matrix5 = DMatrix::<f64>::from_value(2, 2, 10000.0);
        let matrix6 = DMatrix::<f64>::from_value(2, 2, 100000.0);
        let mut matrix7 = DMatrix::<f64>::zeros(2, 2);

        matrix7 = ((&matrix4 - &matrix5) - (&matrix1 - &matrix2 - &matrix3) - &matrix6).evald();

        let exp_value: f64 = (1000.0 - 10000.0) - (1.0 - 10.0 - 100.0) - 100000.0;

        for val in &matrix7
        {
            assert_eq!(*val, exp_value);
        }
    }



    #[test]
    fn test_sub_scalar()
    {
        let matrix1 = DMatrix::<f64>::from_value(2, 2, 10.0);
        let matrix2 = DMatrix::<f64>::from_value(2, 2, 100.0);
        let mut matrix4 = DMatrix::<f64>::zeros(2, 2);

        matrix4 = (4.0 - (2.0 - &matrix1) - (&matrix2 - 3.0) - 5.0).evald();

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
        let matrix1 = DMatrix::<f64>::from_value(2, 2, 1.0);
        let matrix2 = DMatrix::<f64>::from_value(2, 2, 10.0);
        let matrix3 = DMatrix::<f64>::from_value(2, 2, 100.0);
        let matrix4 = DMatrix::<f64>::from_value(2, 2, 1000.0);
        let matrix5 = DMatrix::<f64>::from_value(2, 2, 10000.0);
        let matrix6 = DMatrix::<f64>::from_value(2, 2, 100000.0);
        let mut matrix7 = DMatrix::<f64>::zeros(2, 2);

        matrix7 = ((&matrix4 * &matrix5) * (&matrix1 * &matrix2 * &matrix3) * &matrix6).evald();

        let exp_value: f64 = (1000.0 * 10000.0) * (1.0 * 10.0 * 100.0) * 100000.0;

        for val in &matrix7
        {
            assert_eq!(*val, exp_value);
        }
    }

    #[test]
    fn test_mul_scalar()
    {
        let matrix1 = DMatrix::<f64>::from_value(2, 2, 10.0);
        let matrix2 = DMatrix::<f64>::from_value(2, 2, 100.0);
        let mut matrix4 = DMatrix::<f64>::zeros(2, 2);

        matrix4 = (4.0 * (2.0 * &matrix1) * (&matrix2 * 3.0) * 5.0).evald();

        let exp_val = 4.0 * (2.0 * 10.0) * (100.0 * 3.0) * 5.0;

        for val in &matrix4
        {
            assert_eq!(*val, exp_val);
        }
    }
    //}}}
    //{{{ collection: negation tests
    #[test]
    fn test_neg() {

        let a  = DMatrix::<i32>::from_slice_row(2, 2, &[ 1, 2, 3, 4]);
        let b = -a.clone(); 

        for i in 0..4 {
            assert_eq!(b[i], -a[i]);
        }
    }
    //}}}
}
//}}}
mod scvector_tests 
{

    use topohedral_linalg::{SCVector, EvaluateSMatrix};
    use approx::assert_relative_eq;

    //{{{ collection: mixed tests
    #[test]
    pub fn test_all()
    {
        let aval = 1.0;
        let a = SCVector::<f64, 10>::from_value(aval);

        let bval = 10.0;
        let b = SCVector::<f64, 10>::from_value(bval);

        let cval = 100.0;
        let c = SCVector::<f64, 10>::from_value(cval);

        let dval = 1000.0;
        let d = SCVector::<f64, 10>::from_value(dval);

        let eval = 10000.0;
        let e = SCVector::<f64, 10>::from_value(eval);

        let fval = 100000.0;
        let f = SCVector::<f64, 10>::from_value(fval);

        let gval = fval * (aval + bval) - (cval / (dval * 2.0)) + 1.0 * eval;
        let g: SCVector<f64, 10> = (&f * (&a + &b) - (&c / (&d * 2.0)) + 1.0 * &e).evals();

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
        let vector1 = SCVector::<i32, 10>::from_value(1);

        let vector2 = SCVector::<i32, 10>::from_value(10);

        let vector3 = SCVector::<i32, 10>::from_value(100);

        let vector4 = SCVector::<i32, 10>::from_value(1000);

        let vector5 = SCVector::<i32, 10>::from_value(10000);

        let vector6 = SCVector::<i32, 10>::from_value(100000);

        let vector7 = SCVector::<i32, 10>::from_value(1000000);

        let mut vector8 = SCVector::<i32, 10>::default();

        vector8 = (&vector7 + (&vector4 + &vector5) + (&vector1 + &vector2 + &vector3) + &vector6)
            .evals();

        let exp_value: i32 = 1000000 + (1000 + 10000) + (1 + 10 + 100) + 100000;

        for val in &vector8
        {
            assert_eq!(*val, exp_value);
        }
    }


    #[test]
    fn test_add_scalar()
    {
        let vector1 = SCVector::<i32, 10>::from_value(10);

        let vector2 = SCVector::<i32, 10>::from_value(100);

        let mut vector4 = SCVector::<i32, 10>::default();

        vector4 = (4i32 + (2i32 + &vector1) + (&vector2 + 3i32) + 5i32).evals();

        let exp_val = 4 + (2 + 10) + (100 + 3) + 5;

        for val in &vector4
        {
            assert_eq!(*val, exp_val);
        }
    }
    //}}}
    //{{{ collection: division tests 
    #[test]
    fn test_div()
    {
        let vector1 = SCVector::<f64, 10>::from_value(1.0);

        let vector2 = SCVector::<f64, 10>::from_value(10.0);

        let vector3 = SCVector::<f64, 10>::from_value(100.0);

        let vector4 = SCVector::<f64, 10>::from_value(1000.0);

        let vector5 = SCVector::<f64, 10>::from_value(10000.0);

        let vector6 = SCVector::<f64, 10>::from_value(100000.0);

        let vector7 = SCVector::<f64, 10>::from_value(1000000.0);

        let mut vector8 = SCVector::<f64, 10>::default();

        vector8 = (&vector7 / (&vector4 / &vector5) / (&vector1 / &vector2 / &vector3) / &vector6)
            .evals();

        let exp_value: f64 = 1000000.0 / (1000.0 / 10000.0) / (1.0 / 10.0 / 100.0) / 100000.0;

        for val in &vector8
        {
            assert_eq!(*val, exp_value);
        }
    }

    #[test]
    fn test_div_scalar()
    {
        let vector1 = SCVector::<f64, 10>::from_value(10.0);

        let vector2 = SCVector::<f64, 10>::from_value(100.0);

        let mut vector4 = SCVector::<f64, 10>::default();

        vector4 = (4.0 / (2.0 / &vector1) / (&vector2 / 3.0) / 5.0).evals();

        let exp_val = 4.0 / (2.0 / 10.0) / (100.0 / 3.0) / 5.0;

        for val in &vector4
        {
            assert_eq!(*val, exp_val);
        }
    }
    //}}}
    //{{{ collection: subtraction tests

    #[test]
    fn test_sub()
    {
        let vector1 = SCVector::<i32, 10>::from_value(1);

        let vector2 = SCVector::<i32, 10>::from_value(10);

        let vector3 = SCVector::<i32, 10>::from_value(100);

        let vector4 = SCVector::<i32, 10>::from_value(1000);

        let vector5 = SCVector::<i32, 10>::from_value(10000);

        let vector6 = SCVector::<i32, 10>::from_value(100000);

        let mut vector7 = SCVector::<i32, 10>::default();

        vector7 = ((&vector4 - &vector5) - (&vector1 - &vector2 - &vector3) - &vector6).evals();

        let exp_value: i32 = (1000 - 10000) - (1 - 10 - 100) - 100000;

        for val in &vector7
        {
            assert_eq!(*val, exp_value);
        }
    }

    #[test]
    fn test_sub_scalar()
    {
        let vector1 = SCVector::<i32, 10>::from_value(10);

        let vector2 = SCVector::<i32, 10>::from_value(100);

        let mut vector4 = SCVector::<i32, 10>::default();

        vector4 = (4i32 - (2i32 - &vector1) - (&vector2 - 3i32) - 5i32).evals();

        let exp_val = 4 - (2 - 10) - (100 - 3) - 5;

        for val in &vector4
        {
            assert_eq!(*val, exp_val);
        }
    }
    //}}}
    //{{{ collection: multiplication tests
    #[test]
    fn test_mul()
    {
        let vector1 = SCVector::<f64, 10>::from_value(1.0);

        let vector2 = SCVector::<f64, 10>::from_value(10.0);

        let vector3 = SCVector::<f64, 10>::from_value(100.0);

        let vector4 = SCVector::<f64, 10>::from_value(1000.0);

        let vector5 = SCVector::<f64, 10>::from_value(10000.0);

        let vector6 = SCVector::<f64, 10>::from_value(100000.0);

        let mut vector7 = SCVector::<f64, 10>::default();

        vector7 = ((&vector4 * &vector5) * (&vector1 * &vector2 * &vector3) * &vector6).evals();

        let exp_value: f64 = (1000.0 * 10000.0) * (1.0 * 10.0 * 100.0) * 100000.0;

        for val in &vector7
        {
            assert_eq!(*val, exp_value);
        }
    }

    #[test]
    fn test_mul_scalar()
    {
        let vector1 = SCVector::<i32, 10>::from_value(10);

        let vector2 = SCVector::<i32, 10>::from_value(100);

        let mut vector4 = SCVector::<i32, 10>::default();

        vector4 = (4i32 * (2i32 * &vector1) * (&vector2 * 3i32) * 5i32).evals();

        let exp_val = 4 * (2 * 10) * (100 * 3) * 5;

        for val in &vector4
        {
            assert_eq!(*val, exp_val);
        }
    }
    //}}}
    //{{{ collection: negation tests
    #[test]
    fn test_neg() {

        let a = SCVector::<i32, 4>::from_slice(&[1, 2, 3, 4]);
        let b = -a.clone();

        for i in 0..4 {
            assert_eq!(b[i], -a[i]);
        }
    }
    //}}}
}
