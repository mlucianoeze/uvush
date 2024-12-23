use uvush::error::ShellError;
use uvush::process::{Process, UnixProcess};
use uvush::utils::perror;

fn main() {
    let echo = UnixProcess::new("/usr/bin/echo")
        .arg("Holis, espero que andes bien. Mirame, estoy haciendo mi propia shell uvu");

    let Err(err) = echo.replace();
    let err = ShellError::with_cause("uvush", err);
    perror(err.to_string());
}
