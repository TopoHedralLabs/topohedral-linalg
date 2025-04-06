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

pub use common::Field;
pub use matrix::smatrix::{EvaluateSMatrix, SMatrix};
pub use matrix::dmatrix::{EvaluateDMatrix, DMatrix};
pub use matrix::svector::{SCVector};
pub use matrix::matrix_op::matmul::MatMul;
pub use matrix::matrix_op::lu::{LUError, SLU};
pub use matrix::matrix_op::qr::{QRError, SQR};
pub use matrix::matrix_op::schur::{SchurError, SSchur};
pub use matrix::matrix_op::eig::{EigError, SEig};

// pub use matrix::addop::BinopExpr;


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
