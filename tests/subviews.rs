#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![feature(impl_trait_in_assoc_type)]


mod dmatrix_tests {

    use topohedral_linalg::dmatrix::subviews::{MatrixView, MatrixViewIter};
    use topohedral_linalg::dmatrix::DMatrix;


    #[test]
    fn test_submatrix() {

        let m = DMatrix::<i32>::from_col_slice(&[1, 2, 3, 4, 5, 
                                                 6, 7, 8, 9, 10, 
                                                 11, 12, 13, 14, 15,
                                                 16, 17, 18, 19, 20, 
                                                 21, 22, 23, 24, 25], 
                                                 5, 5);

        let subm = m.subview(1, 3, 1, 3);

        for i in 0..2 {
            for j in 0..2 {
                assert_eq!(subm[(i, j)], m[(i + 1, j + 1)]);
            }
        }
    }

    #[test]
    fn test_row() {

        let m = DMatrix::<i32>::from_col_slice(&[1, 2, 3, 4, 5, 
                                                 6, 7, 8, 9, 10, 
                                                 11, 12, 13, 14, 15,
                                                 16, 17, 18, 19, 20, 
                                                 21, 22, 23, 24, 25], 
                                                 5, 5);

        let subm = m.row(2);
        let expected = [3, 8, 13, 18, 23];

        for (val, exp) in subm.iter().zip(expected.iter()) {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_rows() {
        let m = DMatrix::<i32>::from_col_slice(&[1,  2,  3,  4,  5, 
                                                 6,  7,  8,  9,  10, 
                                                 11, 12, 13, 14, 15,
                                                 16, 17, 18, 19, 20, 
                                                 21, 22, 23, 24, 25], 
                                                 5, 5);

        let subm = m.rows(1, 2);
        let expected = [2, 3, 7,8,12,13,17,18,22,23];


        for (val, exp) in subm.iter().zip(expected.iter()) {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_col() {

        let m = DMatrix::<i32>::from_col_slice(&[1, 2, 3, 4, 5, 
                                                 6, 7, 8, 9, 10, 
                                                 11, 12, 13, 14, 15,
                                                 16, 17, 18, 19, 20, 
                                                 21, 22, 23, 24, 25], 
                                                 5, 5);

        let subm = m.col(4);
        let expected = [21, 22, 23, 24, 25];

        for (val, exp) in subm.iter().zip(expected.iter()) {
            assert_eq!(*val, *exp);
        }
    }

    #[test]
    fn test_cols() {
        let m = DMatrix::<i32>::from_col_slice(&[1, 2, 3, 4, 5, 
                                                 6, 7, 8, 9, 10, 
                                                 11, 12, 13, 14, 15,
                                                 16, 17, 18, 19, 20, 
                                                 21, 22, 23, 24, 25], 
                                                 5, 5);

        let subm = m.cols(1,3);
        let expected = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

        for (val, exp) in subm.iter().zip(expected.iter()) {
            assert_eq!(*val,*exp);
        }
    }


    #[test]
    fn test_submatrix_iter() {
        let m = DMatrix::<i32>::from_col_slice(&[1, 2, 3, 4, 5, 
                                                 6, 7, 8, 9, 10, 
                                                 11, 12, 13, 14, 15,
                                                 16, 17, 18, 19, 20, 
                                                 21, 22, 23, 24, 25], 
                                                 5, 5);

        let subm = m.subview(1, 3, 1, 3);

        let mut iter = subm.iter();

        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(iter.next(), Some(&m[(j + 1, i + 1)]));
            }
        }
    }
}