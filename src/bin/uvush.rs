use uvush::error::ShellError;
use uvush::process::{Process, UnixProcess};
use uvush::utils::perror;

fn main() {
    let echo = UnixProcess::new("/usr/bin/echo")
        .arg("Te fuiste corriendo, y me duele mucho ver lo fácil que fue para vos");

    if let Err(err) = echo.spawn() {
        let err = ShellError::with_cause("uvush", err);
        perror(err.to_string());
    }
}
