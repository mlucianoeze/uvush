use nix::unistd::execv;
use std::convert::Infallible;
use std::ffi::CString;

pub trait Process {
    fn replace(self) -> Result<Infallible, ()>;
}

pub struct UnixProcess {
    command: CString,
    argv: Vec<CString>,
}

impl UnixProcess {
    pub fn new(command: impl AsRef<str>) -> Self {
        let command = CString::new(command.as_ref()).unwrap();
        Self {
            command: command.clone(),
            argv: vec![command],
        }
    }

    pub fn arg(mut self, arg: impl AsRef<str>) -> Self {
        self.argv.push(CString::new(arg.as_ref()).unwrap());
        self
    }
}

impl Process for UnixProcess {
    fn replace(self) -> Result<Infallible, ()> {
        let Err(_) = execv(&self.command, &self.argv);
        Err(())
    }
}
