use bit_field::BitField;

macro_rules! impl_no_unknown {
    ($t:ty, $offset:expr, $($e:ident => $v:expr),* ) => {
        impl crate::utils::FromU8 for $t {
            fn from_u8(v: u8) -> Self {
                use bit_field::BitField;

                let bits = v.get_bits($offset);

                match bits {
                    $(
                        $v => Self::$e,
                    )*
                    _ => panic!("Got unexpected value, wrong logic!!"),
                }
            }
        }


        impl crate::utils::ToU8 for $t {
            fn to_u8(&self, v: &mut u8) {
                match self {

                    $(
                        Self::$e => {
                            v.set_bits($offset, $v);
                        }
                    )*
                }
            }
        }
    };
}

macro_rules! impl_with_unknown {
    ($t:ty, $offset:expr, $unknown:pat, $($e:ident => $v:expr),*) => {
        impl crate::utils::FromU8 for $t {
            fn from_u8(v: u8) -> Self {
                let bits = v.get_bits($offset);

                match bits {
                    $(
                        $v => Self::$e,
                    )*
                    $unknown => Self::Unknown(v),
                    _ => panic!("Got unexpected value, wrong logic!!"),
                }
            }
        }

        impl crate::utils::ToU8 for $t {
            fn to_u8(&self, v: &mut u8) {
                match self {
                    $(
                        Self::$e => { v.set_bits($offset, $v); },
                    )*

                    Self::Unknown(u) => { v.set_bits($offset, *u); },
                }
            }
        }
    };
}

pub enum FrameType {
    Connection,
    Security,
    Transmission,
    Application,
}

impl_no_unknown!(FrameType, 6..8, Connection => 0, Security => 1, Transmission => 2, Application => 3);

pub enum Version {
    V1,
    Unknown(u8),
}

impl_with_unknown!(Version, 3..6, 2..=16, V1 => 1);

pub enum SecurityScheme {
    HandshakeStart,
    HandshakeEnd,
    DataTrans,
    DataRetrans,
}

impl_no_unknown!(SecurityScheme, 6..8, HandshakeStart => 0, HandshakeEnd => 1, DataTrans => 2, DataRetrans => 3);

pub enum DHAlgorithm {
    Ed25519,
    Curve25519,
    Secp256k1,
    RSA2048,
    Unknown(u8),
}

impl_with_unknown!(DHAlgorithm, 2..6, 4..=16, Ed25519 => 0, Curve25519 => 1, Secp256k1 => 2, RSA2048 => 3);

pub enum SpongeAlgorithm {
    Keccak256,
    Aes256cbcSha256,
    Unknown(u8),
}

impl_with_unknown!(SpongeAlgorithm, 0..3, 3 | 4, Keccak256 => 0, Aes256cbcSha256 => 1);

pub enum TransmissionScheme {
    Con,
    Non,
    Ack,
    Rst,
    Stm,
    Req,
    Res,
    Unknown(u8),
}

impl_with_unknown!(TransmissionScheme, 5..8, 7, Con => 0, Non => 1, Ack => 2, Rst => 3, Stm => 4, Req => 5, Res => 6);
