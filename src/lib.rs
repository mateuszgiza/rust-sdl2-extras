extern crate sdl2;

#[cfg(feature = "fspecs")]
extern crate specs;

pub mod adapters;
pub mod common;
pub mod managers;

#[cfg(feature = "fspecs")]
pub mod fspecs;