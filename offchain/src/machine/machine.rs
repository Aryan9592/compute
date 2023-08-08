use std::error::Error;

use async_trait::async_trait;

use crate::machine::commitment::ComputationCommitment;

#[async_trait]
pub trait Machine {
    async fn build_commitment(&self) -> Result<ComputationCommitment, Box<dyn Error>>;
}

pub struct MachineJsonRpcClient {
}

#[async_trait]
impl Machine for MachineJsonRpcClient {
    async fn build_commitment(&self) -> Result<ComputationCommitment, Box<dyn Error>> {
        Ok(ComputationCommitment{})
    }
}