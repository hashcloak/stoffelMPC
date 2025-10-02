use clap::Parser;

/// Compile the Stoffel language file into its constituent components:
/// - A Stoffel binary file for the MPC nodes
/// - A Solidity file for handling public inputs
#[derive(Debug, Default, Parser)]
pub struct Command {

}

pub(crate) fn exec(command: Command) -> Result<()> {
    todo!();
}