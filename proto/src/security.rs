use bit_field::BitField;
use hrblwp_core::HMAC;

use crate::{utils::FromU8, DHAlgorithm, Error, Result, SecurityScheme, SpongeAlgorithm};

pub struct SecurityFrameParser<'a> {
    transmit: bool,
    buff: &'a [u8],
}

impl<'a> SecurityFrameParser<'a> {
    pub fn uncheck_new(buff: &'a [u8]) -> Result<Self> {
        let b = buff.first().ok_or(Error::WrongLength(1))?;

        let transmit = b.get_bit(0);

        Ok(Self {
            buff: buff.get(1..).ok_or(Error::WrongLength(1))?,
            transmit,
        })
    }

    pub fn transmit(&self) -> bool {
        self.transmit
    }

    pub fn index(&self) -> Result<&[u8]> {
        self.buff.get(0..8).ok_or(Error::WrongLength(8))
    }

    pub fn scheme(&self) -> Result<SecurityScheme> {
        let byte = self.buff.first().ok_or(Error::WrongLength(2))?;

        Ok(SecurityScheme::from_u8(*byte))
    }

    pub fn dh_algorithm(&self) -> Result<DHAlgorithm> {
        let byte = self.buff.first().ok_or(Error::WrongLength(2))?;

        Ok(DHAlgorithm::from_u8(*byte))
    }

    pub fn sponge_algorithm(&self) -> Result<SpongeAlgorithm> {
        let byte = self.buff.first().ok_or(Error::WrongLength(2))?;

        Ok(SpongeAlgorithm::from_u8(*byte))
    }

    pub fn hmac(&self) -> Result<HMAC> {
        if self.transmit {
            let bytes = self.buff.get(8..40).ok_or(Error::WrongLength(40))?;

            Ok(HMAC::from_bytes(bytes)?)
        } else {
            let bytes = self.buff.get(1..33).ok_or(Error::WrongLength(33))?;

            Ok(HMAC::from_bytes(bytes)?)
        }
    }

    pub fn payload(&self) -> Result<&[u8]> {
        self.buff.get(33..).ok_or(Error::WrongLength(3))
    }
}
