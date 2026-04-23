#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

mod dmatrix_tests
{
    use approx::assert_relative_eq;
    use std::f64::consts::{FRAC_PI_2, PI};
    use topohedral_linalg::dmatrix::DMatrix;
    use topohedral_linalg::{FloatTransformOps, Shape, TransformOps};

    fn assert_matrix_eq(
        actual: &DMatrix<i32>,
        expected: &DMatrix<i32>,
    )
    {
        assert_eq!(actual.nrows(), expected.nrows());
        assert_eq!(actual.ncols(), expected.ncols());

        for (actual_value, expected_value) in actual.iter().zip(expected.iter())
        {
            assert_eq!(*actual_value, *expected_value);
        }
    }

    fn assert_matrix_eq_f64(
        actual: &DMatrix<f64>,
        expected: &DMatrix<f64>,
    )
    {
        assert_eq!(actual.nrows(), expected.nrows());
        assert_eq!(actual.ncols(), expected.ncols());

        for (actual_value, expected_value) in actual.iter().zip(expected.iter())
        {
            assert_relative_eq!(*actual_value, *expected_value, epsilon = 1.0e-12);
        }
    }

    macro_rules! smoke_float_transform_unary {
        ($method:ident, $methoded:ident, $into_methoded:ident, $value:expr) => {{
            let mut matrix = DMatrix::<f64>::from_row_slice(&[$value], 1, 1);
            matrix.$method();

            let matrix = DMatrix::<f64>::from_row_slice(&[$value], 1, 1);
            let _ = matrix.$methoded();

            let matrix = DMatrix::<f64>::from_row_slice(&[$value], 1, 1);
            let _ = matrix.$into_methoded();
        }};
    }

    macro_rules! smoke_float_transform_with_arg {
        ($method:ident, $methoded:ident, $into_methoded:ident, $value:expr, $arg:expr) => {{
            let mut matrix = DMatrix::<f64>::from_row_slice(&[$value], 1, 1);
            matrix.$method($arg);

            let matrix = DMatrix::<f64>::from_row_slice(&[$value], 1, 1);
            let _ = matrix.$methoded($arg);

            let matrix = DMatrix::<f64>::from_row_slice(&[$value], 1, 1);
            let _ = matrix.$into_methoded($arg);
        }};
    }

    macro_rules! smoke_float_transform_with_two_args {
        (
            $method:ident,
            $methoded:ident,
            $into_methoded:ident,
            $value:expr,
            $arg1:expr,
            $arg2:expr
        ) => {{
            let mut matrix = DMatrix::<f64>::from_row_slice(&[$value], 1, 1);
            matrix.$method($arg1, $arg2);

            let matrix = DMatrix::<f64>::from_row_slice(&[$value], 1, 1);
            let _ = matrix.$methoded($arg1, $arg2);

            let matrix = DMatrix::<f64>::from_row_slice(&[$value], 1, 1);
            let _ = matrix.$into_methoded($arg1, $arg2);
        }};
    }

    #[test]
    fn test_transform_mutates_in_place_and_preserves_shape()
    {
        let mut matrix = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4, 5, 6], 2, 3);

        matrix.transform(|value| 2 * value + 1);

        let expected = DMatrix::<i32>::from_row_slice(&[3, 5, 7, 9, 11, 13], 2, 3);
        assert_matrix_eq(&matrix, &expected);
        assert_eq!(matrix.nrows(), 2);
        assert_eq!(matrix.ncols(), 3);
    }

    #[test]
    fn test_fill_assigns_all_elements()
    {
        let mut matrix = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4], 2, 2);

        matrix.fill(7);

        let expected = DMatrix::<i32>::from_row_slice(&[7, 7, 7, 7], 2, 2);
        assert_matrix_eq(&matrix, &expected);
    }

    #[test]
    fn test_transformed_returns_changed_copy_and_leaves_original_unchanged()
    {
        let matrix = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4], 2, 2);

        let transformed = matrix.transformed(|value| 2 * value + 1);

        let expected = DMatrix::<i32>::from_row_slice(&[3, 5, 7, 9], 2, 2);
        let original = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4], 2, 2);

        assert_matrix_eq(&transformed, &expected);
        assert_matrix_eq(&matrix, &original);
    }

    #[test]
    fn test_into_transformed_returns_transformed_owned_value()
    {
        let matrix = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4], 2, 2);

        let transformed = matrix.into_transformed(|value| 2 * value + 1);

        let expected = DMatrix::<i32>::from_row_slice(&[3, 5, 7, 9], 2, 2);
        assert_matrix_eq(&transformed, &expected);
    }

    #[test]
    fn test_shift_helpers()
    {
        let mut matrix = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4], 2, 2);

        matrix.shift(3);

        let shifted = matrix.shifted(-1);
        let into_shifted = matrix.clone().into_shifted(2);

        let shifted_expected = DMatrix::<i32>::from_row_slice(&[3, 4, 5, 6], 2, 2);
        let into_shifted_expected = DMatrix::<i32>::from_row_slice(&[6, 7, 8, 9], 2, 2);
        let in_place_expected = DMatrix::<i32>::from_row_slice(&[4, 5, 6, 7], 2, 2);

        assert_matrix_eq(&matrix, &in_place_expected);
        assert_matrix_eq(&shifted, &shifted_expected);
        assert_matrix_eq(&into_shifted, &into_shifted_expected);
    }

    #[test]
    fn test_scale_helpers()
    {
        let mut matrix = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4], 2, 2);

        matrix.scale(3);

        let scaled = matrix.scaled(2);
        let into_scaled = matrix.clone().into_scaled(-1);

        let scaled_expected = DMatrix::<i32>::from_row_slice(&[6, 12, 18, 24], 2, 2);
        let into_scaled_expected = DMatrix::<i32>::from_row_slice(&[-3, -6, -9, -12], 2, 2);
        let in_place_expected = DMatrix::<i32>::from_row_slice(&[3, 6, 9, 12], 2, 2);

        assert_matrix_eq(&matrix, &in_place_expected);
        assert_matrix_eq(&scaled, &scaled_expected);
        assert_matrix_eq(&into_scaled, &into_scaled_expected);
    }

    #[test]
    fn test_float_transform_helpers()
    {
        let acos_input = DMatrix::<f64>::from_row_slice(&[1.0, 0.0, -1.0, -1.0, 0.0, 1.0], 2, 3);
        let acos_expected =
            DMatrix::<f64>::from_row_slice(&[0.0, FRAC_PI_2, PI, PI, FRAC_PI_2, 0.0], 2, 3);
        let matrix = DMatrix::<f64>::from_row_slice(&[1.0, 0.0, -1.0, 4.0, -4.0, 9.0], 2, 3);
        let powi_expected =
            DMatrix::<f64>::from_row_slice(&[1.0, 0.0, 1.0, 16.0, 16.0, 81.0], 2, 3);
        let clamp_expected =
            DMatrix::<f64>::from_row_slice(&[1.0, 0.0, -1.0, 1.0, -1.0, 1.0], 2, 3);
        let pos_expected = DMatrix::<f64>::from_row_slice(&[1.0, 0.0, 0.0, 4.0, 0.0, 9.0], 2, 3);
        let neg_expected = DMatrix::<f64>::from_row_slice(&[0.0, 0.0, -1.0, 0.0, -4.0, 0.0], 2, 3);

        let mut in_place_acos = acos_input.clone();
        in_place_acos.acos();
        assert_matrix_eq_f64(&in_place_acos, &acos_expected);
        assert_matrix_eq_f64(&acos_input.acosed(), &acos_expected);
        assert_matrix_eq_f64(&acos_input.clone().into_acosed(), &acos_expected);

        let mut in_place_powi = matrix.clone();
        in_place_powi.powi(2);
        assert_matrix_eq_f64(&in_place_powi, &powi_expected);
        assert_matrix_eq_f64(&matrix.powied(2), &powi_expected);
        assert_matrix_eq_f64(&matrix.clone().into_powied(2), &powi_expected);

        let mut in_place_clamp = matrix.clone();
        in_place_clamp.clamp(-1.0, 1.0);
        assert_matrix_eq_f64(&in_place_clamp, &clamp_expected);
        assert_matrix_eq_f64(&matrix.clamped(-1.0, 1.0), &clamp_expected);
        assert_matrix_eq_f64(&matrix.clone().into_clamped(-1.0, 1.0), &clamp_expected);

        let mut in_place_pos = matrix.clone();
        in_place_pos.pos();
        assert_matrix_eq_f64(&in_place_pos, &pos_expected);
        assert_matrix_eq_f64(&matrix.posed(), &pos_expected);
        assert_matrix_eq_f64(&matrix.clone().into_posed(), &pos_expected);

        let mut in_place_neg = matrix.clone();
        in_place_neg.neg();
        assert_matrix_eq_f64(&in_place_neg, &neg_expected);
        assert_matrix_eq_f64(&matrix.neged(), &neg_expected);
        assert_matrix_eq_f64(&matrix.clone().into_neged(), &neg_expected);

        let sqrt_input = DMatrix::<f64>::from_row_slice(&[1.0, 4.0, 9.0, 16.0], 2, 2);
        let sqrt_expected = DMatrix::<f64>::from_row_slice(&[1.0, 2.0, 3.0, 4.0], 2, 2);

        let mut in_place_sqrt = sqrt_input.clone();
        in_place_sqrt.sqrt();
        assert_matrix_eq_f64(&in_place_sqrt, &sqrt_expected);
        assert_matrix_eq_f64(&sqrt_input.sqrted(), &sqrt_expected);
        assert_matrix_eq_f64(&sqrt_input.into_sqrted(), &sqrt_expected);
    }

    #[test]
    fn test_float_transform_helper_surface_smoke()
    {
        smoke_float_transform_unary!(abs, absed, into_absed, -1.25);
        smoke_float_transform_with_arg!(abs_sub, abs_subed, into_abs_subed, 1.25, 0.5);
        smoke_float_transform_unary!(acos, acosed, into_acosed, 0.5);
        smoke_float_transform_unary!(acosh, acoshed, into_acoshed, 1.5);
        smoke_float_transform_unary!(asin, asined, into_asined, 0.5);
        smoke_float_transform_unary!(asinh, asinhed, into_asinhed, 1.5);
        smoke_float_transform_unary!(atan, ataned, into_ataned, 0.5);
        smoke_float_transform_with_arg!(atan2, atan2ed, into_atan2ed, 1.0, 1.0);
        smoke_float_transform_unary!(atanh, atanhed, into_atanhed, 0.25);
        smoke_float_transform_unary!(cbrt, cbrted, into_cbrted, 8.0);
        smoke_float_transform_unary!(ceil, ceiled, into_ceiled, 1.25);
        smoke_float_transform_with_two_args!(clamp, clamped, into_clamped, 3.0, -1.0, 2.0);
        smoke_float_transform_with_arg!(
            clamp_magnitude,
            clamp_magnituded,
            into_clamp_magnituded,
            -5.0,
            3.0
        );
        smoke_float_transform_with_arg!(copysign, copysigned, into_copysigned, 1.25, -1.0);
        smoke_float_transform_unary!(cos, cosed, into_cosed, 0.5);
        smoke_float_transform_unary!(cosh, coshed, into_coshed, 0.5);
        smoke_float_transform_with_arg!(div_euclid, div_euclided, into_div_euclided, 7.0, 4.0);
        smoke_float_transform_unary!(erf, erfed, into_erfed, 0.0);
        smoke_float_transform_unary!(erfc, erfced, into_erfced, 0.0);
        smoke_float_transform_unary!(exp, exped, into_exped, 1.0);
        smoke_float_transform_unary!(exp2, exp2ed, into_exp2ed, 3.0);
        smoke_float_transform_unary!(exp_m1, exp_m1ed, into_exp_m1ed, 1.0);
        smoke_float_transform_unary!(floor, floored, into_floored, 1.75);
        smoke_float_transform_unary!(fract, fracted, into_fracted, 1.75);
        smoke_float_transform_unary!(gamma, gammaed, into_gammaed, 5.0);
        smoke_float_transform_with_arg!(hypot, hypoted, into_hypoted, 3.0, 4.0);
        smoke_float_transform_unary!(ln, lned, into_lned, std::f64::consts::E);
        smoke_float_transform_unary!(ln_1p, ln_1ped, into_ln_1ped, 0.5);
        smoke_float_transform_with_arg!(log, loged, into_loged, 8.0, 2.0);
        smoke_float_transform_unary!(log10, log10ed, into_log10ed, 100.0);
        smoke_float_transform_unary!(log2, log2ed, into_log2ed, 8.0);
        smoke_float_transform_with_arg!(max, maxed, into_maxed, 1.25, -0.75);
        smoke_float_transform_with_arg!(maximum, maximumed, into_maximumed, 1.25, -0.75);
        smoke_float_transform_with_arg!(midpoint, midpointed, into_midpointed, 1.25, -0.75);
        smoke_float_transform_with_arg!(min, mined, into_mined, 1.25, -0.75);
        smoke_float_transform_with_arg!(minimum, minimumed, into_minimumed, 1.25, -0.75);
        smoke_float_transform_with_two_args!(mul_add, mul_added, into_mul_added, 2.0, 3.0, 4.0);
        smoke_float_transform_unary!(next_down, next_downed, into_next_downed, 1.0);
        smoke_float_transform_unary!(next_up, next_uped, into_next_uped, 1.0);
        smoke_float_transform_with_arg!(powf, powfed, into_powfed, 4.0, 0.5);
        smoke_float_transform_with_arg!(powi, powied, into_powied, 4.0, 2);
        smoke_float_transform_unary!(recip, reciped, into_reciped, 4.0);
        smoke_float_transform_with_arg!(rem_euclid, rem_euclided, into_rem_euclided, 7.0, 4.0);
        smoke_float_transform_unary!(round, rounded, into_rounded, 1.5);
        smoke_float_transform_unary!(
            round_ties_even,
            round_ties_evened,
            into_round_ties_evened,
            2.5
        );
        smoke_float_transform_unary!(signum, signumed, into_signumed, -1.0);
        smoke_float_transform_unary!(sin, sined, into_sined, 0.5);
        smoke_float_transform_unary!(sinh, sinhed, into_sinhed, 0.5);
        smoke_float_transform_unary!(sqrt, sqrted, into_sqrted, 4.0);
        smoke_float_transform_unary!(tan, taned, into_taned, 0.5);
        smoke_float_transform_unary!(tanh, tanhed, into_tanhed, 0.5);
        smoke_float_transform_unary!(to_degrees, to_degreesed, into_to_degreesed, PI);
        smoke_float_transform_unary!(to_radians, to_radiansed, into_to_radiansed, 180.0);
        smoke_float_transform_unary!(trunc, trunced, into_trunced, 1.75);
        smoke_float_transform_with_arg!(
            algebraic_add,
            algebraic_added,
            into_algebraic_added,
            1.5,
            2.0
        );
        smoke_float_transform_with_arg!(
            algebraic_sub,
            algebraic_subed,
            into_algebraic_subed,
            1.5,
            2.0
        );
        smoke_float_transform_with_arg!(
            algebraic_mul,
            algebraic_muled,
            into_algebraic_muled,
            1.5,
            2.0
        );
        smoke_float_transform_with_arg!(
            algebraic_div,
            algebraic_dived,
            into_algebraic_dived,
            3.0,
            2.0
        );
        smoke_float_transform_with_arg!(
            algebraic_rem,
            algebraic_remed,
            into_algebraic_remed,
            7.0,
            4.0
        );
    }

    #[test]
    fn test_mutable_subview_transform_helpers_only_affect_the_view()
    {
        let mut matrix = DMatrix::<i32>::from_row_slice(
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            4,
            4,
        );

        {
            let mut view = matrix.subview_mut(1, 2, 1, 2);
            view.transform(|value| -value);
        }
        {
            let mut view = matrix.col_mut(0);
            view.scale(2);
        }
        {
            let mut view = matrix.subview_mut(0, 1, 3, 3);
            view.shift(100);
        }
        {
            let mut view = matrix.row_mut(3);
            view.fill(0);
        }

        let expected = DMatrix::<i32>::from_row_slice(
            &[2, 2, 3, 104, 10, -6, -7, 108, 18, -10, -11, 12, 0, 0, 0, 0],
            4,
            4,
        );

        assert_matrix_eq(&matrix, &expected);
        assert_eq!(matrix.nrows(), 4);
        assert_eq!(matrix.ncols(), 4);
    }
}

mod smatrix_tests
{
    use approx::assert_relative_eq;
    use std::f64::consts::{FRAC_PI_2, PI};
    use topohedral_linalg::smatrix::SMatrix;
    use topohedral_linalg::{FloatTransformOps, Shape, TransformOps};

    fn assert_matrix_eq<const N: usize, const M: usize>(
        actual: &SMatrix<i32, N, M>,
        expected: &SMatrix<i32, N, M>,
    ) where
        [(); N * M]:,
    {
        assert_eq!(actual.nrows(), expected.nrows());
        assert_eq!(actual.ncols(), expected.ncols());

        for (actual_value, expected_value) in actual.iter().zip(expected.iter())
        {
            assert_eq!(*actual_value, *expected_value);
        }
    }

    fn assert_matrix_eq_f64<const N: usize, const M: usize>(
        actual: &SMatrix<f64, N, M>,
        expected: &SMatrix<f64, N, M>,
    ) where
        [(); N * M]:,
    {
        assert_eq!(actual.nrows(), expected.nrows());
        assert_eq!(actual.ncols(), expected.ncols());

        for (actual_value, expected_value) in actual.iter().zip(expected.iter())
        {
            assert_relative_eq!(*actual_value, *expected_value, epsilon = 1.0e-12);
        }
    }

    #[test]
    fn test_transform_mutates_in_place_and_preserves_shape()
    {
        let mut matrix = SMatrix::<i32, 2, 3>::from_row_slice(&[1, 2, 3, 4, 5, 6]);

        matrix.transform(|value| 2 * value + 1);

        let expected = SMatrix::<i32, 2, 3>::from_row_slice(&[3, 5, 7, 9, 11, 13]);
        assert_matrix_eq(&matrix, &expected);
        assert_eq!(matrix.nrows(), 2);
        assert_eq!(matrix.ncols(), 3);
    }

    #[test]
    fn test_fill_assigns_all_elements()
    {
        let mut matrix = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 2, 3, 4]);

        matrix.fill(7);

        let expected = SMatrix::<i32, 2, 2>::from_row_slice(&[7, 7, 7, 7]);
        assert_matrix_eq(&matrix, &expected);
    }

    #[test]
    fn test_transformed_returns_changed_copy_and_leaves_original_unchanged()
    {
        let matrix = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 2, 3, 4]);

        let transformed = matrix.transformed(|value| 2 * value + 1);

        let expected = SMatrix::<i32, 2, 2>::from_row_slice(&[3, 5, 7, 9]);
        let original = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 2, 3, 4]);

        assert_matrix_eq(&transformed, &expected);
        assert_matrix_eq(&matrix, &original);
    }

    #[test]
    fn test_into_transformed_returns_transformed_owned_value()
    {
        let matrix = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 2, 3, 4]);

        let transformed = matrix.into_transformed(|value| 2 * value + 1);

        let expected = SMatrix::<i32, 2, 2>::from_row_slice(&[3, 5, 7, 9]);
        assert_matrix_eq(&transformed, &expected);
    }

    #[test]
    fn test_shift_helpers()
    {
        let mut matrix = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 2, 3, 4]);

        matrix.shift(3);

        let shifted = matrix.shifted(-1);
        let into_shifted = matrix.into_shifted(2);

        let shifted_expected = SMatrix::<i32, 2, 2>::from_row_slice(&[3, 4, 5, 6]);
        let into_shifted_expected = SMatrix::<i32, 2, 2>::from_row_slice(&[6, 7, 8, 9]);
        let in_place_expected = SMatrix::<i32, 2, 2>::from_row_slice(&[4, 5, 6, 7]);

        assert_matrix_eq(&matrix, &in_place_expected);
        assert_matrix_eq(&shifted, &shifted_expected);
        assert_matrix_eq(&into_shifted, &into_shifted_expected);
    }

    #[test]
    fn test_scale_helpers()
    {
        let mut matrix = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 2, 3, 4]);

        matrix.scale(3);

        let scaled = matrix.scaled(2);
        let into_scaled = matrix.into_scaled(-1);

        let scaled_expected = SMatrix::<i32, 2, 2>::from_row_slice(&[6, 12, 18, 24]);
        let into_scaled_expected = SMatrix::<i32, 2, 2>::from_row_slice(&[-3, -6, -9, -12]);
        let in_place_expected = SMatrix::<i32, 2, 2>::from_row_slice(&[3, 6, 9, 12]);

        assert_matrix_eq(&matrix, &in_place_expected);
        assert_matrix_eq(&scaled, &scaled_expected);
        assert_matrix_eq(&into_scaled, &into_scaled_expected);
    }

    #[test]
    fn test_float_transform_helpers()
    {
        let acos_input = SMatrix::<f64, 2, 3>::from_row_slice(&[1.0, 0.0, -1.0, -1.0, 0.0, 1.0]);
        let acos_expected =
            SMatrix::<f64, 2, 3>::from_row_slice(&[0.0, FRAC_PI_2, PI, PI, FRAC_PI_2, 0.0]);
        let matrix = SMatrix::<f64, 2, 3>::from_row_slice(&[1.0, 0.0, -1.0, 4.0, -4.0, 9.0]);
        let powi_expected =
            SMatrix::<f64, 2, 3>::from_row_slice(&[1.0, 0.0, 1.0, 16.0, 16.0, 81.0]);
        let clamp_expected =
            SMatrix::<f64, 2, 3>::from_row_slice(&[1.0, 0.0, -1.0, 1.0, -1.0, 1.0]);
        let pos_expected = SMatrix::<f64, 2, 3>::from_row_slice(&[1.0, 0.0, 0.0, 4.0, 0.0, 9.0]);
        let neg_expected = SMatrix::<f64, 2, 3>::from_row_slice(&[0.0, 0.0, -1.0, 0.0, -4.0, 0.0]);

        let mut in_place_acos = acos_input;
        in_place_acos.acos();
        assert_matrix_eq_f64(&in_place_acos, &acos_expected);
        assert_matrix_eq_f64(&acos_input.acosed(), &acos_expected);
        assert_matrix_eq_f64(&acos_input.into_acosed(), &acos_expected);

        let mut in_place_powi = matrix;
        in_place_powi.powi(2);
        assert_matrix_eq_f64(&in_place_powi, &powi_expected);
        assert_matrix_eq_f64(&matrix.powied(2), &powi_expected);
        assert_matrix_eq_f64(&matrix.into_powied(2), &powi_expected);

        let mut in_place_clamp = matrix;
        in_place_clamp.clamp(-1.0, 1.0);
        assert_matrix_eq_f64(&in_place_clamp, &clamp_expected);
        assert_matrix_eq_f64(&matrix.clamped(-1.0, 1.0), &clamp_expected);
        assert_matrix_eq_f64(&matrix.into_clamped(-1.0, 1.0), &clamp_expected);

        let mut in_place_pos = matrix;
        in_place_pos.pos();
        assert_matrix_eq_f64(&in_place_pos, &pos_expected);
        assert_matrix_eq_f64(&matrix.posed(), &pos_expected);
        assert_matrix_eq_f64(&matrix.into_posed(), &pos_expected);

        let mut in_place_neg = matrix;
        in_place_neg.neg();
        assert_matrix_eq_f64(&in_place_neg, &neg_expected);
        assert_matrix_eq_f64(&matrix.neged(), &neg_expected);
        assert_matrix_eq_f64(&matrix.into_neged(), &neg_expected);

        let sqrt_input = SMatrix::<f64, 2, 2>::from_row_slice(&[1.0, 4.0, 9.0, 16.0]);
        let sqrt_expected = SMatrix::<f64, 2, 2>::from_row_slice(&[1.0, 2.0, 3.0, 4.0]);

        let mut in_place_sqrt = sqrt_input;
        in_place_sqrt.sqrt();
        assert_matrix_eq_f64(&in_place_sqrt, &sqrt_expected);
        assert_matrix_eq_f64(&sqrt_input.sqrted(), &sqrt_expected);
        assert_matrix_eq_f64(&sqrt_input.into_sqrted(), &sqrt_expected);
    }

    #[test]
    fn test_mutable_subview_transform_helpers_only_affect_the_view()
    {
        let mut matrix = SMatrix::<i32, 4, 4>::from_row_slice(&[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        ]);

        {
            let mut view = matrix.subview_mut(1, 2, 1, 2);
            view.transform(|value| -value);
        }
        {
            let mut view = matrix.col_mut(0);
            view.scale(2);
        }
        {
            let mut view = matrix.subview_mut(0, 1, 3, 3);
            view.shift(100);
        }
        {
            let mut view = matrix.row_mut(3);
            view.fill(0);
        }

        let expected = SMatrix::<i32, 4, 4>::from_row_slice(&[
            2, 2, 3, 104, 10, -6, -7, 108, 18, -10, -11, 12, 0, 0, 0, 0,
        ]);

        assert_matrix_eq(&matrix, &expected);
        assert_eq!(matrix.nrows(), 4);
        assert_eq!(matrix.ncols(), 4);
    }
}
