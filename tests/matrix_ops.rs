#![feature(generic_const_exprs)]
#![feature(impl_trait_in_assoc_type)]

mod smatrix_tests
{

    use approx::assert_relative_eq;
    use topohedral_linalg::{MatrixOps, Complex};
    use topohedral_linalg::smatrix::*;

    //{{{ collection: eig tests
    #[test]
    fn test_eig_simple()
    {
        let a =
            SMatrix::<f64, 3, 3>::from_row_slice(&[1.0, 5.0, 0.0, 2.0, 4.0, -1.0, 0.0, 2.0, 3.0]);

        let eig = a.eig().unwrap();
        // Known eigenvalues for this matrix
        let expected_eigenvalues = vec![
            Complex::<f64>::new(-0.8595233886152194, 0.0),
            Complex::<f64>::new(5.433664629783286, 0.0),
            Complex::<f64>::new(3.42585875883193, 0.0),
        ];

        for i in 0..3
        {
            assert_relative_eq!(
                eig.eigvals[i].re,
                expected_eigenvalues[i].re,
                epsilon = 1e-10
            );
            assert_relative_eq!(
                eig.eigvals[i].im,
                expected_eigenvalues[i].im,
                epsilon = 1e-10
            );
        }

        // Known left eigenvectors for this matrix
        let expected_left_eigenvecotors = SMatrix::<f64, 3, 3>::from_row_slice(&[
            -0.7212203345550064,
            -0.3850687990747861,
            -0.3073880480179293,
            0.6705630402249634,
            -0.8536329572455033,
            -0.3728399943222721,
            0.1737424476304467,
            0.3507603088768719,
            0.8755015285934659,
        ]);

        for i in 0..3
        {
            for j in 0..3
            {
                assert_relative_eq!(
                    eig.left_eigvecs[(i, j)],
                    expected_left_eigenvecotors[(i, j)],
                    epsilon = 1e-10
                );
            }
        }
    }
    //}}}
    //{{{ collection: lu tests
    #[test]
    fn test_lu_non_diagonal_dominant()
    {
        let a = SMatrix::<f64, 3, 3>::from_row_slice(&[
            1.0, 2000.0, 3000.0, 5000.0, 10.0, -8900.0, -10000.0, 9008.0, 0.0,
        ]);

        let lu_ret = a.lu().unwrap();

        let exp_p =
            SMatrix::<f64, 3, 3>::from_row_slice(&[0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0]);

        let exp_l = SMatrix::<f64, 3, 3>::from_row_slice(&[
            1.00000000e+00,
            0.00000000e+00,
            0.00000000e+00,
            -5.00000000e-01,
            1.00000000e+00,
            0.00000000e+00,
            -1.00000000e-04,
            4.43265574e-01,
            1.00000000e+00,
        ]);

        let exp_u = SMatrix::<f64, 3, 3>::from_row_slice(&[
            -10000.0,
            9008.0,
            0.0,
            0.0,
            4514.0,
            -8900.0,
            0.0,
            0.0,
            6945.06360656,
        ]);

        for i in 0..9
        {
            assert_relative_eq!(lu_ret.p[i], exp_p[i], max_relative = 1.0e-8);
            assert_relative_eq!(lu_ret.l[i], exp_l[i], max_relative = 1.0e-8);
            assert_relative_eq!(lu_ret.u[i], exp_u[i], max_relative = 1.0e-8);
        }
    }

    #[test]
    fn test_lu_diagonal_dominant()
    {
        let a = SMatrix::<f64, 4, 4>::from_row_slice(&[
            100000.0, 10.0, 56.0, 10.0, -69.0, 1.56e6, 3.0, -9.0, 0.0, 0.0, -5.6e-5, -700.0, 890.0,
            0.0, -7899.0, 8.0e5,
        ]);

        let lu_ret = a.lu().unwrap();

        let exp_p = [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 0., 1., 0., 0., 1., 0.,
        ];

        let exp_l = SMatrix::<f64, 4, 4>::from_row_slice(&[
            1.00000000e+00,
            0.00000000e+00,
            0.00000000e+00,
            0.00000000e+00,
            -6.90000000e-04,
            1.00000000e+00,
            0.00000000e+00,
            0.00000000e+00,
            8.90000000e-03,
            -5.70512818e-08,
            1.00000000e+00,
            0.00000000e+00,
            0.00000000e+00,
            0.00000000e+00,
            7.08905771e-09,
            1.00000000e+00,
        ]);

        let exp_u = SMatrix::<f64, 4, 4>::from_row_slice(&[
            1.00000000e+05,
            1.00000000e+01,
            5.60000000e+01,
            1.00000000e+01,
            0.00000000e+00,
            1.56000001e+06,
            3.03864000e+00,
            -8.99310000e+00,
            0.00000000e+00,
            0.00000000e+00,
            -7.89949840e+03,
            7.99999911e+05,
            0.00000000e+00,
            0.00000000e+00,
            0.00000000e+00,
            -7.00005671e+02,
        ]);

        for i in 0..16
        {
            assert_relative_eq!(lu_ret.p[i], exp_p[i], max_relative = 1.0e-8);
            assert_relative_eq!(lu_ret.l[i], exp_l[i], max_relative = 1.0e-8);
            assert_relative_eq!(lu_ret.u[i], exp_u[i], max_relative = 1.0e-8);
        }
    }
    //}}}
    //{{{ collection: matmul tests
    #[test]
    fn test_matmul_f64_general()
    {
        let a = SMatrix::<f64, 2, 3>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

        let b = SMatrix::<f64, 3, 2>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

        let expected = SMatrix::<f64, 2, 2>::from_row_slice(&[22.0, 28.0, 49.0, 64.0]);

        let result = (&a).matmul(&b);

        for (res_val, exp_val) in result.iter().zip(expected.iter())
        {
            assert_relative_eq!(res_val, exp_val, epsilon = 1e-10);
        }
    }

    #[test]

    fn test_matmul_f64_col_vector()
    {
        let a = SMatrix::<f64, 2, 3>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

        let b = SMatrix::<f64, 3, 1>::from_row_slice(&[1.0, 2.0, 3.0]);

        let expected = SMatrix::<f64, 2, 1>::from_row_slice(&[14.0, 32.0]);

        let result = (&a).matmul(&b);

        for (res_val, exp_val) in result.iter().zip(expected.iter())
        {
            assert_relative_eq!(res_val, exp_val, epsilon = 1e-10);
        }
    }

    #[test]

    fn test_matmul_f32_general()
    {
        let a = SMatrix::<f32, 2, 3>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

        let b = SMatrix::<f32, 3, 2>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

        let expected = SMatrix::<f32, 2, 2>::from_row_slice(&[22.0, 28.0, 49.0, 64.0]);

        let result = (&a).matmul(&b);

        for (res_val, exp_val) in result.iter().zip(expected.iter())
        {
            assert_relative_eq!(res_val, exp_val, epsilon = 1e-10);
        }
    }

    #[test]

    fn test_matmul_f32_col_vector()
    {
        let a = SMatrix::<f32, 2, 3>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

        let b = SMatrix::<f32, 3, 1>::from_row_slice(&[1.0, 2.0, 3.0]);

        let expected = SMatrix::<f32, 2, 1>::from_row_slice(&[14.0, 32.0]);

        let result = (&a).matmul(&b);

        for (res_val, exp_val) in result.iter().zip(expected.iter())
        {
            assert_relative_eq!(res_val, exp_val, epsilon = 1e-10);
        }
    }

    #[test]

    fn test_matmul_i32()
    {
        let a = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 2, 3, 4]);

        let b = SMatrix::<i32, 2, 2>::from_row_slice(&[5, 6, 7, 8]);

        let expected = SMatrix::<i32, 2, 2>::from_row_slice(&[19, 22, 43, 50]);

        let result = (&a).matmul(&b);

        for (res_val, exp_val) in result.iter().zip(expected.iter())
        {
            assert_eq!(res_val, exp_val);
        }
    }

    #[test]

    fn test_matmul_u64()
    {
        let a = SMatrix::<i64, 2, 2>::from_row_slice(&[1, 2, 3, 4]);

        let b = SMatrix::<i64, 2, 1>::from_row_slice(&[5, 6]);

        let expected = SMatrix::<i64, 2, 1>::from_row_slice(&[17, 39]);

        let result = (&a).matmul(&b);

        for (res_val, exp_val) in result.iter().zip(expected.iter())
        {
            assert_eq!(res_val, exp_val);
        }
    }

    #[test]

    fn test_matmul_f64_row_vector()
    {
        let a = SMatrix::<f64, 1, 3>::from_row_slice(&[1.0, 2.0, 3.0]);

        let b = SMatrix::<f64, 3, 2>::from_row_slice(&[4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

        let expected = SMatrix::<f64, 1, 2>::from_row_slice(&[40.0, 46.0]);

        let result = (&a).matmul(&b);

        for (res_val, exp_val) in result.iter().zip(expected.iter())
        {
            assert_eq!(res_val, exp_val);
        }
    }

    #[test]

    fn test_matmul_f32_row_vector()
    {
        let a = SMatrix::<f32, 1, 2>::from_row_slice(&[1.0, 2.0]);

        let b = SMatrix::<f32, 2, 3>::from_row_slice(&[3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);

        let expected = SMatrix::<f32, 1, 3>::from_row_slice(&[15.0, 18.0, 21.0]);

        let result = (&a).matmul(&b);

        for (res_val, exp_val) in result.iter().zip(expected.iter())
        {
            assert_relative_eq!(res_val, exp_val, epsilon = 1e-10);
        }
    }

    #[test]

    fn test_matmul_i32_row_vector()
    {
        let a = SMatrix::<i32, 1, 2>::from_row_slice(&[2, 3]);

        let b = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 2, 3, 4]);

        let expected = SMatrix::<i32, 1, 2>::from_row_slice(&[11, 16]);

        let result = (&a).matmul(&b);

        for (res_val, exp_val) in result.iter().zip(expected.iter())
        {
            assert_eq!(res_val, exp_val);
        }
    }

    #[test]

    fn test_matmul_u64_row_vector()
    {
        let a = SMatrix::<i64, 1, 3>::from_row_slice(&[1, 2, 3]);

        let b = SMatrix::<i64, 3, 1>::from_row_slice(&[4, 5, 6]);

        let expected = SMatrix::<i64, 1, 1>::from_row_slice(&[32]);

        let result = (&a).matmul(&b);

        for (res_val, exp_val) in result.iter().zip(expected.iter())
        {
            assert_eq!(res_val, exp_val);
        }
    }
    //}}}
    //{{{ collectoin: qr
    #[test]
    fn test_qr_decomposition()
    {
        let a = SMatrix::<f64, 3, 3>::from_row_slice(&[
            12.0, -51.0, 4.0, 6.0, 167.0, -68.0, -4.0, 24.0, -41.0,
        ]);

        let qr::Return { q, r } = a.qr().unwrap();

        // Verify Q*R = A
        let reconstructed: SMatrix<f64, 3, 3> = q.matmul(&r);

        for i in 0..9
        {
            assert_relative_eq!(reconstructed[i], a[i], max_relative = 1.0e-8);
        }

        // Verify Q is orthogonal (Q^T * Q = I)
        let q_transpose = q.transpose();
        let identity: SMatrix<f64, 3, 3> = q.matmul(&q_transpose);

        for i in 0..3
        {
            for j in 0..3
            {
                if i == j
                {
                    assert_relative_eq!(identity[i + j * 3], 1.0, max_relative = 1.0e-8);
                }
                else
                {
                    assert_relative_eq!(identity[i + j * 3], 0.0, max_relative = 1.0e-8);
                }
            }
        }
    }
    //}}}
    //{{{ collection: schur
    #[test]
    fn test_schur_decomposition()
    {
        let a =
            SMatrix::<f64, 3, 3>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

        let schur::Return { q, t } = a.schur().unwrap();

        // Verify Q*T*Q^T = A
        let q_t = q.matmul(&t);
        let q_transpose = q.transpose();
        let reconstructed = q_t.matmul(&q_transpose);

        for i in 0..9
        {
            assert_relative_eq!(reconstructed[i], a[i], max_relative = 1.0e-8);
        }

        // Verify Q is orthogonal (Q^T * Q = I)
        let identity = q.matmul(&q_transpose);

        for i in 0..3
        {
            for j in 0..3
            {
                if i == j
                {
                    assert_relative_eq!(
                        identity[i + j * 3],
                        1.0,
                        max_relative = 1.0e-8,
                        epsilon = 1.0e-10
                    );
                }
                else
                {
                    assert_relative_eq!(
                        identity[i + j * 3],
                        0.0,
                        max_relative = 1.0e-8,
                        epsilon = 1.0e-10
                    );
                }
            }
        }
    }
    //}}}
    //{{{ collection: solve
    #[test]
    fn test_solve() {

        let a = SMatrix::<f64, 3, 3>::from_row_slice(&[
            3.0, -1.0, 2.0,
            1.0, 2.0, 0.0,
            4.0, 0.0, 6.0,
        ]);

        let b = SMatrix::<f64, 3, 3>::from_row_slice(&[
            7.0, -7.0, 2.0,
            1.0, 2.0, 3.0,
            22.0, -10.0, 3.0,
        ]);

        let x = a.solve(&b).unwrap();

        // Verify A * X = B
        let computed_b = a.matmul(&x);
        
        for i in 0..9 {
            assert_relative_eq!(computed_b[i], b[i], max_relative=1.0e-8);
        }
    }
    //}}}
}


mod dmatrix_tests
{

    use approx::assert_relative_eq;
    use topohedral_linalg::{MatrixOps, Complex};
    use topohedral_linalg::dmatrix::*;

    //{{{ collection: eig tests
    #[test]
    fn test_eig_simple()
    {
        let a =
            DMatrix::<f64>::from_row_slice(&[1.0, 5.0, 0.0, 2.0, 4.0, -1.0, 0.0, 2.0, 3.0], 3, 3);

        let eig = a.eig().unwrap();
        // Known eigenvalues for this matrix
        let expected_eigenvalues = vec![
            Complex::<f64>::new(-0.8595233886152194, 0.0),
            Complex::<f64>::new(5.433664629783286, 0.0),
            Complex::<f64>::new(3.42585875883193, 0.0),
        ];

        for i in 0..3
        {
            assert_relative_eq!(
                eig.eigvals[i].re,
                expected_eigenvalues[i].re,
                epsilon = 1e-10
            );
            assert_relative_eq!(
                eig.eigvals[i].im,
                expected_eigenvalues[i].im,
                epsilon = 1e-10
            );
        }

        // Known left eigenvectors for this matrix
        let expected_left_eigenvecotors = DMatrix::<f64>::from_row_slice(&[
            -0.7212203345550064,
            -0.3850687990747861,
            -0.3073880480179293,
            0.6705630402249634,
            -0.8536329572455033,
            -0.3728399943222721,
            0.1737424476304467,
            0.3507603088768719,
            0.8755015285934659,
        ], 3, 3);

        for i in 0..3
        {
            for j in 0..3
            {
                assert_relative_eq!(
                    eig.left_eigvecs[(i, j)],
                    expected_left_eigenvecotors[(i, j)],
                    epsilon = 1e-10
                );
            }
        }
    }
    //}}}
    //{{{ collection: lu tests
    #[test]
    fn test_lu_non_diagonal_dominant()
    {
        let a = DMatrix::<f64>::from_row_slice(&[
            1.0, 2000.0, 3000.0, 5000.0, 10.0, -8900.0, -10000.0, 9008.0, 0.0,
        ], 3, 3);

        let lu_ret = a.lu().unwrap();

        let exp_p =
            DMatrix::<f64>::from_row_slice(&[0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0], 3, 3);

        let exp_l = DMatrix::<f64>::from_row_slice(&[
            1.00000000e+00,
            0.00000000e+00,
            0.00000000e+00,
            -5.00000000e-01,
            1.00000000e+00,
            0.00000000e+00,
            -1.00000000e-04,
            4.43265574e-01,
            1.00000000e+00,
        ], 3, 3);

        let exp_u = DMatrix::<f64>::from_row_slice(&[
            -10000.0,
            9008.0,
            0.0,
            0.0,
            4514.0,
            -8900.0,
            0.0,
            0.0,
            6945.06360656,
        ], 3, 3);

        for i in 0..9
        {
            assert_relative_eq!(lu_ret.p[i], exp_p[i], max_relative = 1.0e-8);
            assert_relative_eq!(lu_ret.l[i], exp_l[i], max_relative = 1.0e-8);
            assert_relative_eq!(lu_ret.u[i], exp_u[i], max_relative = 1.0e-8);
        }
    }

    #[test]
    fn test_lu_diagonal_dominant()
    {
        let a = DMatrix::<f64>::from_row_slice(&[
            100000.0, 10.0, 56.0, 10.0, -69.0, 1.56e6, 3.0, -9.0, 0.0, 0.0, -5.6e-5, -700.0, 890.0,
            0.0, -7899.0, 8.0e5,
        ], 4, 4);

        let lu_ret = a.lu().unwrap();

        let exp_p = [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 0., 1., 0., 0., 1., 0.,
        ];

        let exp_l = DMatrix::<f64>::from_row_slice(&[
            1.00000000e+00,
            0.00000000e+00,
            0.00000000e+00,
            0.00000000e+00,
            -6.90000000e-04,
            1.00000000e+00,
            0.00000000e+00,
            0.00000000e+00,
            8.90000000e-03,
            -5.70512818e-08,
            1.00000000e+00,
            0.00000000e+00,
            0.00000000e+00,
            0.00000000e+00,
            7.08905771e-09,
            1.00000000e+00,
        ], 4, 4);

        let exp_u = DMatrix::<f64>::from_row_slice(&[
            1.00000000e+05,
            1.00000000e+01,
            5.60000000e+01,
            1.00000000e+01,
            0.00000000e+00,
            1.56000001e+06,
            3.03864000e+00,
            -8.99310000e+00,
            0.00000000e+00,
            0.00000000e+00,
            -7.89949840e+03,
            7.99999911e+05,
            0.00000000e+00,
            0.00000000e+00,
            0.00000000e+00,
            -7.00005671e+02,
        ], 4, 4);

        for i in 0..16
        {
            assert_relative_eq!(lu_ret.p[i], exp_p[i], max_relative = 1.0e-8);
            assert_relative_eq!(lu_ret.l[i], exp_l[i], max_relative = 1.0e-8);
            assert_relative_eq!(lu_ret.u[i], exp_u[i], max_relative = 1.0e-8);
        }
    }
    //}}}
    //{{{ collection: matmul tests
    #[test]
    fn test_matmul_f64_general()
    {
        let a = DMatrix::<f64>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3);

        let b = DMatrix::<f64>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 3, 2);

        let expected = DMatrix::<f64>::from_row_slice(&[22.0, 28.0, 49.0, 64.0], 2, 2);

        let result = (&a).matmul(&b);

        for (res_val, exp_val) in result.iter().zip(expected.iter())
        {
            assert_relative_eq!(res_val, exp_val, epsilon = 1e-10);
        }
    }

    #[test]

    fn test_matmul_f64_col_vector()
    {
        let a = DMatrix::<f64>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3);

        let b = DMatrix::<f64>::from_row_slice(&[1.0, 2.0, 3.0], 3, 1);

        let expected = DMatrix::<f64>::from_row_slice(&[14.0, 32.0], 2, 1);

        let result = (&a).matmul(&b);

        for (res_val, exp_val) in result.iter().zip(expected.iter())
        {
            assert_relative_eq!(res_val, exp_val, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_matmul_f32_general()
    {
        let a = DMatrix::<f32>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3);

        let b = DMatrix::<f32>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 3, 2);

        let expected = DMatrix::<f32>::from_row_slice(&[22.0, 28.0, 49.0, 64.0], 2, 2);

        let result = (&a).matmul(&b);

        for (res_val, exp_val) in result.iter().zip(expected.iter())
        {
            assert_relative_eq!(res_val, exp_val, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_matmul_f32_col_vector()
    {
        let a = DMatrix::<f32>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3);

        let b = DMatrix::<f32>::from_row_slice(&[1.0, 2.0, 3.0], 3, 1);

        let expected = DMatrix::<f32>::from_row_slice(&[14.0, 32.0], 2, 1);

        let result = (&a).matmul(&b);

        for (res_val, exp_val) in result.iter().zip(expected.iter())
        {
            assert_relative_eq!(res_val, exp_val, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_matmul_i32()
    {
        let a = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4], 2, 2);

        let b = DMatrix::<i32>::from_row_slice(&[5, 6, 7, 8], 2, 2);

        let expected = DMatrix::<i32>::from_row_slice(&[19, 22, 43, 50], 2, 2);

        let result = (&a).matmul(&b);

        for (res_val, exp_val) in result.iter().zip(expected.iter())
        {
            assert_eq!(res_val, exp_val);
        }
    }

    #[test]
    fn test_matmul_u64()
    {
        let a = DMatrix::<i64>::from_row_slice(&[1, 2, 3, 4], 2, 2);

        let b = DMatrix::<i64>::from_row_slice(&[5, 6], 2, 1);

        let expected = DMatrix::<i64>::from_row_slice(&[17, 39], 2, 1);

        let result = (&a).matmul(&b);

        for (res_val, exp_val) in result.iter().zip(expected.iter())
        {
            assert_eq!(res_val, exp_val);
        }
    }

    #[test]
    fn test_matmul_f64_row_vector()
    {
        let a = DMatrix::<f64>::from_row_slice(&[1.0, 2.0, 3.0], 1, 3);

        let b = DMatrix::<f64>::from_row_slice(&[4.0, 5.0, 6.0, 7.0, 8.0, 9.0], 3, 2);

        let expected = DMatrix::<f64>::from_row_slice(&[40.0, 46.0], 1, 2);

        let result = (&a).matmul(&b);

        for (res_val, exp_val) in result.iter().zip(expected.iter())
        {
            assert_eq!(res_val, exp_val);
        }
    }

    #[test]
    fn test_matmul_f32_row_vector()
    {
        let a = DMatrix::<f32>::from_row_slice(&[1.0, 2.0], 1, 2);

        let b = DMatrix::<f32>::from_row_slice(&[3.0, 4.0, 5.0, 6.0, 7.0, 8.0], 2, 3);

        let expected = DMatrix::<f32>::from_row_slice(&[15.0, 18.0, 21.0], 1, 3);

        let result = (&a).matmul(&b);

        for (res_val, exp_val) in result.iter().zip(expected.iter())
        {
            assert_relative_eq!(res_val, exp_val, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_matmul_i32_row_vector()
    {
        let a = DMatrix::<i32>::from_row_slice(&[2, 3], 1, 2);

        let b = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4], 2, 2);

        let expected = DMatrix::<i32>::from_row_slice(&[11, 16], 1, 2);

        let result = (&a).matmul(&b);

        for (res_val, exp_val) in result.iter().zip(expected.iter())
        {
            assert_eq!(res_val, exp_val);
        }
    }

    #[test]
    fn test_matmul_u64_row_vector()
    {
        let a = DMatrix::<i64>::from_row_slice(&[1, 2, 3], 1, 3);

        let b = DMatrix::<i64>::from_row_slice(&[4, 5, 6], 3, 1);

        let expected = DMatrix::<i64>::from_row_slice(&[32], 1, 1);

        let result = (&a).matmul(&b);

        for (res_val, exp_val) in result.iter().zip(expected.iter())
        {
            assert_eq!(res_val, exp_val);
        }
    }
    //}}}
    // //{{{ collectoin: qr
    // #[test]
    // fn test_qr_decomposition()
    // {
    //     let a = SMatrix::<f64, 3, 3>::from_row_slice(&[
    //         12.0, -51.0, 4.0, 6.0, 167.0, -68.0, -4.0, 24.0, -41.0,
    //     ]);

    //     let qr::Return { q, r } = a.qr().unwrap();

    //     // Verify Q*R = A
    //     let reconstructed: SMatrix<f64, 3, 3> = q.matmul(&r);

    //     for i in 0..9
    //     {
    //         assert_relative_eq!(reconstructed[i], a[i], max_relative = 1.0e-8);
    //     }

    //     // Verify Q is orthogonal (Q^T * Q = I)
    //     let q_transpose = q.transpose();
    //     let identity: SMatrix<f64, 3, 3> = q.matmul(&q_transpose);

    //     for i in 0..3
    //     {
    //         for j in 0..3
    //         {
    //             if i == j
    //             {
    //                 assert_relative_eq!(identity[i + j * 3], 1.0, max_relative = 1.0e-8);
    //             }
    //             else
    //             {
    //                 assert_relative_eq!(identity[i + j * 3], 0.0, max_relative = 1.0e-8);
    //             }
    //         }
    //     }
    // }
    // //}}}
    // //{{{ collection: schur
    // #[test]
    // fn test_schur_decomposition()
    // {
    //     let a =
    //         SMatrix::<f64, 3, 3>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    //     let schur::Return { q, t } = a.schur().unwrap();

    //     // Verify Q*T*Q^T = A
    //     let q_t = q.matmul(&t);
    //     let q_transpose = q.transpose();
    //     let reconstructed = q_t.matmul(&q_transpose);

    //     for i in 0..9
    //     {
    //         assert_relative_eq!(reconstructed[i], a[i], max_relative = 1.0e-8);
    //     }

    //     // Verify Q is orthogonal (Q^T * Q = I)
    //     let identity = q.matmul(&q_transpose);

    //     for i in 0..3
    //     {
    //         for j in 0..3
    //         {
    //             if i == j
    //             {
    //                 assert_relative_eq!(
    //                     identity[i + j * 3],
    //                     1.0,
    //                     max_relative = 1.0e-8,
    //                     epsilon = 1.0e-10
    //                 );
    //             }
    //             else
    //             {
    //                 assert_relative_eq!(
    //                     identity[i + j * 3],
    //                     0.0,
    //                     max_relative = 1.0e-8,
    //                     epsilon = 1.0e-10
    //                 );
    //             }
    //         }
    //     }
    // }
    // //}}}
    // //{{{ collection: solve
    // #[test]
    // fn test_solve() {

    //     let a = SMatrix::<f64, 3, 3>::from_row_slice(&[
    //         3.0, -1.0, 2.0,
    //         1.0, 2.0, 0.0,
    //         4.0, 0.0, 6.0,
    //     ]);

    //     let b = SMatrix::<f64, 3, 3>::from_row_slice(&[
    //         7.0, -7.0, 2.0,
    //         1.0, 2.0, 3.0,
    //         22.0, -10.0, 3.0,
    //     ]);

    //     let x = a.solve(&b).unwrap();

    //     // Verify A * X = B
    //     let computed_b = a.matmul(&x);
        
    //     for i in 0..9 {
    //         assert_relative_eq!(computed_b[i], b[i], max_relative=1.0e-8);
    //     }
    // }
    // //}}}

}