#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

mod dmatrix_tests
{
    use topohedral_linalg::Dimension;
    use topohedral_linalg::dmatrix::*;

    fn assert_matrix_eq(
        actual: &DMatrix<i32>,
        expected: &DMatrix<i32>,
    )
    {
        for (actual_value, expected_value) in actual.iter().zip(expected.iter())
        {
            assert_eq!(*actual_value, *expected_value);
        }
    }

    #[test]
    fn test_sort_rows_in_place()
    {
        let mut a = DMatrix::<i32>::from_row_slice(&[3, 1, 2, 6, 4, 5], 2, 3);
        let expected = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4, 5, 6], 2, 3);
        a.sort(Dimension::Rows);
        assert_matrix_eq(&a, &expected);
    }

    #[test]
    fn test_sort_cols_in_place()
    {
        let mut a = DMatrix::<i32>::from_row_slice(&[6, 4, 5, 3, 1, 2], 2, 3);
        let expected = DMatrix::<i32>::from_row_slice(&[3, 1, 2, 6, 4, 5], 2, 3);
        a.sort(Dimension::Cols);
        assert_matrix_eq(&a, &expected);
    }

    #[test]
    fn test_sort_all_in_place()
    {
        let mut a = DMatrix::<i32>::from_row_slice(&[3, 1, 2, 6, 4, 5], 2, 3);
        let expected = DMatrix::<i32>::from_row_slice(&[1, 3, 5, 2, 4, 6], 2, 3);
        a.sort(Dimension::All);
        assert_matrix_eq(&a, &expected);
    }

    #[test]
    fn test_sorted_returns_sorted_copy()
    {
        let a = DMatrix::<i32>::from_row_slice(&[3, 1, 2, 6, 4, 5], 2, 3);
        let original = DMatrix::<i32>::from_row_slice(&[3, 1, 2, 6, 4, 5], 2, 3);
        let expected = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4, 5, 6], 2, 3);
        let sorted = a.sorted(Dimension::Rows);
        assert_matrix_eq(&sorted, &expected);
        assert_matrix_eq(&a, &original);
    }

    #[test]
    fn test_into_sorted_returns_sorted_matrix()
    {
        let a = DMatrix::<i32>::from_row_slice(&[6, 4, 5, 3, 1, 2], 2, 3);
        let expected = DMatrix::<i32>::from_row_slice(&[3, 1, 2, 6, 4, 5], 2, 3);
        let sorted = a.into_sorted(Dimension::Cols);
        assert_matrix_eq(&sorted, &expected);
    }
}

mod smatrix_tests
{
    use topohedral_linalg::Dimension;
    use topohedral_linalg::smatrix::*;

    fn assert_matrix_eq<const N: usize, const M: usize>(
        actual: &SMatrix<i32, N, M>,
        expected: &SMatrix<i32, N, M>,
    )
    where
        [(); N * M]:,
    {
        for (actual_value, expected_value) in actual.iter().zip(expected.iter())
        {
            assert_eq!(*actual_value, *expected_value);
        }
    }

    #[test]
    fn test_sort_rows_in_place()
    {
        let mut a = SMatrix::<i32, 2, 3>::from_row_slice(&[3, 1, 2, 6, 4, 5]);
        let expected = SMatrix::<i32, 2, 3>::from_row_slice(&[1, 2, 3, 4, 5, 6]);
        a.sort(Dimension::Rows);
        assert_matrix_eq(&a, &expected);
    }

    #[test]
    fn test_sort_cols_in_place()
    {
        let mut a = SMatrix::<i32, 2, 3>::from_row_slice(&[6, 4, 5, 3, 1, 2]);
        let expected = SMatrix::<i32, 2, 3>::from_row_slice(&[3, 1, 2, 6, 4, 5]);
        a.sort(Dimension::Cols);
        assert_matrix_eq(&a, &expected);
    }

    #[test]
    fn test_sort_all_in_place()
    {
        let mut a = SMatrix::<i32, 2, 3>::from_row_slice(&[3, 1, 2, 6, 4, 5]);
        let expected = SMatrix::<i32, 2, 3>::from_row_slice(&[1, 3, 5, 2, 4, 6]);
        a.sort(Dimension::All);
        assert_matrix_eq(&a, &expected);
    }

    #[test]
    fn test_sorted_returns_sorted_copy()
    {
        let a = SMatrix::<i32, 2, 3>::from_row_slice(&[3, 1, 2, 6, 4, 5]);
        let original = SMatrix::<i32, 2, 3>::from_row_slice(&[3, 1, 2, 6, 4, 5]);
        let expected = SMatrix::<i32, 2, 3>::from_row_slice(&[1, 2, 3, 4, 5, 6]);
        let sorted = a.sorted(Dimension::Rows);
        assert_matrix_eq(&sorted, &expected);
        assert_matrix_eq(&a, &original);
    }

    #[test]
    fn test_into_sorted_returns_sorted_matrix()
    {
        let a = SMatrix::<i32, 2, 3>::from_row_slice(&[6, 4, 5, 3, 1, 2]);
        let expected = SMatrix::<i32, 2, 3>::from_row_slice(&[3, 1, 2, 6, 4, 5]);
        let sorted = a.into_sorted(Dimension::Cols);
        assert_matrix_eq(&sorted, &expected);
    }
}
