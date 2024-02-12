use k256::elliptic_curve::group::GroupEncoding;

use crate::{Error, Result, SharedKey};

pub enum PublicKey {
    Ed25519([u8; 32]),
    Secp256k1([u8; 33]),
}

pub enum SecretKey {
    Ed25519([u8; 32]),
    Secp256k1([u8; 32]),
}

pub fn diffle_hellman(pk: PublicKey, sk: SecretKey) -> Result<SharedKey> {
    match (pk, sk) {
        (PublicKey::Ed25519(pk), SecretKey::Ed25519(sk)) => {
            let scalar = curve25519_dalek::Scalar::from_canonical_bytes(sk).expect("Wrong key");
            let point = curve25519_dalek::MontgomeryPoint(pk);

            let sss = scalar * point;

            Ok(SharedKey(sss.to_bytes()))
        }
        (PublicKey::Secp256k1(pk), SecretKey::Secp256k1(sk)) => {
            let sk = k256::SecretKey::from_slice(&sk)?.to_nonzero_scalar();
            let ap = k256::AffinePoint::from_bytes(&(pk).into()).expect("Failed to decode");

            let sss = k256::ecdh::diffie_hellman(sk, ap);

            Ok(SharedKey::from_bytes(sss.raw_secret_bytes().as_ref())?)
        }
        _ => Err(Error::MismatchKeyType),
    }
}
