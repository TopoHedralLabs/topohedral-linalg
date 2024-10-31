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

mod expr;
mod matrix;

pub use expr::Field;    
pub use matrix::SMatrix;
pub use matrix::Expression;
pub use matrix::AddExpr;




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