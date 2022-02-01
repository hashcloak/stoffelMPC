use super::MPCProtocol;

pub struct HoneyBadgerMPC<N: Number> {

}

pub struct HBMPCPreprocessingParameters<N: Number> {

}

impl<N: Number> MPCProtocol<N> for HoneyBadgerMPC<N> {
    type Share = Number;

    type Input = Number;

    type Output = Number;

    type Error = dyn std::error::Error;

    type Parameters = HBMPCPreprocessingParameters<N>;

    fn compute() -> Result<Box<dyn Self::Output>, Box<dyn Error>> {
        todo!();
    }

    fn setup() -> Result<Self::Parameters, Self::Error> {
        todo!();
    }
}