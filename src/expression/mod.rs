//! Expression-template infrastructure for lazy, allocation-free arithmetic.
//!
//! Re-exports the two sub-modules that together implement lazy evaluation for matrix arithmetic:
//! [`binary_expr`] for operations between two operands (addition, subtraction, element-wise
//! multiplication/division) and [`unary_expr`] for single-operand transformations (negation).
//! Results are not evaluated until the expression is converted into a concrete matrix type,
//! letting the compiler fuse chains of operations into a single pass without intermediate heap
//! allocations.
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

pub mod binary_expr;
pub mod boolean_expr;
pub mod comparison_expr;
pub mod outer_product_expr;
pub mod unary_expr;
