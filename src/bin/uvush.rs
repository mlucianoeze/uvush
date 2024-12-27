use std::process::exit;
use uvush::error::ShellError;
use uvush::process::Process;
use uvush::unix::file::UnixFile;
use uvush::unix::process::UnixProcess;
use uvush::utils::perror;

fn main() {
    let mut echo = UnixProcess::new("/usr/bin/echo")
        .arg("Me duele sentir que elegiste escapar ante seguir cultivando tan lindo amor");

    let file = UnixFile::write("/tmp/command.out");
    if let Err(err) = file {
        raise_error(err);
    }

    let file = file.unwrap();
    echo.pipe_output_to(&file);
    if let Err(err) = echo.spawn() {
        raise_error(err);
    }
}

fn raise_error(err: ShellError<()>) -> ! {
    let err = ShellError::with_cause("uvush", err);
    perror(err.to_string());
    exit(1);
}
