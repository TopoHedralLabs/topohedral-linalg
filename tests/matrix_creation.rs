#![feature(generic_const_exprs)]
#![feature(impl_trait_in_assoc_type)]

//{{{  mod: smatrix_tests
mod smatrix_tests
{

    use topohedral_linalg::*;

    #[test]
    fn test_default()
    {
        let matrix = SMatrix::<i32, 2, 2>::default();

        for val in &matrix
        {
            assert_eq!(*val, 0);
        }
    }

    #[test]

    fn test_from_val()
    {
        let matrix = SMatrix::<i32, 2, 2>::from_value(10);

        for val in &matrix
        {
            assert_eq!(*val, 10);
        }
    }

    #[test]

    fn test_from_row_slice()
    {
        let matrix = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 10, 100, 1000]);

        for (res, exp) in matrix.iter().zip([1, 100, 10, 1000].iter())
        {
            assert_eq!(*res, *exp);
        }
    }

    #[test]

    fn test_from_col_slice()
    {
        let matrix = SMatrix::<i32, 2, 2>::from_col_slice(&[1, 10, 100, 1000]);

        for (res, exp) in matrix.iter().zip([1, 10, 100, 1000].iter())
        {
            assert_eq!(*res, *exp);
        }
    }

    #[test]

    fn test_from_uniform_random()
    {
        let matrix = SMatrix::<f64, 2, 2>::from_uniform_random(-1100.0, 100.1);

        for val in &matrix
        {
            assert!(*val >= -1100.0 && *val <= 100.1);
        }
    }

    #[test]
    fn test_indexing()
    {
        let matrix = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 10, 100, 1000]);
        assert_eq!(matrix[(0, 0)], 1);
        assert_eq!(matrix[(0, 1)], 10);
        assert_eq!(matrix[(1, 0)], 100);
        assert_eq!(matrix[(1, 1)], 1000);
    }

    #[test]
    fn test_serde()
    {
        let matrix = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 10, 100, 1000]);
        let matrix_json = serde_json::to_string_pretty(&matrix).unwrap();
        let matrix2: SMatrix<i32, 2, 2> = serde_json::from_str(&matrix_json).unwrap();

        for (val1, val2) in matrix.iter().zip(matrix2.iter())
        {
            assert_eq!(*val1, *val2);
        }
    }

    #[test]
    fn test_transpose()
    {
        let matrix = SMatrix::<i32, 2, 3>::from_row_slice(&[1, 2, 3, 4, 5, 6]);
        let transposed = matrix.transpose();
        assert_eq!(transposed[(0, 0)], 1);
        assert_eq!(transposed[(0, 1)], 4);
        assert_eq!(transposed[(1, 0)], 2);
        assert_eq!(transposed[(1, 1)], 5);
        assert_eq!(transposed[(2, 0)], 3);
        assert_eq!(transposed[(2, 1)], 6);
    }

    #[test]
    fn test_copy_from()
    {
        let mut matrix = SMatrix::<i32, 2, 2>::default();
        let matrix2 = SMatrix::<i32, 2, 2>::from_row_slice(&[1, 2, 3, 4]);
        matrix.copy_from(&matrix2);

        for (val1, val2) in matrix.iter().zip(matrix2.iter())
        {
            assert_eq!(*val1, *val2);
        }
    }
}
//}}}
//{{{ mod: dmatrix_tests
mod dmatrix_tests
{

    use topohedral_linalg::{DMatrix, DMatrixConstructors};

    #[test]
    fn test_matrix_zeros()
    {
        let matrix = DMatrix::<i32>::zeros(2, 2);

        for val in &matrix
        {
            assert_eq!(*val, 0);
        }
    }

    #[test]

    fn test_matrix_from_val()
    {
        let matrix = DMatrix::<i32>::from_value(1, 4, 10);

        for val in &matrix
        {
            assert_eq!(*val, 10);
        }
    }

    #[test]

    fn test_matrix_from_slice()
    {
        let matrix = DMatrix::<i32>::from_row_slice(2, 2, &[1, 10, 100, 1000]);

        for (res, exp) in matrix.iter().zip([1, 100, 10, 1000].iter())
        {
            assert_eq!(*res, *exp);
        }

    }

    #[test]

    fn test_matrix_from_uniform_random()
    {
        let matrix = DMatrix::<f64>::from_uniform_random(4, 4, -1100.0, 100.1);
        for val in &matrix
        {
            assert!(*val >= -1100.0 && *val <= 100.1);
        }
    }

    #[test]
    fn test_matrix_indexing()
    {
        let matrix = DMatrix::<i32>::from_row_slice(2, 2, &[1, 10, 100, 1000]);
        assert_eq!(matrix[(0, 0)], 1);
        assert_eq!(matrix[(0, 1)], 10);
        assert_eq!(matrix[(1, 0)], 100);
        assert_eq!(matrix[(1, 1)], 1000);
    }

    #[test]
    fn test_serde()
    {
        let matrix = DMatrix::<i32>::from_row_slice(2, 2, &[1, 10, 100, 1000]);
        let matrix_json = serde_json::to_string_pretty(&matrix).unwrap();
        let matrix2: DMatrix<i32> = serde_json::from_str(&matrix_json).unwrap();

        for (val1, val2) in matrix.iter().zip(matrix2.iter())
        {
            assert_eq!(*val1, *val2);
        }
    }

    #[test]
    fn test_matrix_transpose()
    {
        let matrix = DMatrix::<i32>::from_row_slice(2, 3, &[1, 2, 3, 4, 5, 6]);
        let transposed = matrix.transpose();
        assert_eq!(transposed[(0, 0)], 1);
        assert_eq!(transposed[(0, 1)], 3);
        assert_eq!(transposed[(1, 0)], 2);
        assert_eq!(transposed[(1, 1)], 5);
        assert_eq!(transposed[(2, 0)], 3);
        assert_eq!(transposed[(2, 1)], 6);
    }
}
//}}}
