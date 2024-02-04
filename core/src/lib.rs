#![no_std]

pub struct CursorBuffer<'a> {
    buff: &'a [u8],
}

impl<'a> CursorBuffer<'a> {
    pub fn new(buff: &'a [u8]) -> Self {
        Self { buff }
    }

    pub fn buffer(&self) -> &[u8] {
        self.buff
    }

    pub fn advance(&mut self, step: usize) -> Result<()> {
        self.buff = &self.buff.get(step..).ok_or(Error::WrongLength(step))?;

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.buff.is_empty()
    }
}

pub trait StackLayer {
    type Error: core::fmt::Debug;

    fn receive(&mut self, cur: CursorBuffer) -> core::result::Result<(), Self::Error>;
}

#[derive(Debug)]
pub enum Error {
    WrongLength(usize),
}

pub type Result<T> = core::result::Result<T, Error>;
