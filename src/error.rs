use std::fmt::{Display, Formatter};

pub struct ShellError<T> {
    source: String,
    error: T,
}

impl<T> ShellError<T> {
    pub fn new(source: impl AsRef<str>, error: T) -> Self {
        Self {
            source: source.as_ref().to_owned(),
            error,
        }
    }

    pub fn with_cause(source: impl AsRef<str>, cause: Self) -> Self {
        Self::new(
            format!("{}: {}", source.as_ref(), cause.source()),
            cause.error,
        )
    }

    fn source(&self) -> String {
        self.source.clone()
    }

    fn error(&self) -> &T {
        &self.error
    }
}

impl<T> Display for ShellError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.source())
    }
}
