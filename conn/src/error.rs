use hrblwp_core::impl_from;

pub enum Error {
    WrongLength(usize),
    HrbCoreError(hrblwp_core::Error),
    ProtoError(hrblwp_proto::Error),
    SecurityError(hrblwp_security::Error),
    BackendError(u32),
}

pub type Result<T> = core::result::Result<T, Error>;

macro_rules! impl_dropped_error_from {
    ($f:ty, $v:ident) => {
        impl From<$f> for DroppedError {
            fn from(v: $f) -> Self {
                Self::Error(Error::$v(v))
            }
        }

        impl_from!(Error, $f, $v);
    };
}

pub enum DroppedError {
    Error(Error),
    Dropped(&'static str),
}

impl From<Error> for DroppedError {
    fn from(value: Error) -> Self {
        Self::Error(value)
    }
}

impl_dropped_error_from!(hrblwp_core::Error, HrbCoreError);
impl_dropped_error_from!(hrblwp_proto::Error, ProtoError);
impl_dropped_error_from!(hrblwp_security::Error, SecurityError);

pub type DroppedResult<T> = core::result::Result<T, DroppedError>;
