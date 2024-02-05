use seq_macro::seq;

use crate::{Error, Result};

macro_rules! impl_bytes {
    ($t:ty, $len:literal) => {
        impl $t {
            pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
                if bytes.len() < $len {
                    Err(Error::WrongLength($len))
                } else {
                    let res = seq!(N in 0..$len {
                        [
                            #(bytes[N],)*
                        ]
                    });

                    Ok(Self(res))
                }
            }
        }

        impl AsRef<[u8; $len]> for $t {
            fn as_ref(&self) -> &[u8; $len] {
                &self.0
            }
        }
    };
}

/// Peer ID
///
/// 20bytes length
pub struct PeerId([u8; 20]);

impl_bytes!(PeerId, 20);

/// Connection ID
///
/// 32bytes length
pub struct ConnectionId(pub [u8; 32]);

impl_bytes!(ConnectionId, 32);
