use hrblwp_proto::SpongeAlgorithm;
use strobe_rs::{SecParam, Strobe};

use crate::{Result, SecurityConfig};

pub struct SecurityLayer {
    pub keccak256: bool,
    pub aes_sha: bool,

    strobe: Strobe,
}

pub const SECURITY_PROTO: &[u8] = b"Harbora Lightweight Protocol V0.1";

impl SecurityLayer {
    pub fn new(cfg: &SecurityConfig) -> Self {
        let strobe = Strobe::new(SECURITY_PROTO, SecParam::B256);

        Self {
            keccak256: cfg.keccak256,
            aes_sha: false,
            strobe,
        }
    }

    pub fn ad(&mut self, data: &[u8], _sponge: &SpongeAlgorithm) {
        let length = data.len() as u32;
        let l = length.to_be_bytes();

        self.strobe.meta_key(&l, false);
        self.strobe.key(data, false);
    }

    pub fn key(&mut self, key: &[u8], _sponge: &SpongeAlgorithm) {
        let length = key.len() as u32;
        let l = length.to_be_bytes();

        self.strobe.meta_key(&l, false);
        self.strobe.key(key, false);
    }

    pub fn recv_enc(&mut self, data: &mut [u8], _sponge: &SpongeAlgorithm) {
        let length = data.len() as u32;
        let mut l = length.to_be_bytes();

        self.strobe.meta_recv_enc(&mut l, false);
        self.strobe.recv_enc(data, false);
    }

    pub fn recv_mac(&mut self, data: &mut [u8]) -> Result<()> {
        self.strobe.recv_mac(data)?;
        Ok(())
    }
}
