use crate::token::Token;

pub enum Error {
    #[allow(non_camel_case_types)]
    UNEXPECTED_TOKEN(Token),
    #[allow(non_camel_case_types)]
    ID_NOT_FOUND(Token),
    #[allow(non_camel_case_types)]
    DUPLICATE_ID(Token),
    #[allow(non_camel_case_types)]
    INVALID_CHARACTER(char),
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let msg = match self {
            Error::UNEXPECTED_TOKEN(token) => format!("Unexpected token: {}", token),
            Error::ID_NOT_FOUND(token) => format!("Identifier not found: {}", token),
            Error::DUPLICATE_ID(token) => format!("Duplicate id found: {}", token),
            Error::INVALID_CHARACTER(ch) => format!("Invalid character: {}", ch),
        };
        write!(fmt, "{}", msg)?;
        Ok(())
    }
}
