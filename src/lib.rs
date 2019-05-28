//! # Everyone's Most Loved Language* - A Gentle Introduction to Rust
//!
//! _*As voted in the Stack Overflow Development survey every year since 2015_
//!
//! ## What is Rust?
//!
//! > A language empowering everyone to build reliable and efficient software.
//!
//! The main focuses of the language are:
//!
//! * [Performance](benefits::performance)
//! * [Reliability](benefits::reliability)
//! * [Productivity](benefits::productivity)
//!
//! Let's take a deeper look at some of the [`benefits`].
//!


pub mod benefits;
pub mod syntax;
pub mod uses;

#[cfg(test)]
pub mod tests {
    #[test]
    pub fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
