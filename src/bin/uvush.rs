use uvush::process::{Process, UnixProcess};
use uvush::utils::perror;

fn main() {
    let command = "/usr/bin/echo";
    let echo = UnixProcess::new(command)
        .arg("Holis, espero que andes bien. Mirame, estoy haciendo mi propia shell uvu");

    let Err(_) = echo.replace();
    perror(format!("uvush: {}", command));
}
