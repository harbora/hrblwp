use hrblwp_core::impl_from;

#[derive(Debug)]
pub enum Error {
    HrbCoreError(hrblwp_core::Error),
    WrongLength(usize),
}

impl_from!(Error, hrblwp_core::Error, HrbCoreError);

pub type Result<T> = core::result::Result<T, Error>;
