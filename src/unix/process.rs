use crate::error::ShellError;
use crate::process::Process;
use crate::unix::pipeable::UnixPipeable;
use libc::STDOUT_FILENO;
use nix::sys::wait::waitpid;
use nix::unistd::{dup2, execv, fork, ForkResult, Pid};
use std::convert::Infallible;
use std::ffi::CString;

pub struct UnixProcess<'a> {
    command: String,
    argv: Vec<CString>,
    stdout_alternative: Option<&'a dyn UnixPipeable>,
}

impl<'a> UnixProcess<'a> {
    pub fn new(command: impl AsRef<str>) -> Self {
        let cmd = CString::new(command.as_ref()).unwrap();
        Self {
            command: command.as_ref().to_owned(),
            argv: vec![cmd],
            stdout_alternative: None,
        }
    }

    pub fn arg(mut self, arg: impl AsRef<str>) -> Self {
        self.argv.push(CString::new(arg.as_ref()).unwrap());
        self
    }

    pub fn pipe_output_to(&mut self, pipeable: &'a impl UnixPipeable) {
        self.stdout_alternative = Some(pipeable);
    }

    fn exec_new_process(&self) -> Result<(), ShellError<()>> {
        if let Some(pipeable) = self.stdout_alternative {
            Self::rediect_fd_to(STDOUT_FILENO, pipeable.get_fd());
        }
        let Err(_) = execv(&self.argv[0], &self.argv);
        Err(ShellError::new(self.command.clone(), ()))
    }

    fn wait_for_child(&self, pid: Pid) -> Result<(), ShellError<()>> {
        println!("Child pid: {}", pid);
        let res = waitpid(pid, None);
        println!("Child terminated: {:?}", res);
        Ok(())
    }

    fn rediect_fd_to(source: i32, target: i32) {
        dup2(target, source)
            .expect("File descriptors should be able to be duplicated at this point");
    }
}

impl<'a> Process for UnixProcess<'a> {
    fn replace(self) -> Result<Infallible, ShellError<()>> {
        let Err(_) = execv(&self.argv[0], &self.argv);
        Err(ShellError::new(self.command.clone(), ()))
    }

    fn spawn(&self) -> Result<(), ShellError<()>> {
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child }) => self.wait_for_child(child),
            Ok(ForkResult::Child) => self.exec_new_process(),
            Err(_) => Err(ShellError::new(self.command.clone(), ())),
        }
    }
}
