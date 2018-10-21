//! # Rust-SDL2-Extras
//! 
//! Additional functionalities and helpers
//! which were not included in the original 
//! crate (but were included in examples)
//! or can help with some bottlenecks 
//! and improve code readability and quality.
//! 

extern crate sdl2;

#[cfg(feature = "fspecs")]
extern crate specs;

pub mod adapters;
pub mod common;
pub mod managers;

#[cfg(feature = "fspecs")]
pub mod fspecs;