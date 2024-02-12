use core::net::SocketAddr;

use hrblwp_core::{Buffer, ConnectionId, PeerId};
use hrblwp_proto::{ConnFrameParser, FrameParser, FrameType, SecurityFrameParser, Version};

use crate::{
    Config, ConnectionStorage, DroppedError, DroppedResult, Result, SecurityLayer, SecurityStorage,
};

pub const MTU_BUFFER: usize = 1450;

pub struct ConnectionLayer<B, S> {
    backend: B,
    security_storage: S,

    buff: Buffer<MTU_BUFFER>,
    conn_buff: usize,

    cfg: Config,
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
    pub fn receive(&mut self, length: usize, from: SocketAddr, to: SocketAddr) -> Result<()> {
        self.buff.init(length);

        let mut security = SecurityLayer::new(&self.cfg.security);

        // handle connection frame
        let fp = dropped!(self.check_version());

        let res = self.handle_connection(&fp, &from, &to);
        let (st, cid, sa, da) = dropped!(res);

        // handle security
        let fp = dropped!(self.check_version());

        let res = self.handle_security(fp, &st, &cid, &mut security);
        dropped!(res);

        // store peer info.

        Ok(())
    }

    fn check_version(&self) -> DroppedResult<FrameType> {
        let fp = FrameParser::new(self.buff.buffer())?;

        let version = fp.version();
        if version != Version::V1 {
            Err(DroppedError::Dropped("Unsupported version"))
        } else {
            Ok(fp.frame_type())
        }
    }

    fn handle_connection(
        &mut self,
        fp: &FrameType,
        from: &SocketAddr,
        to: &SocketAddr,
    ) -> DroppedResult<(ConnFrameStatus, ConnectionId, PeerId, PeerId)> {
        if fp == &FrameType::Connection {
            let cp = ConnFrameParser::uncheck_new(self.buff.buffer())?;

            let cid = cp.connection_id()?;

            let (sa, da, st, step) = if let Some(a) = cp.addrs()? {
                (a.0, a.1, ConnFrameStatus::Handshake, 73)
            } else {
                let a = self
                    .backend
                    .get_peer_by_conn(&cid)?
                    .ok_or(DroppedError::Dropped("No peer id found, drop it"))?;

                (a.0, a.1, ConnFrameStatus::PathMigrate, 33)
            };

            self.conn_buff = step;
            self.buff.advance(step)?;

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
        &mut self,
        fp: FrameType,
        status: &ConnFrameStatus,
        cid: &ConnectionId,
        sl: &mut SecurityLayer,
    ) -> DroppedResult<()> {
        if fp != FrameType::Security {
            return Err(DroppedError::Dropped("Must be security frame"));
        }

        let sp = SecurityFrameParser::uncheck_new(self.buff.buffer())?;

        if (status == &ConnFrameStatus::Transmit || status == &ConnFrameStatus::PathMigrate)
            && sp.transmit()
        {
            // Get transmit key with cid
            let (sponge, key) = self
                .security_storage
                .get_key(cid)?
                .ok_or(DroppedError::Dropped("No key found"))?;

            let index = sp.index()?;

            // Compute packet key.
            if self.conn_buff != 0 {
                sl.ad(&self.buff.raw_buffer()[..self.conn_buff + 1], &sponge);
            }

            // Set key into strobe.
            sl.key(key, &sponge);
            sl.key(index, &sponge);

            let mut hmac = sp.hmac()?;

            // Decrypt all data.
            sl.recv_enc(self.buff.buffer_mut(), &sponge);

            // Auth code
            sl.recv_mac(hmac.as_mut())?;
        } else if status == &ConnFrameStatus::Handshake && !sp.transmit() {
            let security_scheme = sp.scheme()?;
            let dh_algo = sp.dh_algorithm()?;
        } else {
            return Err(DroppedError::Dropped(
                "Security Frame not match Connection Frame",
            ));
        }

        Ok(())
    }
}
