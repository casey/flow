use crate::common::*;
use std::net::AddrParseError;

#[derive(Debug, PartialEq)]
pub(crate) enum Error {
    AddrParse { addr_parse: AddrParseError },
    NoAt { bad_addr: String },
}

impl Display for Error {
    fn fmt(&self, _f: &mut Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl From<AddrParseError> for Error {
    fn from(addr_parse: AddrParseError) -> Self {
        Error::AddrParse { addr_parse }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_addr_parse_error() {
        let ip_err = SocketAddr::from_str("invalid ip").unwrap_err();
        let error: Error = Error::from(ip_err);
        if let Error::AddrParse { .. } = error {
            return;
        }
        panic!(error);
    }
}
