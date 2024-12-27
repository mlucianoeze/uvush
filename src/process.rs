use crate::error::ShellError;
use std::convert::Infallible;

pub trait Process {
    fn replace(self) -> Result<Infallible, ShellError<()>>;
    fn spawn(&self) -> Result<(), ShellError<()>>;
}
