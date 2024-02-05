use bit_field::BitField;
use hrblwp_core::{ConnectionId, PeerId};

use crate::{Error, Result};

pub struct ConnFrameParser<'a> {
    buff: &'a [u8],
    short: bool,
}

impl<'a> ConnFrameParser<'a> {
    pub fn uncheck_new(buff: &'a [u8]) -> Result<Self> {
        let b = buff.first().ok_or(Error::WrongLength(1))?;

        let short = b.get_bit(0);

        Ok(Self {
            buff: buff.get(1..).ok_or(Error::WrongLength(1))?,
            short,
        })
    }

    pub fn connection_id(&self) -> Result<ConnectionId> {
        Ok(ConnectionId::from_bytes(self.buff)?)
    }

    pub fn addrs(&self) -> Result<Option<(PeerId, PeerId)>> {
        if self.short {
            Ok(None)
        } else if self.buff.len() < 72 {
            Err(Error::WrongLength(72))
        } else {
            Ok(Some((
                PeerId::from_bytes(&self.buff[32..52])?,
                PeerId::from_bytes(&self.buff[52..72])?,
            )))
        }
    }

    pub fn length(&self) -> usize {
        if self.short {
            33
        } else {
            73
        }
    }
}
