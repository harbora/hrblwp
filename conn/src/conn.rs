use core::net::SocketAddr;

use hrblwp_core::{ConnectionId, CursorBuffer, PeerId};
use hrblwp_proto::{ConnFrameParser, FrameParser, FrameType, SecurityFrameParser, Version};
use hrblwp_security::SecurityLayer;

use crate::{ConnectionStorage, DroppedError, DroppedResult, Error, Result, SecurityStorage};

pub const MTU_BUFFER: usize = 1450;

pub struct ConnectionLayer<B, S> {
    backend: B,
    pub buff: [u8; MTU_BUFFER],
    pub length: usize,
    pub layer: SecurityLayer,
    security_storage: S,
}

#[derive(Debug, PartialEq, Eq)]
enum ConnFrameStatus {
    Transmit,
    PathMigrate,
    Handshake,
}

macro_rules! dropped {
    ($v:expr) => {
        match $v {
            Ok(v) => v,
            Err(DroppedError::Dropped(m)) => {
                log::debug!("{}", m);
                return Ok(());
            }
            Err(DroppedError::Error(e)) => return Err(e),
        }
    };
}

impl<B, S> ConnectionLayer<B, S>
where
    B: ConnectionStorage,
    S: SecurityStorage,
{
    pub fn receive(&mut self, from: SocketAddr, to: SocketAddr) -> Result<()> {
        let buff = self
            .buff
            .get(..self.length)
            .ok_or(Error::WrongLength(self.length))?;

        let mut cb = CursorBuffer::new(buff);

        let fp = dropped!(self.check_version(&cb));

        let res = self.handle_connection(&fp, &mut cb, &from, &to);
        let (st, cid, sa, da) = dropped!(res);

        let res = self.handle_security(&mut cb, &st, &cid);
        dropped!(res);

        Ok(())
    }

    fn check_version(&self, cb: &CursorBuffer) -> DroppedResult<FrameType> {
        let fp = FrameParser::new(cb.buffer())?;

        let version = fp.version();
        if version != Version::V1 {
            Err(DroppedError::Dropped("Unsupported version"))
        } else {
            Ok(fp.frame_type())
        }
    }

    fn handle_connection(
        &self,
        fp: &FrameType,
        cb: &mut CursorBuffer,
        from: &SocketAddr,
        to: &SocketAddr,
    ) -> DroppedResult<(ConnFrameStatus, ConnectionId, PeerId, PeerId)> {
        if fp == &FrameType::Connection {
            let cp = ConnFrameParser::uncheck_new(cb.buffer())?;

            let cid = cp.connection_id()?;

            let (sa, da, st) = if let Some(a) = cp.addrs()? {
                (a.0, a.1, ConnFrameStatus::Handshake)
            } else {
                let a = self
                    .backend
                    .get_peer_by_conn(&cid)?
                    .ok_or(DroppedError::Dropped("No peer id found, drop it"))?;

                (a.0, a.1, ConnFrameStatus::PathMigrate)
            };

            cb.advance(cp.length())?;

            Ok((st, cid, sa, da))
        } else {
            // Get PeerID and ConnectionID based on sa and da.
            let sa = self
                .backend
                .get_peer_by_addr(from)?
                .ok_or(DroppedError::Dropped("No peer id found, drop it"))?;
            let da = self
                .backend
                .get_peer_by_addr(to)?
                .ok_or(DroppedError::Dropped("No peer id found, drop it"))?;

            let cid = self
                .backend
                .get_conn_by_peer(&sa, &da)?
                .ok_or(DroppedError::Dropped("No connection id found, drop it"))?;

            Ok((ConnFrameStatus::Transmit, cid, sa, da))
        }
    }

    fn handle_security(
        &self,
        cb: &mut CursorBuffer,
        status: &ConnFrameStatus,
        cid: &ConnectionId,
    ) -> DroppedResult<()> {
        let sp = SecurityFrameParser::uncheck_new(cb.buffer())?;

        if (status == &ConnFrameStatus::Transmit || status == &ConnFrameStatus::PathMigrate)
            && sp.transmit()
        {
            // Get transmit key with cid
            let key = self
                .security_storage
                .get_key(cid)?
                .ok_or(DroppedError::Dropped("No key found"))?;

            // Compute packet key.
            // Decrypt all data.
            // modify new frame
        } else {
            return Err(DroppedError::Dropped(
                "Security Frame not match Connection Frame",
            ));
        }

        Ok(())
    }
}
