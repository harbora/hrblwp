pub enum Error {
    WrongLength(usize),
    ProtoError(hrblwp_proto::Error),
    BackendError(u32),
}

impl From<hrblwp_proto::Error> for Error {
    fn from(value: hrblwp_proto::Error) -> Self {
        Self::ProtoError(value)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
