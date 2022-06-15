use clap::Parser;

/// Removes any artifacts generated during the run of `mwrc build`
#[derive(Debug, Default, Parser)]
pub struct Command {

}

pub(crate) fn exec(command: Command) -> Result<()> {
    todo!();
}