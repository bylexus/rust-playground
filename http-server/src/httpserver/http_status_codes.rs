pub enum HTTPStatusCode {
    Info(usize),
	Success(usize),
	Redirect(usize),
	ClientError(usize),
	ServerError(usize),
}

impl HTTPStatusCode {
	pub fn code(&self) -> usize {
		match self {
			Self::Info(code) => *code,
			Self::Success(code) => *code,
			Self::Redirect(code) => *code,
			Self::ClientError(code) => *code,
			Self::ServerError(code) => *code,
		}
	}

    pub fn message(&self) -> &'static str {
        match self {
            Self::Info(100) => "Continue",
			Self::Success(200) => "Success",
			Self::ClientError(400) => "Bad Request",
			Self::ClientError(401) => "Unauthorized",
			Self::ClientError(402) => "Payment Required",
			Self::ClientError(403) => "Forbidden",
			Self::ClientError(404) => "Not Found",
			Self::ClientError(413) => "Payload Too Large",

            _ => "Unknown Error",
        }
    }
}
