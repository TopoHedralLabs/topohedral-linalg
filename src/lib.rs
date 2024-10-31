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
pub use matrix::core::{SMatrix, Expression};
pub use matrix::addop::AddExpr;




//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{
    use ctor::ctor;
    use topohedral_tracing::*;
    

    #[ctor]
    fn init_logger() {
        init().unwrap();
    }

    #[test]
    fn test_logging() 
    {
        info!("Logging is working!");
    }


}
//}}}