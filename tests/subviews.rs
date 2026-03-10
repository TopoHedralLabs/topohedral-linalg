#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![feature(impl_trait_in_assoc_type)]

//{{{ mod: dmatrix_tests
mod dmatrix_tests
{

    use topohedral_linalg::dmatrix::DMatrix;

    #[test]
    fn test_submatrix()
    {
        let m = DMatrix::<i32>::from_col_slice(
            &[
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25,
            ],
            5,
            5,
        );

        let subm = m.subview(1, 3, 1, 3);
        // test via indexing
        for i in 0..2
        {
            for j in 0..2
            {
                assert_eq!(subm[(i, j)], m[(i + 1, j + 1)]);
            }
        }
        // test via iter
        let mut iter = subm.iter();
        for i in 0..3
        {
            for j in 0..3
            {
                assert_eq!(iter.next(), Some(&m[(j + 1, i + 1)]));
            }
        }
    }

    #[test]
    fn test_row()
    {
        let m = DMatrix::<i32>::from_col_slice(
            &[
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25,
            ],
            5,
            5,
        );

        let subm = m.row(2);
        let expected = [3, 8, 13, 18, 23];

        for (val, exp) in subm.iter().zip(expected.iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_rows()
    {
        let m = DMatrix::<i32>::from_col_slice(
            &[
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25,
            ],
            5,
            5,
        );

        let subm = m.rows(1, 2);
        let expected = [2, 3, 7, 8, 12, 13, 17, 18, 22, 23];

        for (val, exp) in subm.iter().zip(expected.iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_col()
    {
        let m = DMatrix::<i32>::from_col_slice(
            &[
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25,
            ],
            5,
            5,
        );

        let subm = m.col(4);
        let expected = [21, 22, 23, 24, 25];

        for (val, exp) in subm.iter().zip(expected.iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_cols()
    {
        let m = DMatrix::<i32>::from_col_slice(
            &[
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25,
            ],
            5,
            5,
        );

        let subm = m.cols(1, 3);
        let expected = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

        for (val, exp) in subm.iter().zip(expected.iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_submatrix_mut()
    {
        let mut m = DMatrix::<i32>::from_col_slice(
            &[
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25,
            ],
            5,
            5,
        );
        // test read
        let expected = m.subview(1, 3, 1, 3).to_dmatrix();
        let subm = m.subview_mut(1, 3, 1, 3);
        for (val, exp) in subm.iter().zip(expected.iter())
        {
            assert_eq!(*val, *exp);
        }
        // test write
        let mut subm = m.subview_mut(1, 3, 1, 3);
        for val in subm.iter_mut()
        {
            *val = 1;
        }
        let subm = m.subview(1, 3, 1, 3);
        for val in subm.iter()
        {
            assert_eq!(*val, 1);
        }
    }

    #[test]
    fn test_row_mut()
    {
        let mut m = DMatrix::<i32>::from_col_slice(
            &[
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25,
            ],
            5,
            5,
        );
        // test read
        {
            let subm = m.row_mut(2);
            let expected = [3, 8, 13, 18, 23];

            for (val, exp) in subm.iter().zip(expected.iter())
            {
                assert_eq!(*val, *exp);
            }
        }
        // test write
        {
            let mut subm = m.row_mut(2);
            for val in subm.iter_mut()
            {
                *val = 1;
            }
            let subm = m.row(2);
            for val in subm.iter()
            {
                assert_eq!(*val, 1);
            }
        }
    }

    #[test]
    fn test_rows_mut()
    {
        let mut m = DMatrix::<i32>::from_col_slice(
            &[
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25,
            ],
            5,
            5,
        );
        // test read
        {
            let subm = m.rows_mut(1, 2);
            let expected = [2, 3, 7, 8, 12, 13, 17, 18, 22, 23];

            for (val, exp) in subm.iter().zip(expected.iter())
            {
                assert_eq!(*val, *exp);
            }
        }
        // test write
        {
            let mut subm = m.rows_mut(1, 2);
            for val in subm.iter_mut()
            {
                *val = -1;
            }
            let subm = m.rows(1, 2);
            for val in subm.iter()
            {
                assert_eq!(*val, -1);
            }
        }
    }

    #[test]
    fn test_col_mut()
    {
        let mut m = DMatrix::<i32>::from_col_slice(
            &[
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25,
            ],
            5,
            5,
        );
        // test read
        let subm = m.col_mut(4);
        let expected = [21, 22, 23, 24, 25];

        for (val, exp) in subm.iter().zip(expected.iter())
        {
            assert_eq!(*val, *exp);
        }
        // test write
        let mut subm = m.col_mut(4);
        for val in subm.iter_mut()
        {
            *val = -1;
        }
        let subm = m.col(4);
        for val in subm.iter()
        {
            assert_eq!(*val, -1);
        }
    }

    #[test]
    fn test_cols_mut()
    {
        let mut m = DMatrix::<i32>::from_col_slice(
            &[
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25,
            ],
            5,
            5,
        );
        // test read
        let subm = m.cols_mut(1, 3);
        let expected = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

        for (val, exp) in subm.iter().zip(expected.iter())
        {
            assert_eq!(*val, *exp);
        }
        // test write
        let mut subm = m.cols_mut(1, 3);
        for val in subm.iter_mut()
        {
            *val = -1;
        }
        let subm = m.cols(1, 3);
        for val in subm.iter()
        {
            assert_eq!(*val, -1);
        }
    }

    #[test]
    fn test_row_copy_from_borrowed_and_moved()
    {
        let mut m = DMatrix::<i32>::zeros(3, 5);
        let borrowed_rhs = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4, 5], 1, 5);
        m.row_mut(0).copy_from(&borrowed_rhs);

        let moved_rhs = DMatrix::<i32>::from_row_slice(&[6, 7, 8, 9, 10], 1, 5);
        m.row_mut(1).copy_from(moved_rhs);

        let rhs_source = DMatrix::<i32>::from_row_slice(&[11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 2, 5);
        m.row_mut(2).copy_from(rhs_source.row(1));

        for (val, exp) in m.row(0).iter().zip([1, 2, 3, 4, 5].iter())
        {
            assert_eq!(*val, *exp);
        }
        for (val, exp) in m.row(1).iter().zip([6, 7, 8, 9, 10].iter())
        {
            assert_eq!(*val, *exp);
        }
        for (val, exp) in m.row(2).iter().zip([16, 17, 18, 19, 20].iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_col_copy_from_borrowed_and_moved()
    {
        let mut m = DMatrix::<i32>::zeros(5, 3);
        let borrowed_rhs = DMatrix::<i32>::from_col_slice(&[1, 2, 3, 4, 5], 5, 1);
        m.col_mut(0).copy_from(&borrowed_rhs);

        let moved_rhs = DMatrix::<i32>::from_col_slice(&[6, 7, 8, 9, 10], 5, 1);
        m.col_mut(1).copy_from(moved_rhs);

        let rhs_source = DMatrix::<i32>::from_col_slice(&[11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 5, 2);
        m.col_mut(2).copy_from(rhs_source.col(1));

        for (val, exp) in m.col(0).iter().zip([1, 2, 3, 4, 5].iter())
        {
            assert_eq!(*val, *exp);
        }
        for (val, exp) in m.col(1).iter().zip([6, 7, 8, 9, 10].iter())
        {
            assert_eq!(*val, *exp);
        }
        for (val, exp) in m.col(2).iter().zip([16, 17, 18, 19, 20].iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_subview_copy_from_borrowed_and_moved()
    {
        let mut m = DMatrix::<i32>::zeros(4, 4);
        let borrowed_rhs = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4], 2, 2);
        m.subview_mut(0, 1, 0, 1).copy_from(&borrowed_rhs);

        let moved_rhs = DMatrix::<i32>::from_row_slice(&[5, 6, 7, 8], 2, 2);
        m.subview_mut(2, 3, 2, 3).copy_from(moved_rhs);

        let rhs_source = DMatrix::<i32>::from_row_slice(&[10, 11, 12, 13, 14, 15, 16, 17, 18], 3, 3);
        m.subview_mut(0, 1, 2, 3).copy_from(rhs_source.subview(1, 2, 0, 1));

        let expected_a = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4], 2, 2);
        let expected_b = DMatrix::<i32>::from_row_slice(&[5, 6, 7, 8], 2, 2);
        let expected_c = DMatrix::<i32>::from_row_slice(&[13, 14, 16, 17], 2, 2);

        for (val, exp) in m.subview(0, 1, 0, 1).iter().zip(expected_a.iter())
        {
            assert_eq!(*val, *exp);
        }
        for (val, exp) in m.subview(2, 3, 2, 3).iter().zip(expected_b.iter())
        {
            assert_eq!(*val, *exp);
        }
        for (val, exp) in m.subview(0, 1, 2, 3).iter().zip(expected_c.iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_subview_copy_from_borrowed_views()
    {
        let mut m = DMatrix::<i32>::zeros(4, 4);
        let rhs_source = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3);

        let rhs_view = rhs_source.subview(1, 2, 1, 2);
        m.subview_mut(0, 1, 0, 1).copy_from(&rhs_view);

        let mut rhs_source_mut =
            DMatrix::<i32>::from_row_slice(&[10, 11, 12, 13, 14, 15, 16, 17, 18], 3, 3);
        let mut rhs_view_mut = rhs_source_mut.subview_mut(0, 1, 0, 1);
        m.subview_mut(2, 3, 2, 3).copy_from(&mut rhs_view_mut);

        let expected_a = DMatrix::<i32>::from_row_slice(&[5, 6, 8, 9], 2, 2);
        let expected_b = DMatrix::<i32>::from_row_slice(&[10, 11, 13, 14], 2, 2);

        for (val, exp) in m.subview(0, 1, 0, 1).iter().zip(expected_a.iter())
        {
            assert_eq!(*val, *exp);
        }
        for (val, exp) in m.subview(2, 3, 2, 3).iter().zip(expected_b.iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_set_row_borrowed_and_moved()
    {
        let mut m = DMatrix::<i32>::zeros(3, 4);
        let borrowed_rhs = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4], 1, 4);
        m.set_row(0, &borrowed_rhs);

        let moved_rhs = DMatrix::<i32>::from_row_slice(&[5, 6, 7, 8], 1, 4);
        m.set_row(1, moved_rhs);

        for (val, exp) in m.row(0).iter().zip([1, 2, 3, 4].iter())
        {
            assert_eq!(*val, *exp);
        }
        for (val, exp) in m.row(1).iter().zip([5, 6, 7, 8].iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_set_col_borrowed_and_moved()
    {
        let mut m = DMatrix::<i32>::zeros(4, 3);
        let borrowed_rhs = DMatrix::<i32>::from_col_slice(&[1, 2, 3, 4], 4, 1);
        m.set_col(0, &borrowed_rhs);

        let moved_rhs = DMatrix::<i32>::from_col_slice(&[5, 6, 7, 8], 4, 1);
        m.set_col(1, moved_rhs);

        for (val, exp) in m.col(0).iter().zip([1, 2, 3, 4].iter())
        {
            assert_eq!(*val, *exp);
        }
        for (val, exp) in m.col(1).iter().zip([5, 6, 7, 8].iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_set_subview_borrowed_and_moved()
    {
        let mut m = DMatrix::<i32>::zeros(4, 4);
        let borrowed_rhs = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4], 2, 2);
        m.set_subview(0, 1, 0, 1, &borrowed_rhs);

        let moved_rhs = DMatrix::<i32>::from_row_slice(&[5, 6, 7, 8], 2, 2);
        m.set_subview(2, 3, 2, 3, moved_rhs);

        let expected_a = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4], 2, 2);
        let expected_b = DMatrix::<i32>::from_row_slice(&[5, 6, 7, 8], 2, 2);

        for (val, exp) in m.subview(0, 1, 0, 1).iter().zip(expected_a.iter())
        {
            assert_eq!(*val, *exp);
        }
        for (val, exp) in m.subview(2, 3, 2, 3).iter().zip(expected_b.iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    #[should_panic(expected = "dimension mismatch")]
    fn test_row_copy_from_dimension_mismatch_panics()
    {
        let mut m = DMatrix::<i32>::zeros(3, 4);
        m.row_mut(0).copy_from(DMatrix::<i32>::zeros(2, 2));
    }

    #[test]
    #[should_panic(expected = "dimension mismatch")]
    fn test_col_copy_from_dimension_mismatch_panics()
    {
        let mut m = DMatrix::<i32>::zeros(4, 3);
        m.col_mut(0).copy_from(DMatrix::<i32>::zeros(2, 2));
    }

    #[test]
    #[should_panic(expected = "dimension mismatch")]
    fn test_subview_copy_from_dimension_mismatch_panics()
    {
        let mut m = DMatrix::<i32>::zeros(4, 4);
        m.subview_mut(0, 1, 0, 1).copy_from(DMatrix::<i32>::zeros(1, 3));
    }
}
//}}}
//{{{ mod: smatrix_tests
mod smatrix_tests
{

    use topohedral_linalg::smatrix::SMatrix;

    #[test]
    fn test_submatrix()
    {
        let m = SMatrix::<i32, 5, 5>::from_col_slice(&[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25,
        ]);

        let subm = m.subview(1, 3, 1, 3);
        // test via indexing
        for i in 0..2
        {
            for j in 0..2
            {
                assert_eq!(subm[(i, j)], m[(i + 1, j + 1)]);
            }
        }
        // test via iter
        let mut iter = subm.iter();
        for i in 0..3
        {
            for j in 0..3
            {
                assert_eq!(iter.next(), Some(&m[(j + 1, i + 1)]));
            }
        }
    }

    #[test]
    fn test_row()
    {
        let m = SMatrix::<i32, 5, 5>::from_col_slice(&[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25,
        ]);

        let subm = m.row(2);
        let expected = [3, 8, 13, 18, 23];

        for (val, exp) in subm.iter().zip(expected.iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_rows()
    {
        let m = SMatrix::<i32, 5, 5>::from_col_slice(&[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25,
        ]);

        let subm = m.rows(1, 2);
        let expected = [2, 3, 7, 8, 12, 13, 17, 18, 22, 23];

        for (val, exp) in subm.iter().zip(expected.iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_col()
    {
        let m = SMatrix::<i32, 5, 5>::from_col_slice(&[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25,
        ]);

        let subm = m.col(4);
        let expected = [21, 22, 23, 24, 25];

        for (val, exp) in subm.iter().zip(expected.iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_cols()
    {
        let m = SMatrix::<i32, 5, 5>::from_col_slice(&[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25,
        ]);

        let subm = m.cols(1, 3);
        let expected = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

        for (val, exp) in subm.iter().zip(expected.iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_submatrix_mut()
    {
        let mut m = SMatrix::<i32, 5, 5>::from_col_slice(&[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25,
        ]);
        // test read
        let expected = m.subview(1, 3, 1, 3).to_dmatrix();
        let subm = m.subview_mut(1, 3, 1, 3);
        for (val, exp) in subm.iter().zip(expected.iter())
        {
            assert_eq!(*val, *exp);
        }
        // test write
        let mut subm = m.subview_mut(1, 3, 1, 3);
        for val in subm.iter_mut()
        {
            *val = 1;
        }
        let subm = m.subview(1, 3, 1, 3);
        for val in subm.iter()
        {
            assert_eq!(*val, 1);
        }
    }

    #[test]
    fn test_row_mut()
    {
        let mut m = SMatrix::<i32, 5, 5>::from_col_slice(&[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25,
        ]);
        // test read
        {
            let subm = m.row_mut(2);
            let expected = [3, 8, 13, 18, 23];

            for (val, exp) in subm.iter().zip(expected.iter())
            {
                assert_eq!(*val, *exp);
            }
        }
        // test write
        {
            let mut subm = m.row_mut(2);
            for val in subm.iter_mut()
            {
                *val = 1;
            }
            let subm = m.row(2);
            for val in subm.iter()
            {
                assert_eq!(*val, 1);
            }
        }
    }

    #[test]
    fn test_rows_mut()
    {
        let mut m = SMatrix::<i32, 5, 5>::from_col_slice(&[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25,
        ]);
        // test read
        {
            let subm = m.rows_mut(1, 2);
            let expected = [2, 3, 7, 8, 12, 13, 17, 18, 22, 23];

            for (val, exp) in subm.iter().zip(expected.iter())
            {
                assert_eq!(*val, *exp);
            }
        }
        // test write
        {
            let mut subm = m.rows_mut(1, 2);
            for val in subm.iter_mut()
            {
                *val = -1;
            }
            let subm = m.rows(1, 2);
            for val in subm.iter()
            {
                assert_eq!(*val, -1);
            }
        }
    }

    #[test]
    fn test_col_mut()
    {
        let mut m = SMatrix::<i32, 5, 5>::from_col_slice(&[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25,
        ]);
        // test read
        let subm = m.col_mut(4);
        let expected = [21, 22, 23, 24, 25];

        for (val, exp) in subm.iter().zip(expected.iter())
        {
            assert_eq!(*val, *exp);
        }
        // test write
        let mut subm = m.col_mut(4);
        for val in subm.iter_mut()
        {
            *val = -1;
        }
        let subm = m.col(4);
        for val in subm.iter()
        {
            assert_eq!(*val, -1);
        }
    }

    #[test]
    fn test_cols_mut()
    {
        let mut m = SMatrix::<i32, 5, 5>::from_col_slice(&[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25,
        ]);
        // test read
        let subm = m.cols_mut(1, 3);
        let expected = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

        for (val, exp) in subm.iter().zip(expected.iter())
        {
            assert_eq!(*val, *exp);
        }
        // test write
        let mut subm = m.cols_mut(1, 3);
        for val in subm.iter_mut()
        {
            *val = -1;
        }
        let subm = m.cols(1, 3);
        for val in subm.iter()
        {
            assert_eq!(*val, -1);
        }
    }

    #[test]
    fn test_row_copy_from_borrowed_and_moved()
    {
        let mut m = SMatrix::<i32, 3, 5>::zeros();
        let borrowed_rhs = SMatrix::<i32, 1, 5>::from_row_slice(&[1, 2, 3, 4, 5]);
        m.row_mut(0).copy_from(&borrowed_rhs);

        let moved_rhs = SMatrix::<i32, 1, 5>::from_row_slice(&[6, 7, 8, 9, 10]);
        m.row_mut(1).copy_from(moved_rhs);

        let rhs_source = SMatrix::<i32, 2, 5>::from_row_slice(&[11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
        m.row_mut(2).copy_from(rhs_source.row(1));

        for (val, exp) in m.row(0).iter().zip([1, 2, 3, 4, 5].iter())
        {
            assert_eq!(*val, *exp);
        }
        for (val, exp) in m.row(1).iter().zip([6, 7, 8, 9, 10].iter())
        {
            assert_eq!(*val, *exp);
        }
        for (val, exp) in m.row(2).iter().zip([16, 17, 18, 19, 20].iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_col_copy_from_borrowed_and_moved()
    {
        let mut m = SMatrix::<i32, 5, 3>::zeros();
        let borrowed_rhs = SMatrix::<i32, 5, 1>::from_col_slice(&[1, 2, 3, 4, 5]);
        m.col_mut(0).copy_from(&borrowed_rhs);

        let moved_rhs = SMatrix::<i32, 5, 1>::from_col_slice(&[6, 7, 8, 9, 10]);
        m.col_mut(1).copy_from(moved_rhs);

        let rhs_source = SMatrix::<i32, 5, 2>::from_col_slice(&[11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
        m.col_mut(2).copy_from(rhs_source.col(1));

        for (val, exp) in m.col(0).iter().zip([1, 2, 3, 4, 5].iter())
        {
            assert_eq!(*val, *exp);
        }
        for (val, exp) in m.col(1).iter().zip([6, 7, 8, 9, 10].iter())
        {
            assert_eq!(*val, *exp);
        }
        for (val, exp) in m.col(2).iter().zip([16, 17, 18, 19, 20].iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_subview_copy_from_borrowed_and_moved()
    {
        let mut m = SMatrix::<i32, 4, 4>::zeros();
        let borrowed_rhs = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 2, 3, 4]);
        m.subview_mut(0, 1, 0, 1).copy_from(&borrowed_rhs);

        let moved_rhs = SMatrix::<i32, 2, 2>::from_row_slice(&[5, 6, 7, 8]);
        m.subview_mut(2, 3, 2, 3).copy_from(moved_rhs);

        let rhs_source = SMatrix::<i32, 3, 3>::from_row_slice(&[10, 11, 12, 13, 14, 15, 16, 17, 18]);
        m.subview_mut(0, 1, 2, 3).copy_from(rhs_source.subview(1, 2, 0, 1));

        let expected_a = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 2, 3, 4]);
        let expected_b = SMatrix::<i32, 2, 2>::from_row_slice(&[5, 6, 7, 8]);
        let expected_c = SMatrix::<i32, 2, 2>::from_row_slice(&[13, 14, 16, 17]);

        for (val, exp) in m.subview(0, 1, 0, 1).iter().zip(expected_a.iter())
        {
            assert_eq!(*val, *exp);
        }
        for (val, exp) in m.subview(2, 3, 2, 3).iter().zip(expected_b.iter())
        {
            assert_eq!(*val, *exp);
        }
        for (val, exp) in m.subview(0, 1, 2, 3).iter().zip(expected_c.iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_subview_copy_from_borrowed_views()
    {
        let mut m = SMatrix::<i32, 4, 4>::zeros();
        let rhs_source = SMatrix::<i32, 3, 3>::from_row_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let rhs_view = rhs_source.subview(1, 2, 1, 2);
        m.subview_mut(0, 1, 0, 1).copy_from(&rhs_view);

        let mut rhs_source_mut =
            SMatrix::<i32, 3, 3>::from_row_slice(&[10, 11, 12, 13, 14, 15, 16, 17, 18]);
        let mut rhs_view_mut = rhs_source_mut.subview_mut(0, 1, 0, 1);
        m.subview_mut(2, 3, 2, 3).copy_from(&mut rhs_view_mut);

        let expected_a = SMatrix::<i32, 2, 2>::from_row_slice(&[5, 6, 8, 9]);
        let expected_b = SMatrix::<i32, 2, 2>::from_row_slice(&[10, 11, 13, 14]);

        for (val, exp) in m.subview(0, 1, 0, 1).iter().zip(expected_a.iter())
        {
            assert_eq!(*val, *exp);
        }
        for (val, exp) in m.subview(2, 3, 2, 3).iter().zip(expected_b.iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_set_row_borrowed_and_moved()
    {
        let mut m = SMatrix::<i32, 3, 4>::zeros();
        let borrowed_rhs = SMatrix::<i32, 1, 4>::from_row_slice(&[1, 2, 3, 4]);
        m.set_row(0, &borrowed_rhs);

        let moved_rhs = SMatrix::<i32, 1, 4>::from_row_slice(&[5, 6, 7, 8]);
        m.set_row(1, moved_rhs);

        for (val, exp) in m.row(0).iter().zip([1, 2, 3, 4].iter())
        {
            assert_eq!(*val, *exp);
        }
        for (val, exp) in m.row(1).iter().zip([5, 6, 7, 8].iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_set_col_borrowed_and_moved()
    {
        let mut m = SMatrix::<i32, 4, 3>::zeros();
        let borrowed_rhs = SMatrix::<i32, 4, 1>::from_col_slice(&[1, 2, 3, 4]);
        m.set_col(0, &borrowed_rhs);

        let moved_rhs = SMatrix::<i32, 4, 1>::from_col_slice(&[5, 6, 7, 8]);
        m.set_col(1, moved_rhs);

        for (val, exp) in m.col(0).iter().zip([1, 2, 3, 4].iter())
        {
            assert_eq!(*val, *exp);
        }
        for (val, exp) in m.col(1).iter().zip([5, 6, 7, 8].iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_set_subview_borrowed_and_moved()
    {
        let mut m = SMatrix::<i32, 4, 4>::zeros();
        let borrowed_rhs = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 2, 3, 4]);
        m.set_subview(0, 1, 0, 1, &borrowed_rhs);

        let moved_rhs = SMatrix::<i32, 2, 2>::from_row_slice(&[5, 6, 7, 8]);
        m.set_subview(2, 3, 2, 3, moved_rhs);

        let expected_a = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 2, 3, 4]);
        let expected_b = SMatrix::<i32, 2, 2>::from_row_slice(&[5, 6, 7, 8]);

        for (val, exp) in m.subview(0, 1, 0, 1).iter().zip(expected_a.iter())
        {
            assert_eq!(*val, *exp);
        }
        for (val, exp) in m.subview(2, 3, 2, 3).iter().zip(expected_b.iter())
        {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    #[should_panic(expected = "dimension mismatch")]
    fn test_row_copy_from_dimension_mismatch_panics()
    {
        let mut m = SMatrix::<i32, 3, 4>::zeros();
        m.row_mut(0).copy_from(SMatrix::<i32, 2, 2>::zeros());
    }

    #[test]
    #[should_panic(expected = "dimension mismatch")]
    fn test_col_copy_from_dimension_mismatch_panics()
    {
        let mut m = SMatrix::<i32, 4, 3>::zeros();
        m.col_mut(0).copy_from(SMatrix::<i32, 2, 2>::zeros());
    }

    #[test]
    #[should_panic(expected = "dimension mismatch")]
    fn test_subview_copy_from_dimension_mismatch_panics()
    {
        let mut m = SMatrix::<i32, 4, 4>::zeros();
        m.subview_mut(0, 1, 0, 1).copy_from(SMatrix::<i32, 1, 3>::zeros());
    }
}
//}}}
