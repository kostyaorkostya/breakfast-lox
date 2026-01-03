use thiserror::Error;

macro_rules! define_parse_error {
    ($name:ident, $msg:literal) => {
        #[derive(Error, Debug)]
        #[error($msg)]
        pub struct $name {
            pub token: String,
            #[source]
            pub source: anyhow::Error,
        }
    };
}

macro_rules! define_parse_error_no_source {
    ($name:ident, $msg:literal) => {
        #[derive(Error, Debug)]
        #[error($msg)]
        pub struct $name {
            pub token: String,
        }
    };
}

define_parse_error!(
    DecimalFloatingPointLiteralParseError,
    "invalid decimal floating point literal {token:?}"
);
define_parse_error!(
    HexadecimalFloatingPointLiteralParseError,
    "invalid hexadecimal floating point literal {token:?}"
);
define_parse_error!(
    IntegerLiteralParseError,
    "invalid integer literal {token:?}"
);

define_parse_error_no_source!(
    NumberIsNotFiniteParseError,
    "number is not finate (too big?) {token:?}"
);

#[derive(Error, Debug)]
#[error("invalid number literal")]
pub enum NumLitParseError {
    DecimalFloatingPointLiteral(#[from] DecimalFloatingPointLiteralParseError),
    HexadecimalFloatingPointLiteral(#[from] HexadecimalFloatingPointLiteralParseError),
    IntegerLiteral(#[from] IntegerLiteralParseError),
    NumberIsNotFinite(#[from] NumberIsNotFiniteParseError),
}

#[derive(Error, Debug)]
#[error("too many arguments; got {got}, maximum is {max}")]
pub struct TooManyArguments {
    pub got: usize,
    pub max: usize,
}

#[derive(Error, Debug)]
#[error("parsing error")]
pub enum ParseError {
    NumLit(#[from] NumLitParseError),
    TooManyArgs(#[from] TooManyArguments),
}
