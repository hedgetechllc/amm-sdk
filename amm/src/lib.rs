#[cfg_attr(not(feature = "std"), no_std)]
#[macro_use]
extern crate alloc;

mod composition;
pub mod context;
pub mod modification;
pub mod note;
pub mod storage;
pub mod structure;
pub mod temporal;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub use composition::Composition;
