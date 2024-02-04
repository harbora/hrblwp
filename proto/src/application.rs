use crate::{Error, Result};

pub struct ApplicationFrameParser<'a> {
    code: u8,
    protocol: u16,
    url_length: u16,
    option_number: u16,
    payload_offset: u32,
    payload_len: u32,

    buff: &'a [u8],
}

impl<'a> ApplicationFrameParser<'a> {
    pub fn uncheck_new(buff: &'a [u8]) -> Result<Self> {
        let code = *buff.get(1).ok_or(Error::WrongLength(2))?;

        let protocol = {
            let a = *buff.get(2).ok_or(Error::WrongLength(2))?;
            let b = *buff.get(3).ok_or(Error::WrongLength(3))?;

            u16::from_be_bytes([a, b])
        };

        let url_length = {
            let a = *buff.get(4).ok_or(Error::WrongLength(4))?;
            let b = *buff.get(5).ok_or(Error::WrongLength(5))?;

            u16::from_be_bytes([a, b])
        };

        let option_number = {
            let a = *buff.get(6).ok_or(Error::WrongLength(6))?;
            let b = *buff.get(7).ok_or(Error::WrongLength(7))?;

            u16::from_be_bytes([a, b])
        };

        let payload_offset = {
            let a = *buff.get(8).ok_or(Error::WrongLength(8))?;
            let b = *buff.get(9).ok_or(Error::WrongLength(9))?;
            let c = *buff.get(10).ok_or(Error::WrongLength(10))?;
            let d = *buff.get(11).ok_or(Error::WrongLength(11))?;

            u32::from_be_bytes([a, b, c, d])
        };

        let payload_len = {
            let a = *buff.get(12).ok_or(Error::WrongLength(12))?;
            let b = *buff.get(13).ok_or(Error::WrongLength(13))?;
            let c = *buff.get(14).ok_or(Error::WrongLength(14))?;
            let d = *buff.get(15).ok_or(Error::WrongLength(15))?;

            u32::from_be_bytes([a, b, c, d])
        };

        Ok(Self {
            code,
            protocol,
            url_length,
            option_number,
            payload_offset,
            payload_len,
            buff: buff.get(16..).ok_or(Error::WrongLength(16))?,
        })
    }

    pub fn code(&self) -> u8 {
        self.code
    }

    pub fn protocol(&self) -> u16 {
        self.protocol
    }

    pub fn url(&self) -> Result<&[u8]> {
        self.buff
            .get(0..self.url_length as usize)
            .ok_or(Error::WrongLength(self.url_length as usize))
    }

    pub fn options(&self) -> Result<OptionParser> {
        let buff = self
            .buff
            .get(self.url_length as usize..self.payload_offset as usize)
            .ok_or(Error::WrongLength(self.payload_offset as usize))?;

        Ok(OptionParser {
            option_number: self.option_number,
            curser: 0,
            buff,
        })
    }

    pub fn payload(&self) -> Result<&[u8]> {
        let payload_end = self.payload_len + self.payload_offset;

        self.buff
            .get(self.payload_offset as usize..payload_end as usize)
            .ok_or(Error::WrongLength(payload_end as usize))
    }
}

pub struct OptionParser<'a> {
    option_number: u16,
    curser: u16,
    buff: &'a [u8],
}

impl<'a> OptionParser<'a> {
    fn _next<'b>(&'b mut self) -> Result<(u16, &'a [u8])>
    where
        'a: 'b,
    {
        let a = *self.buff.first().ok_or(Error::WrongLength(1))?;
        let b = *self.buff.get(1).ok_or(Error::WrongLength(2))?;

        let index = u16::from_be_bytes([a, b]);

        let c = *self.buff.get(2).ok_or(Error::WrongLength(3))?;
        let d = *self.buff.get(3).ok_or(Error::WrongLength(4))?;

        let length = u16::from_be_bytes([c, d]) as usize;

        self.curser += 1;
        self.buff = self
            .buff
            .get(length + 4..)
            .ok_or(Error::WrongLength(length + 4))?;

        let res = self
            .buff
            .get(4..4 + length)
            .ok_or(Error::WrongLength(length + 4))?;

        Ok((index, res))
    }
}

impl<'a> Iterator for OptionParser<'a> {
    type Item = Result<(u16, &'a [u8])>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curser < self.option_number {
            Some(self._next())
        } else {
            None
        }
    }
}
