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

//{{{ collection: private modules
mod blaslapack;
mod common;
mod expression;
//}}}
//{{{ collection: public API
pub use common::{Complex, MatrixOps};
pub mod dmatrix;
pub mod smatrix;
//}}}

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
