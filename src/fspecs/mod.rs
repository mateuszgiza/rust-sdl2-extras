//!
//! Extras for Specs crate (ECS framework)
//! 
//! To use this you'll need to build with the feature `fspecs`:
//! 
//! ```bash
//! $ cargo build --features "fspecs"
//! ```
//! 
//! or add the following line in Cargo.toml file inside your crate:
//! 
//! ```toml
//! [dependencies.sdl2-extras]
//! version = ...
//! default-features = false
//! features = ["fspecs"]
//! ```

mod extensions;
pub use self::extensions::WorldExt;