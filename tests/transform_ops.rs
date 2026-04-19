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

        assert_matrix_eq_f64(&acos_input.acos(), &acos_expected);
        assert_matrix_eq_f64(&matrix.powi(2), &powi_expected);
        assert_matrix_eq_f64(&matrix.clamp(-1.0, 1.0), &clamp_expected);

        let sqrt_input = DMatrix::<f64>::from_row_slice(&[1.0, 4.0, 9.0, 16.0], 2, 2);
        let sqrt_expected = DMatrix::<f64>::from_row_slice(&[1.0, 2.0, 3.0, 4.0], 2, 2);
        assert_matrix_eq_f64(&sqrt_input.sqrt(), &sqrt_expected);
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

        assert_matrix_eq_f64(&acos_input.acos(), &acos_expected);
        assert_matrix_eq_f64(&matrix.powi(2), &powi_expected);
        assert_matrix_eq_f64(&matrix.clamp(-1.0, 1.0), &clamp_expected);

        let sqrt_input = SMatrix::<f64, 2, 2>::from_row_slice(&[1.0, 4.0, 9.0, 16.0]);
        let sqrt_expected = SMatrix::<f64, 2, 2>::from_row_slice(&[1.0, 2.0, 3.0, 4.0]);
        assert_matrix_eq_f64(&sqrt_input.sqrt(), &sqrt_expected);
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
