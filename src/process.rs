use crate::error::ShellError;
use nix::unistd::execv;
use std::convert::Infallible;
use std::ffi::CString;

pub trait Process {
    fn replace(self) -> Result<Infallible, ShellError<()>>;
}

pub struct UnixProcess {
    command: String,
    argv: Vec<CString>,
}

impl UnixProcess {
    pub fn new(command: impl AsRef<str>) -> Self {
        let cmd = CString::new(command.as_ref()).unwrap();
        Self {
            command: command.as_ref().to_owned(),
            argv: vec![cmd],
        }
    }

    pub fn arg(mut self, arg: impl AsRef<str>) -> Self {
        self.argv.push(CString::new(arg.as_ref()).unwrap());
        self
    }
}

impl Process for UnixProcess {
    fn replace(self) -> Result<Infallible, ShellError<()>> {
        let Err(_) = execv(&self.argv[0], &self.argv);
        Err(ShellError::new(self.command.clone(), ()))
    }
}
