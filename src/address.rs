use crate::common::*;

pub(crate) struct Address { }

impl FromStr for Address {
    type Err = Error;

    fn from_str(_text: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    // good:
    // "foo@bar"
    //
    // bad:
    // - ""
    // - "@"
    // - "@bar"
    // - "foo"
    // - "foo@"
}
