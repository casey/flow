use crate::common::*;
use std::net::AddrParseError;

#[derive(Debug, PartialEq)]
pub(crate) enum Error {
    AddrParse {
        addr_parse: AddrParseError,
        bad_addr: String,
    },
    NoAt {
        bad_addr: String,
    },
    PubkeyChar {
        bad_char: char,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::AddrParse {
                addr_parse,
                bad_addr,
            } => write!(f, "Invalid address, {}: {}", addr_parse, bad_addr),
            Self::NoAt { bad_addr } => write!(
                f,
                "Invalid address, expected format `PUBKEY@ADDRESS`: {}",
                bad_addr
            ),
            Self::PubkeyChar { bad_char } => {
                write!(f, "Invalid character {} in hex public key", bad_char)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addr_parse_display() {
        let err = Address::from_str("pubkey@ADDRESS").unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid address, invalid IP address syntax: ADDRESS"
        );
    }

    #[test]
    fn no_at_display() {
        let err = Error::NoAt {
            bad_addr: String::from("INVALID_ADDRESS"),
        };

        assert_eq!(
            err.to_string(),
            "Invalid address, expected format `PUBKEY@ADDRESS`: INVALID_ADDRESS"
        );
    }
}
