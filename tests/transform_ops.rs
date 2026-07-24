mod dmatrix_tests {
    use approx::assert_relative_eq;
    use std::f64::consts::{FRAC_PI_2, PI};
    use topohedral_linalg::*;

    fn assert_matrix_eq(
        actual: &DMatrix<i32>,
        expected: &DMatrix<i32>,
    ) {
        assert_eq!(actual.nrows(), expected.nrows());
        assert_eq!(actual.ncols(), expected.ncols());

        for (actual_value, expected_value) in actual.iter().zip(expected.iter()) {
            assert_eq!(*actual_value, *expected_value);
        }
    }

    fn assert_matrix_eq_f64(
        actual: &DMatrix<f64>,
        expected: &DMatrix<f64>,
    ) {
        assert_eq!(actual.nrows(), expected.nrows());
        assert_eq!(actual.ncols(), expected.ncols());

        for (actual_value, expected_value) in actual.iter().zip(expected.iter()) {
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
    fn test_transform_mutates_in_place_and_preserves_shape() {
        let mut matrix = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4, 5, 6], 2, 3);

        matrix.transform_mut(|value| 2 * value + 1);

        let expected = DMatrix::<i32>::from_row_slice(&[3, 5, 7, 9, 11, 13], 2, 3);
        assert_matrix_eq(&matrix, &expected);
        assert_eq!(matrix.nrows(), 2);
        assert_eq!(matrix.ncols(), 3);
    }

    #[test]
    fn test_fill_assigns_all_elements() {
        let mut matrix = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4], 2, 2);

        matrix.fill(7);

        let expected = DMatrix::<i32>::from_row_slice(&[7, 7, 7, 7], 2, 2);
        assert_matrix_eq(&matrix, &expected);
    }

    #[test]
    fn test_transformed_returns_changed_copy_and_leaves_original_unchanged() {
        let matrix = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4], 2, 2);

        let transformed = matrix.to_transformed(|value| 2 * value + 1);

        let expected = DMatrix::<i32>::from_row_slice(&[3, 5, 7, 9], 2, 2);
        let original = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4], 2, 2);

        assert_matrix_eq(&transformed, &expected);
        assert_matrix_eq(&matrix, &original);
    }

    #[test]
    fn test_into_transformed_returns_transformed_owned_value() {
        let matrix = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4], 2, 2);

        let transformed = matrix.into_transformed(|value| 2 * value + 1);

        let expected = DMatrix::<i32>::from_row_slice(&[3, 5, 7, 9], 2, 2);
        assert_matrix_eq(&transformed, &expected);
    }

    #[test]
    fn test_shift_helpers() {
        let mut matrix = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4], 2, 2);

        matrix.shift(3);

        let shifted = matrix.to_shifted(-1);
        let into_shifted = matrix.clone().into_shifted(2);

        let shifted_expected = DMatrix::<i32>::from_row_slice(&[3, 4, 5, 6], 2, 2);
        let into_shifted_expected = DMatrix::<i32>::from_row_slice(&[6, 7, 8, 9], 2, 2);
        let in_place_expected = DMatrix::<i32>::from_row_slice(&[4, 5, 6, 7], 2, 2);

        assert_matrix_eq(&matrix, &in_place_expected);
        assert_matrix_eq(&shifted, &shifted_expected);
        assert_matrix_eq(&into_shifted, &into_shifted_expected);
    }

    #[test]
    fn test_scale_helpers() {
        let mut matrix = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4], 2, 2);

        matrix.scale(3);

        let scaled = matrix.to_scaled(2);
        let into_scaled = matrix.clone().into_scaled(-1);

        let scaled_expected = DMatrix::<i32>::from_row_slice(&[6, 12, 18, 24], 2, 2);
        let into_scaled_expected = DMatrix::<i32>::from_row_slice(&[-3, -6, -9, -12], 2, 2);
        let in_place_expected = DMatrix::<i32>::from_row_slice(&[3, 6, 9, 12], 2, 2);

        assert_matrix_eq(&matrix, &in_place_expected);
        assert_matrix_eq(&scaled, &scaled_expected);
        assert_matrix_eq(&into_scaled, &into_scaled_expected);
    }

    #[test]
    fn test_float_transform_helpers() {
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
        in_place_acos.acos_mut();
        assert_matrix_eq_f64(&in_place_acos, &acos_expected);
        assert_matrix_eq_f64(&acos_input.to_acos(), &acos_expected);
        assert_matrix_eq_f64(&acos_input.clone().into_acos(), &acos_expected);

        let mut in_place_powi = matrix.clone();
        in_place_powi.powi_mut(2);
        assert_matrix_eq_f64(&in_place_powi, &powi_expected);
        assert_matrix_eq_f64(&matrix.to_powi(2), &powi_expected);
        assert_matrix_eq_f64(&matrix.clone().into_powi(2), &powi_expected);

        let mut in_place_clamp = matrix.clone();
        in_place_clamp.clamp_mut(-1.0, 1.0);
        assert_matrix_eq_f64(&in_place_clamp, &clamp_expected);
        assert_matrix_eq_f64(&matrix.to_clamp(-1.0, 1.0), &clamp_expected);
        assert_matrix_eq_f64(&matrix.clone().into_clamp(-1.0, 1.0), &clamp_expected);

        let mut in_place_pos = matrix.clone();
        in_place_pos.pos_mut();
        assert_matrix_eq_f64(&in_place_pos, &pos_expected);
        assert_matrix_eq_f64(&matrix.to_pos(), &pos_expected);
        assert_matrix_eq_f64(&matrix.clone().into_pos(), &pos_expected);

        let mut in_place_neg = matrix.clone();
        in_place_neg.neg_mut();
        assert_matrix_eq_f64(&in_place_neg, &neg_expected);
        assert_matrix_eq_f64(&matrix.to_neg(), &neg_expected);
        assert_matrix_eq_f64(&matrix.clone().into_neg(), &neg_expected);

        let sqrt_input = DMatrix::<f64>::from_row_slice(&[1.0, 4.0, 9.0, 16.0], 2, 2);
        let sqrt_expected = DMatrix::<f64>::from_row_slice(&[1.0, 2.0, 3.0, 4.0], 2, 2);

        let mut in_place_sqrt = sqrt_input.clone();
        in_place_sqrt.sqrt_mut();
        assert_matrix_eq_f64(&in_place_sqrt, &sqrt_expected);
        assert_matrix_eq_f64(&sqrt_input.to_sqrt(), &sqrt_expected);
        assert_matrix_eq_f64(&sqrt_input.into_sqrt(), &sqrt_expected);
    }

    #[test]
    fn test_float_transform_helper_surface_smoke() {
        smoke_float_transform_unary!(abs_mut, to_abs, into_abs, -1.25);
        smoke_float_transform_with_arg!(abs_sub_mut, to_abs_sub, into_abs_sub, 1.25, 0.5);
        smoke_float_transform_unary!(acos_mut, to_acos, into_acos, 0.5);
        smoke_float_transform_unary!(acosh_mut, to_acosh, into_acosh, 1.5);
        smoke_float_transform_unary!(asin_mut, to_asin, into_asin, 0.5);
        smoke_float_transform_unary!(asinh_mut, to_asinh, into_asinh, 1.5);
        smoke_float_transform_unary!(atan_mut, to_atan, into_atan, 0.5);
        smoke_float_transform_with_arg!(atan2_mut, to_atan2, into_atan2, 1.0, 1.0);
        smoke_float_transform_unary!(atanh_mut, to_atanh, into_atanh, 0.25);
        smoke_float_transform_unary!(cbrt_mut, to_cbrt, into_cbrt, 8.0);
        smoke_float_transform_unary!(ceil_mut, to_ceil, into_ceil, 1.25);
        smoke_float_transform_with_two_args!(clamp_mut, to_clamp, into_clamp, 3.0, -1.0, 2.0);
        smoke_float_transform_with_arg!(copysign_mut, to_copysign, into_copysign, 1.25, -1.0);
        smoke_float_transform_unary!(cos_mut, to_cos, into_cos, 0.5);
        smoke_float_transform_unary!(cosh_mut, to_cosh, into_cosh, 0.5);
        smoke_float_transform_with_arg!(div_euclid_mut, to_div_euclid, into_div_euclid, 7.0, 4.0);
        smoke_float_transform_unary!(exp_mut, to_exp, into_exp, 1.0);
        smoke_float_transform_unary!(exp2_mut, to_exp2, into_exp2, 3.0);
        smoke_float_transform_unary!(exp_m1_mut, to_exp_m1, into_exp_m1, 1.0);
        smoke_float_transform_unary!(floor_mut, to_floor, into_floor, 1.75);
        smoke_float_transform_unary!(fract_mut, to_fract, into_fract, 1.75);
        smoke_float_transform_with_arg!(hypot_mut, to_hypot, into_hypot, 3.0, 4.0);
        smoke_float_transform_unary!(ln_mut, to_ln, into_ln, std::f64::consts::E);
        smoke_float_transform_unary!(ln_1p_mut, to_ln_1p, into_ln_1p, 0.5);
        smoke_float_transform_with_arg!(log_mut, to_log, into_log, 8.0, 2.0);
        smoke_float_transform_unary!(log10_mut, to_log10, into_log10, 100.0);
        smoke_float_transform_unary!(log2_mut, to_log2, into_log2, 8.0);
        smoke_float_transform_with_arg!(max_mut, to_max, into_max, 1.25, -0.75);
        smoke_float_transform_with_arg!(midpoint_mut, to_midpoint, into_midpoint, 1.25, -0.75);
        smoke_float_transform_with_arg!(min_mut, to_min, into_min, 1.25, -0.75);
        smoke_float_transform_with_two_args!(mul_add_mut, to_mul_add, into_mul_add, 2.0, 3.0, 4.0);
        smoke_float_transform_unary!(next_down_mut, to_next_down, into_next_down, 1.0);
        smoke_float_transform_unary!(next_up_mut, to_next_up, into_next_up, 1.0);
        smoke_float_transform_with_arg!(powf_mut, to_powf, into_powf, 4.0, 0.5);
        smoke_float_transform_with_arg!(powi_mut, to_powi, into_powi, 4.0, 2);
        smoke_float_transform_unary!(recip_mut, to_recip, into_recip, 4.0);
        smoke_float_transform_with_arg!(rem_euclid_mut, to_rem_euclid, into_rem_euclid, 7.0, 4.0);
        smoke_float_transform_unary!(round_mut, to_round, into_round, 1.5);
        smoke_float_transform_unary!(
            round_ties_even_mut,
            to_round_ties_even,
            into_round_ties_even,
            2.5
        );
        smoke_float_transform_unary!(signum_mut, to_signum, into_signum, -1.0);
        smoke_float_transform_unary!(sin_mut, to_sin, into_sin, 0.5);
        smoke_float_transform_unary!(sinh_mut, to_sinh, into_sinh, 0.5);
        smoke_float_transform_unary!(sqrt_mut, to_sqrt, into_sqrt, 4.0);
        smoke_float_transform_unary!(tan_mut, to_tan, into_tan, 0.5);
        smoke_float_transform_unary!(tanh_mut, to_tanh, into_tanh, 0.5);
        smoke_float_transform_unary!(to_degrees_mut, to_degrees, into_degrees, PI);
        smoke_float_transform_unary!(to_radians_mut, to_radians, into_radians, 180.0);
        smoke_float_transform_unary!(trunc_mut, to_trunc, into_trunc, 1.75);
    }

    #[test]
    fn test_mutable_subview_transform_helpers_only_affect_the_view() {
        let mut matrix = DMatrix::<i32>::from_row_slice(
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            4,
            4,
        );

        {
            let mut view = matrix.subview_range_mut(1, 2, 1, 2);
            view.transform_mut(|value| -value);
        }
        {
            let mut view = matrix.col_mut(0);
            view.scale(2);
        }
        {
            let mut view = matrix.subview_range_mut(0, 1, 3, 3);
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

mod smatrix_tests {
    use approx::assert_relative_eq;
    use std::f64::consts::{FRAC_PI_2, PI};
    use topohedral_linalg::*;

    fn assert_matrix_eq<const N: usize, const M: usize>(
        actual: &SMatrix<i32, N, M>,
        expected: &SMatrix<i32, N, M>,
    ) {
        assert_eq!(actual.nrows(), expected.nrows());
        assert_eq!(actual.ncols(), expected.ncols());

        for (actual_value, expected_value) in actual.iter().zip(expected.iter()) {
            assert_eq!(*actual_value, *expected_value);
        }
    }

    fn assert_matrix_eq_f64<const N: usize, const M: usize>(
        actual: &SMatrix<f64, N, M>,
        expected: &SMatrix<f64, N, M>,
    ) {
        assert_eq!(actual.nrows(), expected.nrows());
        assert_eq!(actual.ncols(), expected.ncols());

        for (actual_value, expected_value) in actual.iter().zip(expected.iter()) {
            assert_relative_eq!(*actual_value, *expected_value, epsilon = 1.0e-12);
        }
    }

    #[test]
    fn test_transform_mutates_in_place_and_preserves_shape() {
        let mut matrix = SMatrix::<i32, 2, 3>::from_row_slice(&[1, 2, 3, 4, 5, 6]);

        matrix.transform_mut(|value| 2 * value + 1);

        let expected = SMatrix::<i32, 2, 3>::from_row_slice(&[3, 5, 7, 9, 11, 13]);
        assert_matrix_eq(&matrix, &expected);
        assert_eq!(matrix.nrows(), 2);
        assert_eq!(matrix.ncols(), 3);
    }

    #[test]
    fn test_fill_assigns_all_elements() {
        let mut matrix = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 2, 3, 4]);

        matrix.fill(7);

        let expected = SMatrix::<i32, 2, 2>::from_row_slice(&[7, 7, 7, 7]);
        assert_matrix_eq(&matrix, &expected);
    }

    #[test]
    fn test_transformed_returns_changed_copy_and_leaves_original_unchanged() {
        let matrix = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 2, 3, 4]);

        let transformed = matrix.to_transformed(|value| 2 * value + 1);

        let expected = SMatrix::<i32, 2, 2>::from_row_slice(&[3, 5, 7, 9]);
        let original = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 2, 3, 4]);

        assert_matrix_eq(&transformed, &expected);
        assert_matrix_eq(&matrix, &original);
    }

    #[test]
    fn test_into_transformed_returns_transformed_owned_value() {
        let matrix = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 2, 3, 4]);

        let transformed = matrix.into_transformed(|value| 2 * value + 1);

        let expected = SMatrix::<i32, 2, 2>::from_row_slice(&[3, 5, 7, 9]);
        assert_matrix_eq(&transformed, &expected);
    }

    #[test]
    fn test_shift_helpers() {
        let mut matrix = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 2, 3, 4]);

        matrix.shift(3);

        let shifted = matrix.to_shifted(-1);
        let into_shifted = matrix.into_shifted(2);

        let shifted_expected = SMatrix::<i32, 2, 2>::from_row_slice(&[3, 4, 5, 6]);
        let into_shifted_expected = SMatrix::<i32, 2, 2>::from_row_slice(&[6, 7, 8, 9]);
        let in_place_expected = SMatrix::<i32, 2, 2>::from_row_slice(&[4, 5, 6, 7]);

        assert_matrix_eq(&matrix, &in_place_expected);
        assert_matrix_eq(&shifted, &shifted_expected);
        assert_matrix_eq(&into_shifted, &into_shifted_expected);
    }

    #[test]
    fn test_scale_helpers() {
        let mut matrix = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 2, 3, 4]);

        matrix.scale(3);

        let scaled = matrix.to_scaled(2);
        let into_scaled = matrix.into_scaled(-1);

        let scaled_expected = SMatrix::<i32, 2, 2>::from_row_slice(&[6, 12, 18, 24]);
        let into_scaled_expected = SMatrix::<i32, 2, 2>::from_row_slice(&[-3, -6, -9, -12]);
        let in_place_expected = SMatrix::<i32, 2, 2>::from_row_slice(&[3, 6, 9, 12]);

        assert_matrix_eq(&matrix, &in_place_expected);
        assert_matrix_eq(&scaled, &scaled_expected);
        assert_matrix_eq(&into_scaled, &into_scaled_expected);
    }

    #[test]
    fn test_float_transform_helpers() {
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
        in_place_acos.acos_mut();
        assert_matrix_eq_f64(&in_place_acos, &acos_expected);
        assert_matrix_eq_f64(&acos_input.to_acos(), &acos_expected);
        assert_matrix_eq_f64(&acos_input.into_acos(), &acos_expected);

        let mut in_place_powi = matrix;
        in_place_powi.powi_mut(2);
        assert_matrix_eq_f64(&in_place_powi, &powi_expected);
        assert_matrix_eq_f64(&matrix.to_powi(2), &powi_expected);
        assert_matrix_eq_f64(&matrix.into_powi(2), &powi_expected);

        let mut in_place_clamp = matrix;
        in_place_clamp.clamp_mut(-1.0, 1.0);
        assert_matrix_eq_f64(&in_place_clamp, &clamp_expected);
        assert_matrix_eq_f64(&matrix.to_clamp(-1.0, 1.0), &clamp_expected);
        assert_matrix_eq_f64(&matrix.into_clamp(-1.0, 1.0), &clamp_expected);

        let mut in_place_pos = matrix;
        in_place_pos.pos_mut();
        assert_matrix_eq_f64(&in_place_pos, &pos_expected);
        assert_matrix_eq_f64(&matrix.to_pos(), &pos_expected);
        assert_matrix_eq_f64(&matrix.into_pos(), &pos_expected);

        let mut in_place_neg = matrix;
        in_place_neg.neg_mut();
        assert_matrix_eq_f64(&in_place_neg, &neg_expected);
        assert_matrix_eq_f64(&matrix.to_neg(), &neg_expected);
        assert_matrix_eq_f64(&matrix.into_neg(), &neg_expected);

        let sqrt_input = SMatrix::<f64, 2, 2>::from_row_slice(&[1.0, 4.0, 9.0, 16.0]);
        let sqrt_expected = SMatrix::<f64, 2, 2>::from_row_slice(&[1.0, 2.0, 3.0, 4.0]);

        let mut in_place_sqrt = sqrt_input;
        in_place_sqrt.sqrt_mut();
        assert_matrix_eq_f64(&in_place_sqrt, &sqrt_expected);
        assert_matrix_eq_f64(&sqrt_input.to_sqrt(), &sqrt_expected);
        assert_matrix_eq_f64(&sqrt_input.into_sqrt(), &sqrt_expected);
    }

    #[test]
    fn test_mutable_subview_transform_helpers_only_affect_the_view() {
        let mut matrix = SMatrix::<i32, 4, 4>::from_row_slice(&[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        ]);

        {
            let mut view = matrix.subview_range_mut(1, 2, 1, 2);
            view.transform_mut(|value| -value);
        }
        {
            let mut view = matrix.col_mut(0);
            view.scale(2);
        }
        {
            let mut view = matrix.subview_range_mut(0, 1, 3, 3);
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
