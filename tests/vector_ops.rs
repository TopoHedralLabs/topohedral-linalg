#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![feature(impl_trait_in_assoc_type)]

mod scvector_tests
{
    use approx::assert_relative_eq;
    use topohedral_linalg::scvector::SCVector;
    use topohedral_linalg::{FloatVectorOps, VectorOps};

    #[test]
    fn test_norm()
    {
        let v = SCVector::<f64, 3>::from_col_slice(&[1.0, 2.0, 3.0]);
        let norm = v.norm();
        assert_relative_eq!(norm, 3.7416573867739413);
    }

    #[test]
    fn test_dot()
    {
        let v1 = SCVector::<f64, 3>::from_col_slice(&[1.0, 2.0, 3.0]);
        let v2 = SCVector::<f64, 3>::from_col_slice(&[4.0, 5.0, 6.0]);
        let dot = v1.dot(&v2);
        assert_relative_eq!(dot, 32.0);
    }

    #[test]
    fn test_normalize()
    {
        let v = SCVector::<f64, 3>::from_col_slice(&[1.0, 2.0, 3.0]);
        let norm = v.norm();
        let normalized = v.normalize();
        assert_relative_eq!(normalized[0], 1.0 / norm);
        assert_relative_eq!(normalized[1], 2.0 / norm);
        assert_relative_eq!(normalized[2], 3.0 / norm);
    }

    #[test]
    fn test_cross()
    {
        let v1 = SCVector::<f64, 3>::from_col_slice(&[1.0, 2.0, 3.0]);
        let v2 = SCVector::<f64, 3>::from_col_slice(&[4.0, 5.0, 6.0]);
        let cross = v1.cross(&v2);
        assert_relative_eq!(cross[0], -3.0);
        assert_relative_eq!(cross[1], 6.0);
        assert_relative_eq!(cross[2], -3.0);
    }

    #[test]
    fn test_angle()
    {
        let v1 = SCVector::<f64, 3>::from_col_slice(&[1.0, 0.0, 0.0]);
        let v2 = SCVector::<f64, 3>::from_col_slice(&[0.0, 1.0, 0.0]);
        let angle1 = v1.angle(&v2);
        assert_relative_eq!(angle1, std::f64::consts::FRAC_PI_2);
        let angle2 = v2.angle(&v1);
        assert_relative_eq!(angle2, std::f64::consts::FRAC_PI_2);
    }
}

mod srvector_tests
{

    use approx::assert_relative_eq;
    use topohedral_linalg::srvector::SRVector;
    use topohedral_linalg::{FloatVectorOps, VectorOps};

    #[test]
    fn test_norm()
    {
        let v = SRVector::<f64, 3>::from_row_slice(&[1.0, 2.0, 3.0]);
        let norm = v.norm();
        assert_relative_eq!(norm, 3.7416573867739413);
    }

    #[test]
    fn test_dot()
    {
        let v1 = SRVector::<f64, 3>::from_row_slice(&[1.0, 2.0, 3.0]);
        let v2 = SRVector::<f64, 3>::from_row_slice(&[4.0, 5.0, 6.0]);
        let dot = v1.dot(&v2);
        assert_relative_eq!(dot, 32.0);
    }

    #[test]
    fn test_normalize()
    {
        let v = SRVector::<f64, 3>::from_row_slice(&[1.0, 2.0, 3.0]);
        let norm = v.norm();
        let normalized = v.normalize();
        assert_relative_eq!(normalized[0], 1.0 / norm);
        assert_relative_eq!(normalized[1], 2.0 / norm);
        assert_relative_eq!(normalized[2], 3.0 / norm);
    }

    #[test]
    fn test_cross()
    {
        let v1 = SRVector::<f64, 3>::from_row_slice(&[1.0, 2.0, 3.0]);
        let v2 = SRVector::<f64, 3>::from_row_slice(&[4.0, 5.0, 6.0]);
        let cross = v1.cross(&v2);
        assert_relative_eq!(cross[0], -3.0);
        assert_relative_eq!(cross[1], 6.0);
        assert_relative_eq!(cross[2], -3.0);
    }

    #[test]
    fn test_angle()
    {
        let v1 = SRVector::<f64, 3>::from_row_slice(&[1.0, 0.0, 0.0]);
        let v2 = SRVector::<f64, 3>::from_row_slice(&[0.0, 1.0, 0.0]);
        let angle1 = v1.angle(&v2);
        assert_relative_eq!(angle1, std::f64::consts::FRAC_PI_2);
        let angle2 = v2.angle(&v1);
        assert_relative_eq!(angle2, std::f64::consts::FRAC_PI_2);
    }
}
mod dvector_tests
{
    use approx::assert_relative_eq;
    use topohedral_linalg::dvector::{DVector, VecType};
    use topohedral_linalg::{FloatVectorOps, VectorOps};

    #[test]
    fn test_norm()
    {
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
    fn test_dot()
    {
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
    fn test_normalize()
    {
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
    fn test_cross()
    {
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
    fn test_angle()
    {
        {
            let v1 = DVector::<f64>::from_slice_vec(&[1.0, 0.0, 0.0], 3, VecType::Row);
            let v2 = DVector::<f64>::from_slice_vec(&[0.0, 1.0, 0.0], 3, VecType::Row);
            let angle1 = v1.angle(&v2);
            assert_relative_eq!(angle1, std::f64::consts::FRAC_PI_2);
            let angle2 = v2.angle(&v1);
            assert_relative_eq!(angle2, std::f64::consts::FRAC_PI_2);
        }
    }
}
