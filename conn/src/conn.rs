use core::net::SocketAddr;

use hrblwp_core::{CursorBuffer, StackLayer};
use hrblwp_proto::{FrameParser, FrameType, Version};

use crate::{Backend, Error, Result};

pub const MTU_BUFFER: usize = 1450;

pub struct ConnectionLayer<B, L> {
    backend: B,
    pub buff: [u8; MTU_BUFFER],
    pub length: usize,
    layer: L,
}

impl<B, L> ConnectionLayer<B, L>
where
    B: Backend,
    L: StackLayer,
{
    pub fn receive(&mut self, from: SocketAddr) -> Result<()> {
        let buff = self
            .buff
            .get(..self.length)
            .ok_or(Error::WrongLength(self.length))?;

        let mut cb = CursorBuffer::new(buff);

        while !cb.is_empty() {
            let fp = FrameParser::new(buff)?;

            let version = fp.version();
            if version != Version::V1 {
                log::warn!("Receive unsupported version: {:?}", version);
                continue;
            }

            let ty = fp.frame_type();
            if ty == FrameType::Connection {
                // let
            } else {
                self.layer.receive(&mut cb);
            }
        }

        Ok(())
    }
}
