use crate::{Error, Result};

/// Cursor on buffer
#[derive(Debug)]
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
