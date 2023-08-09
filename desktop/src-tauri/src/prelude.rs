use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub struct W<T>(pub T);
