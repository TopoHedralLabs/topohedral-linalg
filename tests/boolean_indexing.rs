use topohedral_linalg::{
    DMatrix, ElementwiseCompare, Maskable, MatrixExpr, ReduceOps, SMatrix, Shape, SubViewable,
    SubViewableMut, TransformOps,
};

#[test]
fn boolean_matrices_support_structural_operations()
{
    let mut static_mask =
        SMatrix::<bool, 2, 3>::from_row_slice(&[true, false, true, false, true, false]);
    assert!(static_mask[(0, 0)]);
    static_mask[(1, 0)] = true;
    static_mask.transform(|value| !value);
    assert_eq!(
        static_mask.fold(0, |count, value| count + usize::from(value)),
        2
    );

    {
        let mut view = static_mask.subview_range_mut(0, 1, 1, 2);
        view.fill(true);
    }
    assert!(static_mask
        .subview_range(0, 1, 1, 2)
        .iter()
        .all(|value| *value));

    let encoded = serde_json::to_string(&static_mask).unwrap();
    let decoded: SMatrix<bool, 2, 3> = serde_json::from_str(&encoded).unwrap();
    assert_eq!(
        decoded.iter().copied().collect::<Vec<_>>(),
        static_mask.iter().copied().collect::<Vec<_>>()
    );

    let mut dynamic_mask = DMatrix::<bool>::from_value(false, 2, 3);
    dynamic_mask.copy_from(&decoded);
    assert_eq!(
        dynamic_mask.iter().copied().collect::<Vec<_>>(),
        decoded.iter().copied().collect::<Vec<_>>()
    );

    let dynamic_encoded = serde_json::to_string(&dynamic_mask).unwrap();
    let dynamic_decoded: DMatrix<bool> = serde_json::from_str(&dynamic_encoded).unwrap();
    assert_eq!(
        dynamic_decoded.iter().copied().collect::<Vec<_>>(),
        dynamic_mask.iter().copied().collect::<Vec<_>>()
    );

    assert!(format!("{static_mask}").contains("true"));
    assert!(format!("{}", DMatrix::<i32>::from_value(1, 1, 1)).contains("1.0000e0"));
}

#[test]
fn comparisons_materialise_for_static_and_dynamic_matrices()
{
    let a = SMatrix::<i32, 2, 3>::from_col_slice(&[1, 4, 2, 5, 3, 6]);
    let b = SMatrix::<i32, 2, 3>::from_value(3);

    let eq: SMatrix<bool, 2, 3> = a.eq(3).into();
    let ne: SMatrix<bool, 2, 3> = a.ne(3).into();
    let lt: SMatrix<bool, 2, 3> = a.lt(&b).into();
    let le: SMatrix<bool, 2, 3> = a.le(&b).into();
    let gt: SMatrix<bool, 2, 3> = a.gt(&b).into();
    let ge: SMatrix<bool, 2, 3> = a.ge(&b).into();

    assert_eq!(
        eq.iter().copied().collect::<Vec<_>>(),
        [false, false, false, false, true, false]
    );
    assert_eq!(
        ne.iter().copied().collect::<Vec<_>>(),
        [true, true, true, true, false, true]
    );
    assert_eq!(
        lt.iter().copied().collect::<Vec<_>>(),
        [true, false, true, false, false, false]
    );
    assert_eq!(
        le.iter().copied().collect::<Vec<_>>(),
        [true, false, true, false, true, false]
    );
    assert_eq!(
        gt.iter().copied().collect::<Vec<_>>(),
        [false, true, false, true, false, true]
    );
    assert_eq!(
        ge.iter().copied().collect::<Vec<_>>(),
        [false, true, false, true, true, true]
    );

    let da = DMatrix::<i32>::from_col_slice(&[1, 4, 2, 5, 3, 6], 2, 3);
    let db = DMatrix::<i32>::from_value(3, 2, 3);
    let result: DMatrix<bool> = da.gt(&db).into();
    assert_eq!(result.size(), (2, 3));
    assert_eq!(
        result.iter().copied().collect::<Vec<_>>(),
        gt.iter().copied().collect::<Vec<_>>()
    );
}

#[test]
fn boolean_expressions_compose_lazily()
{
    let values = DMatrix::<i32>::from_col_slice(&[1, 4, 2, 5, 3, 6], 2, 3);

    let combined: DMatrix<bool> = ((values.gt(1) & values.lt(6)) ^ !values.eq(3)).into();
    let expected = [true, false, false, false, true, true];
    assert_eq!(combined.iter().copied().collect::<Vec<_>>(), expected);

    let left = DMatrix::<bool>::from_col_slice(&[true, false, true, false], 2, 2);
    let right = DMatrix::<bool>::from_col_slice(&[true, true, false, false], 2, 2);
    let anded: DMatrix<bool> = (&left & &right).into();
    let ored: DMatrix<bool> = (&left | &right).into();
    let xored: DMatrix<bool> = (&left ^ &right).into();
    let inverted: DMatrix<bool> = (!&left).into();
    assert_eq!(
        anded.iter().copied().collect::<Vec<_>>(),
        [true, false, false, false]
    );
    assert_eq!(
        ored.iter().copied().collect::<Vec<_>>(),
        [true, true, true, false]
    );
    assert_eq!(
        xored.iter().copied().collect::<Vec<_>>(),
        [false, true, true, false]
    );
    assert_eq!(
        inverted.iter().copied().collect::<Vec<_>>(),
        [false, true, false, true]
    );
}

#[test]
fn masked_selection_returns_column_major_column_vector()
{
    let source = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4, 5, 6], 2, 3);
    let threshold = DMatrix::<i32>::from_value(3, 2, 3);

    let selected = source.masked(source.gt(&threshold));
    assert_eq!(selected.size(), (3, 1));
    assert_eq!(selected.iter().copied().collect::<Vec<_>>(), [4, 5, 6]);

    let materialised = selected.to_dmatrix();
    assert_eq!(materialised.size(), (3, 1));
    assert_eq!(materialised.iter().copied().collect::<Vec<_>>(), [4, 5, 6]);

    let static_source = SMatrix::<i32, 2, 3>::from_row_slice(&[1, 2, 3, 4, 5, 6]);
    let static_selected = static_source.masked(static_source.ge(3)).to_dmatrix();
    assert_eq!(static_selected.size(), (4, 1));
    assert_eq!(
        static_selected.iter().copied().collect::<Vec<_>>(),
        [4, 5, 3, 6]
    );
}

#[test]
fn masked_selection_handles_views_and_empty_masks()
{
    let source = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], 3, 4);
    let source_view = source.subview_range(1, 2, 1, 3);
    let mask = DMatrix::<bool>::from_row_slice(&[true, false, true, false, true, false], 2, 3);
    let selected = source_view.masked(&mask).to_dmatrix();
    assert_eq!(selected.iter().copied().collect::<Vec<_>>(), [6, 11, 8]);

    let empty = source.masked(source.gt(100)).to_dmatrix();
    assert_eq!(empty.size(), (0, 1));
    assert!(empty.iter().next().is_none());
}

#[test]
#[should_panic(expected = "masked selection dimension mismatch")]
fn masked_selection_rejects_dimension_mismatch()
{
    let source = DMatrix::<i32>::from_value(1, 2, 3);
    let mask = DMatrix::<bool>::from_value(true, 3, 2);
    let _ = source.masked(mask);
}

#[test]
#[should_panic(expected = "element-wise comparison dimension mismatch")]
fn comparisons_reject_dimension_mismatch()
{
    let lhs = DMatrix::<i32>::from_value(1, 2, 3);
    let rhs = DMatrix::<i32>::from_value(1, 3, 2);
    let _ = lhs.gt(&rhs);
}

#[test]
fn comparison_expressions_implement_matrix_expr()
{
    fn assert_bool_expr<E: MatrixExpr<ScalarType = bool>>(expr: &E)
    {
        assert_eq!(expr.size(), (2, 2));
    }

    let matrix = DMatrix::<i32>::from_col_slice(&[1, 2, 3, 4], 2, 2);
    let expression = matrix.gt(2);
    assert_bool_expr(&expression);
}

#[test]
fn comparisons_accept_views_and_arithmetic_expressions()
{
    let lhs = DMatrix::<i32>::from_row_slice(&[1, 2, 3, 4, 5, 6], 2, 3);
    let rhs = DMatrix::<i32>::from_value(1, 2, 3);
    let sum = &lhs + &rhs;
    let compared: DMatrix<bool> = sum.ge(4).into();
    assert_eq!(
        compared.iter().copied().collect::<Vec<_>>(),
        [false, true, false, true, true, true]
    );

    let view = lhs.subview_range(0, 1, 1, 2);
    let view_mask: DMatrix<bool> = view.lt(6).into();
    assert_eq!(view_mask.size(), (2, 2));
    assert_eq!(
        view_mask.iter().copied().collect::<Vec<_>>(),
        [true, true, true, false]
    );
}
