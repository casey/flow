use crate::common::*;

#[derive(Debug, PartialEq)]
pub(crate) struct Address {
    pubkey: String,
    ip: SocketAddr,
}

impl FromStr for Address {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let idx = match text.find('@') {
            Some(i) => i,
            None => {
                return Err(Error::NoAt {
                    bad_addr: text.to_owned(),
                })
            }
        };
        let pubkey = text[..idx].to_owned();
        let ip = SocketAddr::from_str(&text[idx + 1..])?;
        Ok(Address { pubkey, ip })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_good() -> Result<(), Error> {
        let want = Address {
            pubkey: String::from("pub"),
            ip: SocketAddr::from_str("127.0.0.1:8080")?,
        };
        let my_str = "pub@127.0.0.1:8080";
        let have = Address::from_str(my_str)?;
        assert_eq!(have, want);
        Ok(())
    }

    #[test]
    fn no_at() {
        match Address::from_str("invalid") {
            Err(Error::NoAt { bad_addr }) => assert_eq!(bad_addr, "invalid"),
            other => panic!(other),
        }
    }
}
