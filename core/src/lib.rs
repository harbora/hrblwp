#![no_std]

pub trait StackLayer {
    type Error: core::fmt::Debug;

    fn receive(&mut self, cur: &mut CursorBuffer) -> core::result::Result<(), Self::Error>;
}

mod types;
pub use types::*;

mod error;
pub use error::*;
