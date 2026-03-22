#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

mod dmatrix_tests
{
    use topohedral_linalg::dmatrix::*;
    use topohedral_linalg::ReduceOps;

    #[test]
    fn test_fold_and_fold_indexed()
    {
        let a = DMatrix::<i32>::from_row_slice(&[3, -2, 5, 4, -7, 1], 2, 3);

        let sum = a.fold(0, |acc, value| acc + value);
        let weighted_sum =
            a.fold_indexed(0, |acc, (row, col), value| acc + (row + col) as i32 * value);

        assert_eq!(sum, 4);
        assert_eq!(weighted_sum, 1);
    }

    #[test]
    fn test_sum_and_product()
    {
        let a = DMatrix::<i32>::from_row_slice(&[3, -2, 5, 4, -7, 1], 2, 3);

        assert_eq!(a.sum(), 4);
        assert_eq!(a.product(), 840);
    }

    #[test]
    fn test_min_max_and_arg_reductions()
    {
        let a = DMatrix::<i32>::from_row_slice(&[3, -2, 5, 4, -7, 1], 2, 3);

        assert_eq!(a.min(), Some(-7));
        assert_eq!(a.max(), Some(5));
        assert_eq!(a.argmin(), Some(((1, 1), -7)));
        assert_eq!(a.argmax(), Some(((0, 2), 5)));
    }

    #[test]
    fn test_abs_and_transform_reductions()
    {
        let a = DMatrix::<i32>::from_row_slice(&[3, -2, 5, 4, -7, 1], 2, 3);

        assert_eq!(a.abs_min(), Some(1));
        assert_eq!(a.abs_max(), Some(-7));
        assert_eq!(a.transform_min(|value| value * value), Some(1));
        assert_eq!(a.transform_max(|value| value * value), Some(-7));
    }

    #[test]
    fn test_empty_matrix_reductions()
    {
        let a = DMatrix::<i32>::zeros(0, 0);

        assert!(a.is_empty());
        assert_eq!(a.sum(), 0);
        assert_eq!(a.product(), 1);
        assert_eq!(a.min(), None);
        assert_eq!(a.max(), None);
        assert_eq!(a.argmin(), None);
        assert_eq!(a.argmax(), None);
    }
}

mod smatrix_tests
{
    use topohedral_linalg::smatrix::*;
    use topohedral_linalg::ReduceOps;

    #[test]
    fn test_fold_and_fold_indexed()
    {
        let a = SMatrix::<i32, 2, 3>::from_row_slice(&[3, -2, 5, 4, -7, 1]);

        let sum = a.fold(0, |acc, value| acc + value);
        let weighted_sum =
            a.fold_indexed(0, |acc, (row, col), value| acc + (row + col) as i32 * value);

        assert_eq!(sum, 4);
        assert_eq!(weighted_sum, 1);
    }

    #[test]
    fn test_sum_and_product()
    {
        let a = SMatrix::<i32, 2, 3>::from_row_slice(&[3, -2, 5, 4, -7, 1]);

        assert_eq!(a.sum(), 4);
        assert_eq!(a.product(), 840);
    }

    #[test]
    fn test_min_max_and_arg_reductions()
    {
        let a = SMatrix::<i32, 2, 3>::from_row_slice(&[3, -2, 5, 4, -7, 1]);

        assert_eq!(a.min(), Some(-7));
        assert_eq!(a.max(), Some(5));
        assert_eq!(a.argmin(), Some(((1, 1), -7)));
        assert_eq!(a.argmax(), Some(((0, 2), 5)));
    }

    #[test]
    fn test_abs_and_transform_reductions()
    {
        let a = SMatrix::<i32, 2, 3>::from_row_slice(&[3, -2, 5, 4, -7, 1]);

        assert_eq!(a.abs_min(), Some(1));
        assert_eq!(a.abs_max(), Some(-7));
        assert_eq!(a.transform_min(|value| value * value), Some(1));
        assert_eq!(a.transform_max(|value| value * value), Some(-7));
    }

    #[test]
    fn test_empty_matrix_reductions()
    {
        let a = SMatrix::<i32, 0, 0>::zeros();

        assert!(a.is_empty());
        assert_eq!(a.sum(), 0);
        assert_eq!(a.product(), 1);
        assert_eq!(a.min(), None);
        assert_eq!(a.max(), None);
        assert_eq!(a.argmin(), None);
        assert_eq!(a.argmax(), None);
    }
}
