//! # Basic Rust Syntax
//!
//! ## Hello, World:
//!
//! See [`syntax::hello_world`]
//!
//! ```
//! fn hello_world() {
//!     println!("Hello, world!")
//! }
//!
//! hello_world();
//! ```

/// # Say hello to the world!
///
/// This is where every tutorial starts. Printing some stuff to the
/// screen.
///
/// # Example
///
/// ```
/// // Here, we are importing the "syntax" module from our
/// // introductory library and including it in the local namespace.
/// use rust_intro::syntax;
///
/// // This allows us to call functions defined in that namespace
/// syntax::a_hello_world();
/// ```
pub fn hello_world() {
    println!("Hello, world!")
}

