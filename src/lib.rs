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
//! * Performance
//! * Reliability
//! * Productivity
//!
//! ### Performance
//!
//! Well-written Rust programs are consistently just as fast or faster
//! than equivalent C code. **Rust beats C** in 5/10 of the benchmarks
//! in the [Benchmarks Game](https://benchmarksgame-team.pages.debian.net/benchmarksgame/faster/rust.html).
//! It also beats C++ in 5/10, and beats Go and Java in 10/10.
//!
//! ### Reliability
//!
//! Rust provides some of the strongest compile-time guarantees of any
//! language, using a "borrow-checker" to guarantee that all memory usage is
//! safe, even across threads (no user-after-free, no out-of-bounds arrays,
//! no null pointer exceptions, etc.). It also has a strong and consistent
//! type system that ensures your programs function as intended.
//!
//! ### Productivity
//!
//! Want to read the documentation for the standard library on an airplane?
//! No problem! Just run `rustup docs`. The entire Rust book (which goes
//! through learning Rust step by step) and the stdlib docs are available
//! offline.
//!
//! Rust also has an amazing package/build manager (Cargo), IDE support via
//! the Rust Language Server, a builtin autoformatter, and lots of other
//! features to make developers' lives easy.
//!
//! It's even got builtin cross-compilation support for all of the following
//! targets (as of 2019-05-15):
//!
//! * aarch64-apple-ios
//! * aarch64-fuchsia
//! * aarch64-linux-android
//! * aarch64-pc-windows-msvc
//! * aarch64-unknown-cloudabi
//! * aarch64-unknown-freebsd
//! * aarch64-unknown-hermit
//! * aarch64-unknown-linux-gnu
//! * aarch64-unknown-linux-musl
//! * aarch64-unknown-netbsd
//! * aarch64-unknown-none
//! * aarch64-unknown-openbsd
//! * arm-linux-androideabi
//! * arm-unknown-linux-gnueabi
//! * arm-unknown-linux-gnueabihf
//! * arm-unknown-linux-musleabi
//! * arm-unknown-linux-musleabihf
//! * armebv7r-none-eabi
//! * armebv7r-none-eabihf
//! * armv4t-unknown-linux-gnueabi
//! * armv5te-unknown-linux-gnueabi
//! * armv5te-unknown-linux-musleabi
//! * armv6-unknown-netbsd-eabihf
//! * armv7-apple-ios
//! * armv7-linux-androideabi
//! * armv7-unknown-cloudabi-eabihf
//! * armv7-unknown-linux-gnueabihf
//! * armv7-unknown-linux-musleabihf
//! * armv7-unknown-netbsd-eabihf
//! * armv7r-none-eabi
//! * armv7r-none-eabihf
//! * armv7s-apple-ios
//! * asmjs-unknown-emscripten
//! * i386-apple-ios
//! * i586-pc-windows-msvc
//! * i586-unknown-linux-gnu
//! * i586-unknown-linux-musl
//! * i686-apple-darwin
//! * i686-linux-android
//! * i686-pc-windows-gnu
//! * i686-pc-windows-msvc
//! * i686-unknown-cloudabi
//! * i686-unknown-dragonfly
//! * i686-unknown-freebsd
//! * i686-unknown-haiku
//! * i686-unknown-linux-gnu
//! * i686-unknown-linux-musl
//! * i686-unknown-netbsd
//! * i686-unknown-openbsd
//! * mips-unknown-linux-gnu
//! * mips-unknown-linux-musl
//! * mips-unknown-linux-uclibc
//! * mips64-unknown-linux-gnuabi64
//! * mips64el-unknown-linux-gnuabi64
//! * mipsel-unknown-linux-gnu
//! * mipsel-unknown-linux-musl
//! * mipsel-unknown-linux-uclibc
//! * msp430-none-elf
//! * nvptx64-nvidia-cuda
//! * powerpc-unknown-linux-gnu
//! * powerpc-unknown-linux-gnuspe
//! * powerpc-unknown-linux-musl
//! * powerpc-unknown-netbsd
//! * powerpc64-unknown-freebsd
//! * powerpc64-unknown-linux-gnu
//! * powerpc64-unknown-linux-musl
//! * powerpc64le-unknown-linux-gnu
//! * powerpc64le-unknown-linux-musl
//! * riscv32imac-unknown-none-elf
//! * riscv32imc-unknown-none-elf
//! * riscv64gc-unknown-none-elf
//! * riscv64imac-unknown-none-elf
//! * s390x-unknown-linux-gnu
//! * sparc-unknown-linux-gnu
//! * sparc64-unknown-linux-gnu
//! * sparc64-unknown-netbsd
//! * sparcv9-sun-solaris
//! * thumbv6m-none-eabi
//! * thumbv7a-pc-windows-msvc
//! * thumbv7em-none-eabi
//! * thumbv7em-none-eabihf
//! * thumbv7m-none-eabi
//! * thumbv7neon-linux-androideabi
//! * thumbv7neon-unknown-linux-gnueabihf
//! * thumbv8m.base-none-eabi
//! * thumbv8m.main-none-eabi
//! * thumbv8m.main-none-eabihf
//! * wasm32-experimental-emscripten
//! * wasm32-unknown-emscripten
//! * wasm32-unknown-unknown
//! * x86_64-apple-darwin
//! * x86_64-apple-ios
//! * x86_64-fortanix-unknown-sgx
//! * x86_64-fuchsia
//! * x86_64-linux-android
//! * x86_64-pc-windows-gnu
//! * x86_64-pc-windows-msvc
//! * x86_64-rumprun-netbsd
//! * x86_64-sun-solaris
//! * x86_64-unknown-bitrig
//! * x86_64-unknown-cloudabi
//! * x86_64-unknown-dragonfly
//! * x86_64-unknown-freebsd
//! * x86_64-unknown-haiku
//! * x86_64-unknown-hermit
//! * x86_64-unknown-l4re-uclibc
//! * x86_64-unknown-linux-gnu
//! * x86_64-unknown-linux-gnux32
//! * x86_64-unknown-linux-musl
//! * x86_64-unknown-netbsd
//! * x86_64-unknown-openbsd
//! * x86_64-unknown-redox
//! * x86_64-unknown-uefi
//!
//! ## What is it good for?
//!
//! Due to Rust's strong ecosystem and active community, it's easy to
//! do all kinds of cool things with Rust:
//!
//! * Commandline tools
//!   * See for example ripgrep (`brew install ripgrep`), which beats
//!     GNU grep and every other commandline search tool in pretty much
//!     every benchmark, while supporting regex and unicode out of the
//!     box
//! * WebAssembly
//!   * It's easy to implement performance-critical functionality in the
//!     browser by writing in Rust and targeting WebAssembly, creating
//!     modules that can be published to `npm` and called straight from JS
//! * Networking
//!   * Rust has mature, fully functional networking libraries for HTTP,
//!     websockets, and most other protocols. It's also got great database
//!     suppore, and Rust webservers regularly [beat industry standards by a factor of more than 30](https://github.com/the-benchmarker/web-frameworks)
//! * Embedded
//!   * For low-level code running in systems with extreme resource
//!     constraints, Rust can be built without the standard library,
//!     so you can just include what is absolutely required.
//!




/// # Basic Rust Syntax
///
/// ## Hello, World:
///
/// See [`syntax::hello_world`]
///
/// ```
/// fn hello_world() {
///     println!("Hello, world!")
/// }
///
/// hello_world();
/// ```
pub mod syntax {

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


}


#[cfg(test)]
pub mod tests {
    #[test]
    pub fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
