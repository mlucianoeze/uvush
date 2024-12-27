use crate::error::ShellError;
use crate::unix::pipeable::UnixPipeable;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::close;

pub struct UnixFile {
    path: String,
    fd: i32,
}

impl UnixFile {
    pub fn write(path: impl AsRef<str>) -> Result<Self, ShellError<()>> {
        let result = open(
            path.as_ref(),
            OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC,
            Mode::S_IWUSR | Mode::S_IRUSR | Mode::S_IRGRP | Mode::S_IROTH,
        );
        match result {
            Err(_) => Err(ShellError::new(path.as_ref(), ())),
            Ok(fd) => Ok(Self {
                path: path.as_ref().to_string(),
                fd,
            }),
        }
    }
}

impl UnixPipeable for UnixFile {
    fn get_fd(&self) -> i32 {
        self.fd
    }
}

impl Drop for UnixFile {
    fn drop(&mut self) {
        let result = close(self.fd);
        if let Err(errno) = result {
            println!("warning: close fd {} error: {}", self.fd, errno);
        }
    }
}
