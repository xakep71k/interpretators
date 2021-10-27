use crate::token::Token;

pub enum Error {
    #[allow(non_camel_case_types)]
    UNEXPECTED_TOKEN(Token),
    #[allow(non_camel_case_types)]
    ID_NOT_FOUND(String),
    #[allow(non_camel_case_types)]
    DUPLICATE_ID(String),
    #[allow(non_camel_case_types)]
    INVALID_CHARACTER(char),
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let msg = match self {
            Error::UNEXPECTED_TOKEN(token) => format!("Unexpected token: {}", token),
            Error::ID_NOT_FOUND(id) => format!("Identifier not found: {}", id),
            Error::DUPLICATE_ID(id) => format!("Duplicate id found: {}", id),
            Error::INVALID_CHARACTER(ch) => format!("Invalid character: {}", ch),
        };
        write!(fmt, "{}", msg)?;
        Ok(())
    }
}
