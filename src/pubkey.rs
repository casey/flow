use crate::common::*;

pub(crate) struct Pubkey {
    bytes: [u8; 32],
}

fn char_to_u8(c: char) -> Result<u8, Error> {
    match c {
        '0' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        'A' | 'a' => Ok(10),
        'B' | 'b' => Ok(11),
        'C' | 'c' => Ok(12),
        'D' | 'd' => Ok(13),
        'E' | 'e' => Ok(14),
        'F' | 'f' => Ok(15),
        _ => Err(Error::PubkeyChar { bad_char: c }),
    }
}

impl FromStr for Pubkey {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut bytes = [0; 32];
        for (i, c) in text.chars().enumerate() {
            let value = char_to_u8(c)?;
            let destination: &mut u8 = &mut bytes[i / 2];
            if i % 2 == 0 {
                *destination = value << 4;
            // most significant bits
            } else {
                // least significant bits
                *destination = value;
            }
        }

        Ok(Pubkey { bytes })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_to_u8_ok() -> Result<(), Error> {
        for n in 0..10 {
            let c = ('0' as u8 + n) as char;
            assert_eq!(char_to_u8(c)?, n);
        }

        for n in 10..16 {
            let c = ('a' as u8 + (n - 10)) as char;
            assert_eq!(char_to_u8(c)?, n);

            let c = ('A' as u8 + (n - 10)) as char;
            assert_eq!(char_to_u8(c)?, n);
        }

        Ok(())
    }

    #[test]
    fn char_to_u8_error() {
        assert_eq!(char_to_u8('r'), Err(Error::PubkeyChar { bad_char: 'r' }));
    }
}
