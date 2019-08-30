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
    PubkeyLength {
        bad_length: usize,
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
            Self::PubkeyLength { bad_length } => write!(
                f,
                "Invalid pubkey hex length {}, expected {} characters",
                bad_length,
                Pubkey::BYTE_LEN * 2
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addr_parse_display() {
        let pk = &"deadbeef".repeat(8);
        let ip = "BAD_ADDRESS";
        let text = format!("{}@{}", pk, ip);
        let err = Address::from_str(&text).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid address, invalid IP address syntax: BAD_ADDRESS"
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

    #[test]
    fn pubkey_bad_length_display() {
        let bad_pk = "deadbeef";
        let err = Pubkey::from_str(bad_pk).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid pubkey hex length 8, expected 64 characters"
        );
    }
}
