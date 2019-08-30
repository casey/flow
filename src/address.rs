use crate::common::*;

#[derive(Debug, PartialEq)]
pub(crate) struct Address {
    pubkey: Pubkey,
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

        let pubkey = Pubkey::from_str(&text[..idx])?;
        let addr = &text[idx + 1..];
        let ip = SocketAddr::from_str(addr).map_err(|addr_parse| Error::AddrParse {
            addr_parse,
            bad_addr: addr.to_string(),
        })?;

        Ok(Address { pubkey, ip })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_from_str() -> Result<(), Error> {
        let pk = &"deadbeef".repeat(8);
        let ip = "127.0.0.1:8080";
        let text = format!("{}@{}", pk, ip);
        let want = Address {
            pubkey: Pubkey::from_str(pk).unwrap(),
            ip: SocketAddr::from_str(ip).unwrap(),
        };
        let have = Address::from_str(&text)?;
        assert_eq!(have, want);
        Ok(())
    }

    #[test]
    fn no_at() {
        assert_eq!(
            Address::from_str("invalid"),
            Err(Error::NoAt {
                bad_addr: "invalid".to_owned(),
            })
        );
    }
}
