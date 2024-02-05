#[macro_export]
macro_rules! impl_from {
    ($t:ty, $e:ty, $b:ident) => {
        impl From<$e> for $t {
            fn from(value: $e) -> Self {
                Self::$b(value)
            }
        }
    };
}

#[derive(Debug)]
pub enum Error {
    WrongLength(usize),
}

pub type Result<T> = core::result::Result<T, Error>;
