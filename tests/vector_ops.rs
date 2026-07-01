mod scvector_tests {
    use approx::assert_relative_eq;
    use topohedral_linalg::*;

    fn assert_smatrix_relative_eq<const N: usize, const M: usize>(
        actual: &SMatrix<f64, N, M>,
        expected: &SMatrix<f64, N, M>,
    ) {
        for i in 0..actual.nrows() {
            for j in 0..actual.ncols() {
                assert_relative_eq!(actual[(i, j)], expected[(i, j)]);
            }
        }
    }

    #[test]
    fn test_norm() {
        let v = SCVector::<f64, 3>::from_col_slice(&[1.0, 2.0, 3.0]);
        let norm = v.norm();
        assert_relative_eq!(norm, 3.7416573867739413);
    }

    #[test]
    fn test_dot() {
        let v1 = SCVector::<f64, 3>::from_col_slice(&[1.0, 2.0, 3.0]);
        let v2 = SCVector::<f64, 3>::from_col_slice(&[4.0, 5.0, 6.0]);
        let dot = v1.dot(&v2);
        assert_relative_eq!(dot, 32.0);
    }

    #[test]
    fn test_normalize() {
        let v = SCVector::<f64, 3>::from_col_slice(&[1.0, 2.0, 3.0]);
        let norm = v.norm();
        let normalized = v.normalize();
        assert_relative_eq!(normalized[0], 1.0 / norm);
        assert_relative_eq!(normalized[1], 2.0 / norm);
        assert_relative_eq!(normalized[2], 3.0 / norm);
    }

    #[test]
    fn test_cross() {
        let v1 = SCVector::<f64, 3>::from_col_slice(&[1.0, 2.0, 3.0]);
        let v2 = SCVector::<f64, 3>::from_col_slice(&[4.0, 5.0, 6.0]);
        let cross = v1.cross(&v2);
        assert_relative_eq!(cross[0], -3.0);
        assert_relative_eq!(cross[1], 6.0);
        assert_relative_eq!(cross[2], -3.0);
    }

    #[test]
    fn test_angle() {
        let v1 = SCVector::<f64, 3>::from_col_slice(&[1.0, 0.0, 0.0]);
        let v2 = SCVector::<f64, 3>::from_col_slice(&[0.0, 1.0, 0.0]);
        let angle1 = v1.angle(&v2);
        assert_relative_eq!(angle1, std::f64::consts::FRAC_PI_2);
        let angle2 = v2.angle(&v1);
        assert_relative_eq!(angle2, std::f64::consts::FRAC_PI_2);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn test_static_outer_expression_assign_arithmetic() {
        let b = SMatrix::<f64, 2, 3>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let v = SCVector::<f64, 2>::from_col_slice(&[1.0, 2.0]);
        let w = SCVector::<f64, 3>::from_col_slice(&[10.0, 20.0, 30.0]);
        let expr_expected =
            SMatrix::<f64, 2, 3>::from_row_slice(&[11.0, 22.0, 33.0, 24.0, 45.0, 66.0]);

        let mut add = SMatrix::<f64, 2, 3>::zeros();
        add += &b + v.outer(&w);
        assert_smatrix_relative_eq(&add, &expr_expected);

        let mut sub = SMatrix::<f64, 2, 3>::from_value(100.0);
        sub -= &b + v.outer(&w);
        let sub_expected =
            SMatrix::<f64, 2, 3>::from_row_slice(&[89.0, 78.0, 67.0, 76.0, 55.0, 34.0]);
        assert_smatrix_relative_eq(&sub, &sub_expected);

        let mut mul = SMatrix::<f64, 2, 3>::from_value(1.0);
        mul *= &b + v.outer(&w);
        assert_smatrix_relative_eq(&mul, &expr_expected);

        let mut div = SMatrix::<f64, 2, 3>::from_row_slice(&[22.0, 44.0, 66.0, 48.0, 90.0, 132.0]);
        div /= &b + v.outer(&w);
        let div_expected = SMatrix::<f64, 2, 3>::from_value(2.0);
        assert_smatrix_relative_eq(&div, &div_expected);
    }
}

mod srvector_tests {

    use approx::assert_relative_eq;
    use topohedral_linalg::*;

    #[test]
    fn test_norm() {
        let v = SRVector::<f64, 3>::from_row_slice(&[1.0, 2.0, 3.0]);
        let norm = v.norm();
        assert_relative_eq!(norm, 3.7416573867739413);
    }

    #[test]
    fn test_dot() {
        let v1 = SRVector::<f64, 3>::from_row_slice(&[1.0, 2.0, 3.0]);
        let v2 = SRVector::<f64, 3>::from_row_slice(&[4.0, 5.0, 6.0]);
        let dot = v1.dot(&v2);
        assert_relative_eq!(dot, 32.0);
    }

    #[test]
    fn test_normalize() {
        let v = SRVector::<f64, 3>::from_row_slice(&[1.0, 2.0, 3.0]);
        let norm = v.norm();
        let normalized = v.normalize();
        assert_relative_eq!(normalized[0], 1.0 / norm);
        assert_relative_eq!(normalized[1], 2.0 / norm);
        assert_relative_eq!(normalized[2], 3.0 / norm);
    }

    #[test]
    fn test_cross() {
        let v1 = SRVector::<f64, 3>::from_row_slice(&[1.0, 2.0, 3.0]);
        let v2 = SRVector::<f64, 3>::from_row_slice(&[4.0, 5.0, 6.0]);
        let cross = v1.cross(&v2);
        assert_relative_eq!(cross[0], -3.0);
        assert_relative_eq!(cross[1], 6.0);
        assert_relative_eq!(cross[2], -3.0);
    }

    #[test]
    fn test_angle() {
        let v1 = SRVector::<f64, 3>::from_row_slice(&[1.0, 0.0, 0.0]);
        let v2 = SRVector::<f64, 3>::from_row_slice(&[0.0, 1.0, 0.0]);
        let angle1 = v1.angle(&v2);
        assert_relative_eq!(angle1, std::f64::consts::FRAC_PI_2);
        let angle2 = v2.angle(&v1);
        assert_relative_eq!(angle2, std::f64::consts::FRAC_PI_2);
    }
}
mod dvector_tests {
    use approx::assert_relative_eq;
    use topohedral_linalg::*;

    fn assert_matrix_relative_eq(
        actual: &DMatrix<f64>,
        expected: &DMatrix<f64>,
    ) {
        assert_eq!(actual.nrows(), expected.nrows());
        assert_eq!(actual.ncols(), expected.ncols());
        for i in 0..actual.nrows() {
            for j in 0..actual.ncols() {
                assert_relative_eq!(actual[(i, j)], expected[(i, j)]);
            }
        }
    }

    #[test]
    fn test_norm() {
        {
            let v = DVector::<f64>::from_slice_vec(&[1.0, 2.0, 3.0], 3, VecType::Row);
            let norm = v.norm();
            assert_relative_eq!(norm, 3.7416573867739413);
        }
        {
            let v = DVector::<f64>::from_slice_vec(&[1.0, 2.0, 3.0], 3, VecType::Col);
            let norm = v.norm();
            assert_relative_eq!(norm, 3.7416573867739413);
        }
    }

    #[test]
    fn test_dot() {
        {
            let v1 = DVector::<f64>::from_slice_vec(&[1.0, 2.0, 3.0], 3, VecType::Row);
            let v2 = DVector::<f64>::from_slice_vec(&[4.0, 5.0, 6.0], 3, VecType::Row);
            let dot = v1.dot(&v2);
            assert_relative_eq!(dot, 32.0);
        }
        {
            let v1 = DVector::<f64>::from_slice_vec(&[1.0, 2.0, 3.0], 3, VecType::Col);
            let v2 = DVector::<f64>::from_slice_vec(&[4.0, 5.0, 6.0], 3, VecType::Col);
            let dot = v1.dot(&v2);
            assert_relative_eq!(dot, 32.0);
        }
    }

    #[test]
    fn test_normalize() {
        {
            let v = DVector::<f64>::from_slice_vec(&[1.0, 2.0, 3.0], 3, VecType::Row);
            let norm = v.norm();
            let normalized = v.normalize();
            assert_relative_eq!(normalized[0], 1.0 / norm);
            assert_relative_eq!(normalized[1], 2.0 / norm);
            assert_relative_eq!(normalized[2], 3.0 / norm);
        }
        {
            let v = DVector::<f64>::from_slice_vec(&[1.0, 2.0, 3.0], 3, VecType::Col);
            let norm = v.norm();
            let normalized = v.normalize();
            assert_relative_eq!(normalized[0], 1.0 / norm);
            assert_relative_eq!(normalized[1], 2.0 / norm);
            assert_relative_eq!(normalized[2], 3.0 / norm);
        }
    }

    #[test]
    fn test_cross() {
        {
            let v1 = DVector::<f64>::from_slice_vec(&[1.0, 2.0, 3.0], 3, VecType::Row);
            let v2 = DVector::<f64>::from_slice_vec(&[4.0, 5.0, 6.0], 3, VecType::Row);
            let cross = v1.cross(&v2);
            assert_relative_eq!(cross[0], -3.0);
            assert_relative_eq!(cross[1], 6.0);
            assert_relative_eq!(cross[2], -3.0);
        }
        {
            let v1 = DVector::<f64>::from_slice_vec(&[1.0, 2.0, 3.0], 3, VecType::Col);
            let v2 = DVector::<f64>::from_slice_vec(&[4.0, 5.0, 6.0], 3, VecType::Col);
            let cross = v1.cross(&v2);
            assert_relative_eq!(cross[0], -3.0);
            assert_relative_eq!(cross[1], 6.0);
            assert_relative_eq!(cross[2], -3.0);
        }
    }

    #[test]
    fn test_angle() {
        {
            let v1 = DVector::<f64>::from_slice_vec(&[1.0, 0.0, 0.0], 3, VecType::Row);
            let v2 = DVector::<f64>::from_slice_vec(&[0.0, 1.0, 0.0], 3, VecType::Row);
            let angle1 = v1.angle(&v2);
            assert_relative_eq!(angle1, std::f64::consts::FRAC_PI_2);
            let angle2 = v2.angle(&v1);
            assert_relative_eq!(angle2, std::f64::consts::FRAC_PI_2);
        }
    }

    #[test]
    fn test_outer_materializes_expected_matrix() {
        let v = DVector::<f64>::from_slice_vec(&[1.0, 2.0], 2, VecType::Col);
        let w = DVector::<f64>::from_slice_vec(&[10.0, 20.0, 30.0], 3, VecType::Col);

        let actual = DMatrix::<f64>::from(v.outer(&w));
        let expected = DMatrix::<f64>::from_row_slice(&[10.0, 20.0, 30.0, 20.0, 40.0, 60.0], 2, 3);

        assert_matrix_relative_eq(&actual, &expected);
    }

    #[test]
    fn test_outer_ignores_dynamic_vector_orientation() {
        for left_type in [VecType::Row, VecType::Col] {
            for right_type in [VecType::Row, VecType::Col] {
                let v = DVector::<f64>::from_slice_vec(&[1.0, 2.0], 2, left_type);
                let w = DVector::<f64>::from_slice_vec(&[10.0, 20.0, 30.0], 3, right_type);

                let actual = DMatrix::<f64>::from(v.outer(&w));
                let expected =
                    DMatrix::<f64>::from_row_slice(&[10.0, 20.0, 30.0, 20.0, 40.0, 60.0], 2, 3);

                assert_matrix_relative_eq(&actual, &expected);
            }
        }
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn test_outer_participates_in_fused_add_assign_expression() {
        let mut a = DMatrix::<f64>::zeros(3, 3);
        let b =
            DMatrix::<f64>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], 3, 3);
        let v = DVector::<f64>::from_slice_vec(&[1.0, 2.0, 3.0], 3, VecType::Col);
        let w = DVector::<f64>::from_slice_vec(&[10.0, 20.0, 30.0], 3, VecType::Col);

        a += &b + v.outer(&w);

        let expected = DMatrix::<f64>::from_row_slice(
            &[11.0, 22.0, 33.0, 24.0, 45.0, 66.0, 37.0, 68.0, 99.0],
            3,
            3,
        );
        assert_matrix_relative_eq(&a, &expected);
    }

    #[test]
    fn test_outer_add_assigns_directly() {
        let mut a = DMatrix::<f64>::from_value(1.0, 2, 3);
        let v = DVector::<f64>::from_slice_vec(&[1.0, 2.0], 2, VecType::Col);
        let w = DVector::<f64>::from_slice_vec(&[10.0, 20.0, 30.0], 3, VecType::Col);

        a += v.outer(&w);

        let expected = DMatrix::<f64>::from_row_slice(&[11.0, 21.0, 31.0, 21.0, 41.0, 61.0], 2, 3);
        assert_matrix_relative_eq(&a, &expected);
    }

    #[test]
    fn test_outer_assigns_directly_with_remaining_arithmetic() {
        let v = DVector::<f64>::from_slice_vec(&[1.0, 2.0], 2, VecType::Col);
        let w = DVector::<f64>::from_slice_vec(&[10.0, 20.0, 30.0], 3, VecType::Col);

        let mut sub = DMatrix::<f64>::from_value(100.0, 2, 3);
        sub -= v.outer(&w);
        let sub_expected =
            DMatrix::<f64>::from_row_slice(&[90.0, 80.0, 70.0, 80.0, 60.0, 40.0], 2, 3);
        assert_matrix_relative_eq(&sub, &sub_expected);

        let mut mul = DMatrix::<f64>::from_value(1.0, 2, 3);
        mul *= v.outer(&w);
        let mul_expected =
            DMatrix::<f64>::from_row_slice(&[10.0, 20.0, 30.0, 20.0, 40.0, 60.0], 2, 3);
        assert_matrix_relative_eq(&mul, &mul_expected);

        let mut div =
            DMatrix::<f64>::from_row_slice(&[100.0, 120.0, 150.0, 200.0, 240.0, 300.0], 2, 3);
        div /= v.outer(&w);
        let div_expected = DMatrix::<f64>::from_row_slice(&[10.0, 6.0, 5.0, 10.0, 6.0, 5.0], 2, 3);
        assert_matrix_relative_eq(&div, &div_expected);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn test_outer_expression_assigns_with_remaining_arithmetic() {
        let b = DMatrix::<f64>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3);
        let v = DVector::<f64>::from_slice_vec(&[1.0, 2.0], 2, VecType::Col);
        let w = DVector::<f64>::from_slice_vec(&[10.0, 20.0, 30.0], 3, VecType::Col);
        let expr_expected =
            DMatrix::<f64>::from_row_slice(&[11.0, 22.0, 33.0, 24.0, 45.0, 66.0], 2, 3);

        let mut sub = DMatrix::<f64>::from_value(100.0, 2, 3);
        sub -= &b + v.outer(&w);
        let sub_expected =
            DMatrix::<f64>::from_row_slice(&[89.0, 78.0, 67.0, 76.0, 55.0, 34.0], 2, 3);
        assert_matrix_relative_eq(&sub, &sub_expected);

        let mut mul = DMatrix::<f64>::from_value(1.0, 2, 3);
        mul *= &b + v.outer(&w);
        assert_matrix_relative_eq(&mul, &expr_expected);

        let mut div = DMatrix::<f64>::from_row_slice(&[22.0, 44.0, 66.0, 48.0, 90.0, 132.0], 2, 3);
        div /= &b + v.outer(&w);
        let div_expected = DMatrix::<f64>::from_value(2.0, 2, 3);
        assert_matrix_relative_eq(&div, &div_expected);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn test_outer_chains_with_lazy_expressions() {
        let b = DMatrix::<f64>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3);
        let v = DVector::<f64>::from_slice_vec(&[1.0, 2.0], 2, VecType::Col);
        let w = DVector::<f64>::from_slice_vec(&[10.0, 20.0, 30.0], 3, VecType::Col);

        let actual: DMatrix<f64> = ((&b + v.outer(&w)) * 2.0).into();

        let expected = DMatrix::<f64>::from_row_slice(&[22.0, 44.0, 66.0, 48.0, 90.0, 132.0], 2, 3);
        assert_matrix_relative_eq(&actual, &expected);
    }

    #[test]
    #[should_panic(expected = "Outer product operands must be vectors")]
    fn test_outer_panics_for_non_vector_operand() {
        let matrix = DMatrix::<f64>::zeros(2, 2);
        let v = DVector::<f64>::from_slice_vec(&[1.0, 2.0], 2, VecType::Col);

        let _ = matrix.outer(&v);
    }

    #[test]
    #[should_panic(expected = "DMatrix::add_assign dimension mismatch")]
    fn test_outer_add_assign_dimension_mismatch_panics() {
        let mut a = DMatrix::<f64>::zeros(2, 2);
        let v = DVector::<f64>::from_slice_vec(&[1.0, 2.0, 3.0], 3, VecType::Col);
        let w = DVector::<f64>::from_slice_vec(&[10.0, 20.0, 30.0], 3, VecType::Col);

        a += v.outer(&w);
    }
}
