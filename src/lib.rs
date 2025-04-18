//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------
#![feature(generic_const_exprs)]
#![feature(impl_trait_in_assoc_type)]

mod common;
mod matrix;
mod vector;

pub use common::{Field, Complex};
pub use matrix::{DMatrixConstructors, SMatrixConstructors};
pub use matrix::smatrix::{EvaluateSMatrix, SMatrix};
pub use matrix::dmatrix::{EvaluateDMatrix, DMatrix};
pub use matrix::matrix_op::matmul::MatMul;
pub use matrix::matrix_op::lu::{LUError, SLU};
pub use matrix::matrix_op::qr::{QRError, SQR};
pub use matrix::matrix_op::schur::{SchurError, SSchur};
pub use matrix::matrix_op::eig::{EigError, SEig};
pub use vector::{DVectorConstructors, SVectorConstructors};
pub use vector::svector::{SCVector, SRVector};
pub use vector::dvector::{DCVector, DRVector};


//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]

mod tests
{

    use ctor::ctor;
    use topohedral_tracing::*;


    #[ctor]

    fn init_logger()
    {

        init().unwrap();
    }

    #[test]

    fn test_logging()
    {

        info!("Logging is working!");
    }
}

//}}}
