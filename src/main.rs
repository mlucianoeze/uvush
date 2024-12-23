mod utils;

use crate::utils::perror;
use nix::unistd::execv;

fn main() {
    let cmd = c"/usr/bin/echo";
    let args = [
        cmd,
        c"Holis, espero que andes bien. Mirame, estoy haciendo mi propia shell uvu",
    ];
    let Err(_) = execv(&cmd, &args);

    perror(format!("uvush: {}", cmd.to_str().unwrap()));
}
