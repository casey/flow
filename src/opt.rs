use crate::common::*;

#[derive(StructOpt)]
#[cfg_attr(test, structopt(raw(setting = "AppSettings::ColorNever")))]
pub(crate) struct Opt {
    #[structopt(name = "DESTINATION", long = "destination")]
    _destination: Address,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test arguement parsing for `flow --destination PUBKEY@IP`
    fn cli_destination() -> Result<(), structopt::clap::Error> {
        let _opt = Opt::from_iter_safe(vec!["flow", "--destination", "PUBKEY@127.0.0.1:8080"])?;

        Ok(())
    }

}
