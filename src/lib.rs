#![cfg_attr(feature = "unstable", feature(test))]
#[cfg(all(test, feature = "unstable"))]
extern crate test;

extern crate rand;
pub mod graph;
pub mod vertex;
