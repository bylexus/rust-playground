use std::fmt::{Display, Formatter, Result};

pub enum LogSeverity {
    DEBUG = 1,
    INFO = 2,
    WARNING = 3,
    ERROR = 4,
    FATAL = 5,
}

impl Display for LogSeverity {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            Self::DEBUG => "DEBUG",
            Self::INFO => "INFO",
            Self::WARNING => "WARNING",
            Self::ERROR => "ERROR",
            Self::FATAL => "FATAL",
        })
    }
}