pub enum PublicKey {
    Ed25519([u8; 32]),
    Secp256k1([u8; 33]),
}

pub enum SecretKey {
    Ed25519([u8; 32]),
    Secp256k1([u8; 32]),
}
