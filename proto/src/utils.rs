pub trait FromU8 {
    fn from_u8(v: u8) -> Self;
}

pub trait ToU8 {
    fn to_u8(&self, v: &mut u8);
}
