#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

mod dmatrix_tests
{
    use topohedral_linalg::dmatrix::*;
    use topohedral_linalg::ReduceOps;

    fn assert_rectangular_view_reductions<R>(view: &R)
    where
        R: ReduceOps<Item = i32, Index = (usize, usize)>,
    {
        let traversal = view.fold(Vec::new(), |mut acc, value| {
            acc.push(value);
            acc
        });
        let indexed_traversal = view.fold_indexed(Vec::new(), |mut acc, index, value| {
            acc.push((index, value));
            acc
        });

        assert_eq!(traversal, vec![6, 10, 14, 7, 11, 15]);
        assert_eq!(
            indexed_traversal,
            vec![
                ((0, 0), 6),
                ((1, 0), 10),
                ((2, 0), 14),
                ((0, 1), 7),
                ((1, 1), 11),
                ((2, 1), 15),
            ]
        );
        assert_eq!(view.sum(), 63);
        assert_eq!(view.product(), 970200);
        assert_eq!(view.min(), Some(6));
        assert_eq!(view.max(), Some(15));
        assert_eq!(view.argmin(), Some(((0, 0), 6)));
        assert_eq!(view.argmax(), Some(((2, 1), 15)));
    }

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

    #[test]
    fn test_immutable_subview_reductions_use_column_major_order_and_local_indices()
    {
        let matrix = DMatrix::<i32>::from_row_slice(
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            4,
            4,
        );
        let view = matrix.subview(1, 3, 1, 2);

        assert_rectangular_view_reductions(&view);
    }

    #[test]
    fn test_mutable_subview_reductions_use_column_major_order_and_local_indices()
    {
        let mut matrix = DMatrix::<i32>::from_row_slice(
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            4,
            4,
        );
        let view = matrix.subview_mut(1, 3, 1, 2);

        assert_rectangular_view_reductions(&view);
    }
}

mod smatrix_tests
{
    use topohedral_linalg::smatrix::*;
    use topohedral_linalg::ReduceOps;

    fn assert_rectangular_view_reductions<R>(view: &R)
    where
        R: ReduceOps<Item = i32, Index = (usize, usize)>,
    {
        let traversal = view.fold(Vec::new(), |mut acc, value| {
            acc.push(value);
            acc
        });
        let indexed_traversal = view.fold_indexed(Vec::new(), |mut acc, index, value| {
            acc.push((index, value));
            acc
        });

        assert_eq!(traversal, vec![6, 10, 14, 7, 11, 15]);
        assert_eq!(
            indexed_traversal,
            vec![
                ((0, 0), 6),
                ((1, 0), 10),
                ((2, 0), 14),
                ((0, 1), 7),
                ((1, 1), 11),
                ((2, 1), 15),
            ]
        );
        assert_eq!(view.sum(), 63);
        assert_eq!(view.product(), 970200);
        assert_eq!(view.min(), Some(6));
        assert_eq!(view.max(), Some(15));
        assert_eq!(view.argmin(), Some(((0, 0), 6)));
        assert_eq!(view.argmax(), Some(((2, 1), 15)));
    }

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

    #[test]
    fn test_immutable_subview_reductions_use_column_major_order_and_local_indices()
    {
        let matrix = SMatrix::<i32, 4, 4>::from_row_slice(&[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        ]);
        let view = matrix.subview(1, 3, 1, 2);

        assert_rectangular_view_reductions(&view);
    }

    #[test]
    fn test_mutable_subview_reductions_use_column_major_order_and_local_indices()
    {
        let mut matrix = SMatrix::<i32, 4, 4>::from_row_slice(&[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        ]);
        let view = matrix.subview_mut(1, 3, 1, 2);

        assert_rectangular_view_reductions(&view);
    }
}
