use bit_field::BitField;

use crate::{Error, Result};

pub struct ConnFrameParser<'a> {
    buff: &'a [u8],
    short: bool,
}

impl<'a> ConnFrameParser<'a> {
    pub fn uncheck_new(buff: &'a [u8]) -> Result<Self> {
        let b = buff.first().ok_or(Error::WrongLength(1))?;

        let short = b.get_bit(7);

        Ok(Self {
            buff: buff.get(1..).ok_or(Error::WrongLength(1))?,
            short,
        })
    }

    pub fn cid(&self) -> Result<&[u8]> {
        self.buff.get(0..32).ok_or(Error::WrongLength(33))
    }

    pub fn sa(&self) -> Result<Option<&[u8]>> {
        if self.short {
            Ok(None)
        } else {
            Ok(Some(self.buff.get(32..52).ok_or(Error::WrongLength(53))?))
        }
    }

    pub fn da(&self) -> Result<Option<&[u8]>> {
        if self.short {
            Ok(None)
        } else {
            Ok(Some(self.buff.get(52..72).ok_or(Error::WrongLength(73))?))
        }
    }
}
