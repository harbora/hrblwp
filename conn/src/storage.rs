use core::net::SocketAddr;

use hrblwp_core::{ConnectionId, PeerId};

use crate::Result;

pub trait ConnectionStorage {
    fn set_peer(&mut self, peer: PeerId, addr: SocketAddr) -> Result<()>;

    fn get_addr_by_peer(&self, peer: &PeerId) -> Result<Option<SocketAddr>>;

    fn get_peer_by_addr(&self, addr: &SocketAddr) -> Result<Option<PeerId>>;

    fn set_conn(&mut self, cid: ConnectionId, sa: PeerId, da: PeerId) -> Result<()>;

    fn get_peer_by_conn(&self, cid: &ConnectionId) -> Result<Option<(PeerId, PeerId)>>;

    fn get_conn_by_peer(&self, sa: &PeerId, da: &PeerId) -> Result<Option<ConnectionId>>;
}

pub trait SecurityStorage {
    fn set_key(&mut self, cid: ConnectionId, key: &[u8]) -> Result<()>;

    fn get_key(&self, cid: &ConnectionId) -> Result<Option<&[u8]>>;
}
