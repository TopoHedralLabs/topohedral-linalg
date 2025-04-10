//! This module implements all of the supported element-wise matrix operations.
//!
//! Operations include:
//!
//! - Addition [`addop`]
//! - Subtraction [`subop`]
//! - Multiplication [`mulop`]
//! - Division [`divop`]
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

mod addop;
mod common;
mod divop;
mod mulop;
mod negop;
mod subop;

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]

mod tests
{
}

//}}}
