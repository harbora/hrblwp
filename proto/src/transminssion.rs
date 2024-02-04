use bit_field::BitField;

use crate::{utils::FromU8, Error, Result, TransmissionScheme};

pub struct TransmissionFrameParser<'a> {
    end: bool,
    flag: bool,
    buff: &'a [u8],
}

impl<'a> TransmissionFrameParser<'a> {
    pub fn uncheck_new(buff: &'a [u8]) -> Result<Self> {
        let b = buff.first().ok_or(Error::WrongLength(1))?;

        let end = b.get_bit(1);
        let flag = b.get_bit(0);

        Ok(Self {
            buff: buff.get(1..).ok_or(Error::WrongLength(1))?,
            end,
            flag,
        })
    }

    pub fn scheme(&self) -> Result<TransmissionScheme> {
        let mut byte = *self.buff.first().ok_or(Error::WrongLength(2))?;

        byte >>= 1;
        byte.set_bit(7, self.flag);

        let byte2 = self.buff.get(4).ok_or(Error::WrongLength(5))?;
        byte.set_bit(5, byte2.get_bit(7));

        Ok(TransmissionScheme::from_u8(byte))
    }

    pub fn end(&self) -> bool {
        self.end
    }

    pub fn message_id(&self) -> Result<u32> {
        let mut a = *self.buff.first().ok_or(Error::WrongLength(9))?;
        let b = *self.buff.get(1).ok_or(Error::WrongLength(9))?;
        let c = *self.buff.get(2).ok_or(Error::WrongLength(9))?;
        let d = *self.buff.get(3).ok_or(Error::WrongLength(9))?;

        a.set_bit(7, false);

        Ok(u32::from_be_bytes([a, b, c, d]))
    }

    pub fn message_no(&self) -> Result<u32> {
        let mut a = *self.buff.get(4).ok_or(Error::WrongLength(9))?;
        let b = *self.buff.get(5).ok_or(Error::WrongLength(9))?;
        let c = *self.buff.get(6).ok_or(Error::WrongLength(9))?;
        let d = *self.buff.get(7).ok_or(Error::WrongLength(9))?;

        a.set_bit(7, false);

        Ok(u32::from_be_bytes([a, b, c, d]))
    }
}
