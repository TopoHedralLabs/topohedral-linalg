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

mod blaslapack; 
mod common;
pub use common::{Field, Complex, Float, Zero, One, IndexValue, MatrixOps};
mod expression;
pub mod smatrix;


// mod matrix;
// mod vector;


// pub use matrix::{DMatrixConstructors};
// pub use smatrix::{SMatrix };
// pub use matrix::dmatrix::{EvaluateDMatrix, DMatrix};
// pub use vector::{DVectorConstructors};
// pub use vector::svector::{SCVector, SRVector};
// pub use vector::dvector::{DCVector, DRVector};


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
