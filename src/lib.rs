//! **Radis** is a minimal Redis client library affected by a severe form of *Primitive Type Obsession* desease.
//!
//! Designed to be used in async contexts, and with a highly opinionated
#![allow(clippy::pedantic)]
mod cmd;
mod frame;
mod resp_stream;
mod response;

pub use cmd::*;
pub use resp_stream::*;
