use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use ethers::{
    contract::abigen,
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::Wallet,
    types::{Address, U256},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::ContractFunctionCall;

/// Decommitment for a merkle statement
#[derive(Serialize, Deserialize, Debug)]
pub struct MerkleStatement {
    expected_root: U256,
    n_unique_queries: usize,
    merkle_height: usize,
    merkle_queue_indices: Vec<U256>,
    merkle_queue_values: Vec<U256>,
    proof: Vec<U256>,
}

abigen!(
    MerkleStatementContract,
    r#"[
        function verifyMerkle(uint256[] proof,uint256[] merkle_queue,uint256 merkle_height,uint256 expected_root)
    ]"#,
    derives(serde::Deserialize, serde::Serialize)
);

impl MerkleStatement {
    pub fn new(
        expected_root: U256,
        n_unique_queries: usize,
        merkle_height: usize,
        merkle_queue_indices: Vec<U256>,
        merkle_queue_values: Vec<U256>,
        proof: Vec<U256>,
    ) -> MerkleStatement {
        MerkleStatement {
            expected_root,
            n_unique_queries,
            merkle_height,
            merkle_queue_indices,
            merkle_queue_values,
            proof,
        }
    }

    /// Constructs the merkle_queue by interleaving indices and values.
    fn merkle_queue(&self) -> Vec<U256> {
        self.merkle_queue_indices
            .iter()
            .zip(self.merkle_queue_values.iter())
            .flat_map(|(&index, &value)| vec![index, value])
            .collect()
    }

    /// Constructs `verifyMerkle` contract function call.
    pub fn contract_function_call(&self) -> VerifyMerkleCall {
        VerifyMerkleCall {
            proof: self.proof.clone(),
            merkle_queue: self.merkle_queue(),
            merkle_height: U256::from(self.merkle_height),
            expected_root: self.expected_root,
        }
    }

    /// Initiates `verifyMerkle` contract call.
    pub fn verify(
        &self,
        address: Address,
        signer: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    ) -> ContractFunctionCall {
        let contract = MerkleStatementContract::new(address, signer);

        let verify_merkle_call = self.contract_function_call();
        contract.method("verifyMerkle", verify_merkle_call).unwrap()
    }

    pub fn to_json(&self) -> String{

        let initial_merkle_queue: Vec<String> = self
            .merkle_queue_indices
            .iter()
            .zip(self.merkle_queue_values.iter())
            .flat_map(|(&index, &value)| vec![index.to_string(), value.to_string()])
            .collect();

        let json_data = json!({
            "expectedRoot": self.expected_root.to_string(),
            "height": self.merkle_height.to_string(),
            "merkleView": self.proof.iter().map(|p| p.to_string()).collect::<Vec<String>>(),
            "initialMerkleQueue": initial_merkle_queue,
        });

        serde_json::to_string_pretty(&json_data).expect("Unable to serialize data")
    }

    pub fn write_to_json(&self,  file_name: &str) {
        let file_path = format!("{}.json", file_name);
        let mut file = File::create(file_path).expect("Unable to create file");

        let json_string = self.to_json();
        file.write_all(json_string.as_bytes()).expect("Unable to write data");

    }
}
