use crate::{utils::FromU8, DHAlgorithm, Error, Result, SecurityScheme, SpongeAlgorithm};

pub struct SecurityFrameParser<'a> {
    buff: &'a [u8],
}

impl<'a> SecurityFrameParser<'a> {
    pub fn uncheck_new(buff: &'a [u8]) -> Result<Self> {
        Ok(Self {
            buff: buff.get(1..).ok_or(Error::WrongLength(1))?,
        })
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

    pub fn payload(&self) -> Result<&[u8]> {
        self.buff.get(1..).ok_or(Error::WrongLength(3))
    }
}
