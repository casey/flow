use crate::common::*;

#[derive(Debug)]
pub(crate) enum Error { }

impl Display for Error {
    fn fmt(&self, _f: &mut Formatter) -> fmt::Result {
        unimplemented!()
    }
}

