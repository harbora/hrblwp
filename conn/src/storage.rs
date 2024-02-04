use core::net::SocketAddr;

use crate::{ConnectionId, PeerId, Result};

pub trait Backend {
    fn set_peer(&mut self, peer: PeerId, addr: SocketAddr) -> Result<()>;

    fn get_peer(&self, peer: PeerId) -> Result<SocketAddr>;

    fn set_conn(&mut self, cid: ConnectionId, sa: PeerId, da: PeerId) -> Result<()>;

    fn get_conn(&self, cid: ConnectionId) -> Result<(PeerId, PeerId)>;
}
