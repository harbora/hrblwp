use crate::{utils::FromU8, Error, FrameType, Result, Version};

pub struct FrameParser {
    byte: u8,
}

impl FrameParser {
    pub fn new(buff: &[u8]) -> Result<FrameParser> {
        let byte = *buff.first().ok_or(Error::WrongLength(1))?;

        Ok(FrameParser { byte })
    }

    pub fn frame_type(&self) -> FrameType {
        FrameType::from_u8(self.byte)
    }

    pub fn version(&self) -> Version {
        Version::from_u8(self.byte)
    }
}
