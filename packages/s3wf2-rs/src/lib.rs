//! The Rust implementation of ShortStoryServer Writer's Format v2 (S3WF2).

#![doc(html_logo_url = "https://imgur.com/ULId5MF.png")]

pub mod document;
pub mod emitter;
pub mod error;
pub mod parser;

#[cfg(feature = "foreign")]
pub mod foreign;

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(feature = "wasm")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
