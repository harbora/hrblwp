#[derive(Debug)]
pub enum Error {
    WrongLength(usize),
}

pub type Result<T> = core::result::Result<T, Error>;
