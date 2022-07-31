//! # test-macros
//!
//! Helper macros for righting clean tests
//!
//!
//! ## Example
//!
//! ```rust
//! use test_macros;
//!
//! fn setup_db() {
//!     // Create test data, load env etc.
//! }
//!
//! fn cleanup_db() {
//!     // Delete data from tables, close connections etc.
//! }
//!
//! #[test]
//! #[before(call = "setup_db")]
//! #[after("cleanup_db")]
//! fn some_test() {
//!     // … your stuff goes here …
//! }
//! ```
//!

pub(crate) mod expand;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, AttributeArgs, ItemFn};
use crate::expand::{FnTemplate, parse_args, get_altered_fn};

/// ## Call a function **BEFORE** a test has run. Useful for e.g. DB setup
///
/// ### Example
///
/// ```rust
/// fn setup_db() -> &'static str {
///     // Create test data, load env etc.
///
///     "Return value, stored in variable '_before_values'"
/// }
///
/// #[test]
/// #[before(call = "setup_db")]
/// fn some_test() {
///     // … your stuff goes here …
/// }
/// ```
///
/// ### Variable '_before\_values'
///
/// As shown in the _setup_db_ fn example, the return value of the _before_ fn can be used inside
/// the test by using the variable name __before_values_
///
/// ```rust
/// fn setup_db() -> &'static str {
///     // Create test data, load env etc.
///
///     "Return value!"
/// }
///
/// #[test]
/// #[before(call = "setup_db")]
/// fn some_test() {
///     assert_eq!(_before_values, "Return value!") // Asserts positively
/// }
/// ```
///
#[proc_macro_attribute]
pub fn before(args: TokenStream, input: TokenStream) -> TokenStream {
    apply_fn(args, input, FnTemplate::Before)
}

/// ## Call a function **AFTER** a test has run. Useful for e.g. DB cleanup
///
/// ### Example
///
/// ```rust
/// fn cleanup_db() {
///     // Delete data from tables, close connections etc.
/// }
///
/// #[test]
/// #[after(call = "cleanup_db")]
/// fn some_test() {
///     // … your stuff goes here …
/// }
/// ```
#[proc_macro_attribute]
pub fn after(args: TokenStream, input: TokenStream) -> TokenStream {
    apply_fn(args, input, FnTemplate::After)
}

fn apply_fn(args: TokenStream, input: TokenStream, template: FnTemplate) -> TokenStream {
    let parsed_args = parse_args(parse_macro_input!(args as AttributeArgs));
    let func = parse_macro_input!(input as ItemFn);

    get_altered_fn(func, parsed_args, template).into_token_stream().into()
}