use std::fmt::Debug;

use topohedral_linalg::{
    DCholeskyError, DLuError, DMatrix, SMatrix, Shape, SquareMatrixOps, SubViewable, SubViewableMut,
};

fn assert_clone_debug_default<T: Clone + Debug + Default>() {}
fn assert_clone_debug_eq_error<T: Clone + Debug + Eq + std::error::Error>() {}

#[test]
fn matrices_support_non_copy_elements_for_storage_apis() {
    assert_clone_debug_default::<DMatrix<String>>();
    assert_clone_debug_default::<SMatrix<String, 2, 2>>();

    let dynamic = DMatrix::from_row_vec(
        vec![
            "a".to_owned(),
            "b".to_owned(),
            "c".to_owned(),
            "d".to_owned(),
        ],
        2,
        2,
    );
    assert_eq!(
        dynamic.as_ref(),
        &[
            "a".to_owned(),
            "c".to_owned(),
            "b".to_owned(),
            "d".to_owned()
        ]
    );
    assert_eq!(dynamic.nrows(), 2);
    assert_eq!(dynamic.ncols(), 2);
    {
        let view = dynamic.subview_range(0, 1, 0, 0);
        assert_eq!((&view).into_iter().cloned().collect::<Vec<_>>(), ["a", "c"]);
        let indexed = dynamic.subview_indices([1, 0], [1]);
        assert_eq!(
            (&indexed).into_iter().cloned().collect::<Vec<_>>(),
            ["d", "b"]
        );
    }
    assert_eq!(
        dynamic.into_iter().collect::<Vec<_>>(),
        ["a", "c", "b", "d"]
    );

    let fixed = SMatrix::<String, 2, 2>::from_row_slice(&[
        "a".to_owned(),
        "b".to_owned(),
        "c".to_owned(),
        "d".to_owned(),
    ]);
    assert_eq!(fixed.nrows(), 2);
    assert_eq!(fixed.ncols(), 2);
    let encoded = serde_json::to_string(&fixed).unwrap();
    let decoded: SMatrix<String, 2, 2> = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded.as_ref(), fixed.as_ref());
    assert_eq!(fixed.into_iter().collect::<Vec<_>>(), ["a", "c", "b", "d"]);
}

#[test]
fn mutable_views_support_standard_iteration() {
    let mut matrix = DMatrix::from_row_vec(vec![1_i32, 2, 3, 4], 2, 2);
    {
        let mut view = matrix.subview_range_mut(0, 1, 0, 1);
        for value in &mut view {
            *value *= 2;
        }
        assert_eq!(
            (&view).into_iter().copied().collect::<Vec<_>>(),
            [2, 6, 4, 8]
        );
    }
    assert_eq!(matrix.as_ref(), &[2, 6, 4, 8]);
}

#[test]
fn dynamic_deserialization_rejects_inconsistent_storage() {
    let malformed = r#"{"data":[1,2,3],"nrows":2,"ncols":2}"#;
    let error = serde_json::from_str::<DMatrix<i32>>(malformed).unwrap_err();
    assert!(error
        .to_string()
        .contains("matrix data length does not match its dimensions"));
}

#[test]
fn public_decomposition_errors_have_common_error_traits() {
    assert_clone_debug_eq_error::<DCholeskyError>();
    assert_clone_debug_eq_error::<DLuError>();
}

#[test]
fn trace_and_determinant_are_correct_for_square_matrices() {
    let empty = DMatrix::<f64>::zeros(0, 0);
    assert_eq!(empty.trace(), 0.0);
    assert_eq!(empty.determinant(), 1.0);

    let one = DMatrix::from_row_slice(&[7.0_f64], 1, 1);
    assert_eq!(one.trace(), 7.0);
    assert_eq!(one.determinant(), 7.0);

    let pivoted = DMatrix::from_row_slice(&[0.0_f64, 1.0, 1.0, 0.0], 2, 2);
    assert_eq!(pivoted.trace(), 0.0);
    assert_eq!(pivoted.determinant(), -1.0);

    let singular = DMatrix::from_row_slice(&[1.0_f64, 2.0, 2.0, 4.0], 2, 2);
    assert_eq!(singular.determinant(), 0.0);

    let fixed = SMatrix::<f64, 2, 2>::from_row_slice(&[1.0, 2.0, 3.0, 4.0]);
    assert_eq!(fixed.trace(), 5.0);
    assert_eq!(fixed.determinant(), -2.0);

    let fixed_empty = SMatrix::<f64, 0, 0>::zeros();
    assert_eq!(fixed_empty.trace(), 0.0);
    assert_eq!(fixed_empty.determinant(), 1.0);
}

#[test]
fn numerical_lapack_failures_are_semantic() {
    let singular = DMatrix::from_row_slice(&[1.0_f64, 2.0, 2.0, 4.0], 2, 2);
    assert_eq!(singular.lu().unwrap_err(), DLuError::Singular { pivot: 2 });

    let not_positive_definite = DMatrix::from_row_slice(&[1.0_f64, 0.0, 0.0, -1.0], 2, 2);
    assert_eq!(
        not_positive_definite.cholesky().unwrap_err(),
        DCholeskyError::NotPositiveDefinite { minor: 2 }
    );
}

#[test]
#[should_panic(expected = "matrix dimensions overflow usize")]
fn dynamic_storage_size_overflow_panics() {
    let _ = DMatrix::<i32>::zeros(usize::MAX, 2);
}

#[test]
#[should_panic(expected = "vector length must match matrix dimensions")]
fn owned_constructor_rejects_wrong_length() {
    let _ = DMatrix::from_col_vec(vec![1_i32, 2, 3], 2, 2);
}

#[test]
#[should_panic(expected = "determinant is only defined for square matrices")]
fn dynamic_determinant_rejects_rectangular_matrices() {
    let _ = DMatrix::<f64>::zeros(2, 3).determinant();
}

#[test]
#[should_panic(expected = "trace is only defined for square matrices")]
fn dynamic_trace_rejects_rectangular_matrices() {
    let _ = DMatrix::<i32>::zeros(2, 3).trace();
}

#[test]
fn dynamic_lapack_shape_preconditions_are_unconditional() {
    assert!(std::panic::catch_unwind(|| DMatrix::<f64>::zeros(2, 3).cholesky()).is_err());
    assert!(std::panic::catch_unwind(|| DMatrix::<f64>::zeros(2, 3).eig()).is_err());
    assert!(std::panic::catch_unwind(|| DMatrix::<f64>::zeros(2, 3).schur()).is_err());
    assert!(std::panic::catch_unwind(|| DMatrix::<f64>::zeros(2, 3).symeig()).is_err());
    assert!(std::panic::catch_unwind(|| {
        DMatrix::<f64>::identity(2, 2).solve(DMatrix::zeros(3, 1))
    })
    .is_err());
}

#[test]
#[should_panic(expected = "subview row range must be ordered")]
fn subview_rejects_reversed_ranges() {
    let matrix = DMatrix::<i32>::zeros(2, 2);
    let _ = matrix.subview_range(1, 0, 0, 1);
}

#[test]
#[should_panic(expected = "cannot create a row view of a matrix with no columns")]
fn row_view_rejects_zero_column_matrix() {
    let matrix = DMatrix::<i32>::zeros(2, 0);
    let _ = matrix.row(0);
}
