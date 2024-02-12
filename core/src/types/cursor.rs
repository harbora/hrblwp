use crate::Result;

/// Cursor on buffer
#[derive(Debug)]
pub struct Buffer<const N: usize> {
    buff: [u8; N],
    pos: usize,
    length: usize,
}

impl<const N: usize> Default for Buffer<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Buffer<N> {
    pub fn new() -> Self {
        Self {
            buff: [0u8; N],
            pos: 0,
            length: 0,
        }
    }

    pub fn init(&mut self, length: usize) {
        self.length = length;
        self.pos = 0;
    }

    pub fn raw_buffer(&self) -> &[u8] {
        &self.buff
    }

    pub fn raw_buffer_mut(&mut self) -> &mut [u8] {
        &mut self.buff
    }

    pub fn buffer(&self) -> &[u8] {
        &self.buff[self.pos..self.length]
    }

    pub fn buffer_mut(&mut self) -> &mut [u8] {
        &mut self.buff[self.pos..self.length]
    }

    pub fn advance(&mut self, step: usize) -> Result<()> {
        // TODO: Check pos over length?
        self.pos += step;

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.buff.is_empty()
    }
}
