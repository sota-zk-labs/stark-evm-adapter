use std::env;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use ethers::types::U256;
use serde_json::{json, to_string_pretty};
use crate::annotated_proof::AnnotatedProof;
use crate::annotation_parser::{split_fri_merkle_statements, SplitProofs};
use crate::default_prime;
use crate::oods_statement::FactTopology;

#[cfg(test)]
#[test]
fn test_parser_layout7() -> Result<(), Box<dyn std::error::Error>>{
    let origin_proof_file = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/proof_layout7/bootloader_serialized_proof.json"
    // "/tests/fixtures/annotated_proof.json"
    ));


    let annotated_proof: AnnotatedProof = serde_json::from_str(&origin_proof_file)?;
    // generate split proofs
    let mut split_proofs: SplitProofs = split_fri_merkle_statements(annotated_proof.clone()).unwrap();

    let topologies_file = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/proof_layout7/fact_topologies.json"
    ));
    let topology_json: serde_json::Value = serde_json::from_str(&topologies_file).unwrap();

    let fact_topologies: Vec<FactTopology> =
        serde_json::from_value(topology_json.get("fact_topologies").unwrap().clone()).unwrap();

    split_proofs.main_proof.fit_layout_7();

    // for (i, fri_statement) in split_proofs.fri_merkle_statements.iter().enumerate() {
    //         fri_statement.write_to_json(&format!("proof/fri_verify_{}", i + 1));
    // }
    //
    // for i in 0..split_proofs.merkle_statements.len() {
    //     let key = format!("Trace {}", i);
    //     let trace_merkle = split_proofs.merkle_statements.get(&key).unwrap();
    //     trace_merkle.write_to_json(&format!("proof/merkle_verify_{}", i+1));
    // }
    //
    //
    // let (_, continuous_pages) = split_proofs.main_proof.memory_page_registration_args();
    // let mut memory_page_entries: Vec<serde_json::Value> = vec![];
    // for page in continuous_pages {
    //     let page_json = page.to_json(
    //         split_proofs.main_proof.interaction_z,
    //         split_proofs.main_proof.interaction_alpha,
    //         default_prime(),
    //         // U256::from("3618502788666131213697322783095070105623107215331596699973092056135872020481")
    //     );
    //     memory_page_entries.push(serde_json::from_str(&page_json).unwrap());
    // }
    // let json_data = json!({
    //     "memoryPageEntries": memory_page_entries
    // });
    //
    // let json_string = serde_json::to_string_pretty(&json_data).expect("Unable to serialize data");
    // let mut file = File::create("proof/memory_page_entries.json").expect("Unable to create file");
    // file.write_all(json_string.as_bytes()).expect("Unable to write data");


    split_proofs.main_proof.write_to_json(fact_topologies, &format!("proof_layout7/verify_proof_and_register",), 7);


    println!("vjp");

    Ok(())
}

#[test]
fn test_parser_layout6() -> Result<(), Box<dyn std::error::Error>>{
    let origin_proof_file = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/proof_layout6/bootloader_serialized_proof.json"
    ));
    let annotated_proof: AnnotatedProof = serde_json::from_str(&origin_proof_file)?;
    // generate split proofs
    let mut split_proofs: SplitProofs = split_fri_merkle_statements(annotated_proof.clone()).unwrap();

    let topologies_file = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/proof_layout6/fact_topologies.json"
    ));
    let topology_json: serde_json::Value = serde_json::from_str(&topologies_file).unwrap();

    let fact_topologies: Vec<FactTopology> =
        serde_json::from_value(topology_json.get("fact_topologies").unwrap().clone()).unwrap();

    for (i, fri_statement) in split_proofs.fri_merkle_statements.iter().enumerate() {
            fri_statement.write_to_json(&format!("proof_layout6/fri_verify_{}", i + 1));
    }

    for i in 0..split_proofs.merkle_statements.len() {
        let key = format!("Trace {}", i);
        let trace_merkle = split_proofs.merkle_statements.get(&key).unwrap();
        trace_merkle.write_to_json(&format!("proof_layout6/merkle_verify_{}", i+1));
    }

    let (_, continuous_pages) = split_proofs.main_proof.memory_page_registration_args();
    let mut memory_page_entries: Vec<serde_json::Value> = vec![];
    for page in continuous_pages {
        let page_json = page.to_json(
            split_proofs.main_proof.interaction_z,
            split_proofs.main_proof.interaction_alpha,
            default_prime(),
        );
        memory_page_entries.push(serde_json::from_str(&page_json).unwrap());
    }
    let json_data = json!({
        "memoryPageEntries": memory_page_entries
    });

    let json_string = serde_json::to_string_pretty(&json_data).expect("Unable to serialize data");
    let mut file = File::create("proof_layout6/memory_page_entries.json").expect("Unable to create file");
    file.write_all(json_string.as_bytes()).expect("Unable to write data");

    split_proofs.main_proof.write_to_json(fact_topologies, &format!("proof_layout6/verify_proof_and_register",), 6);
    Ok(())
}

fn write_to_json(data: &[U256], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_data: Vec<String> = data.iter().map(|u| u.to_string()).collect();
    let json_string = to_string_pretty(&json_data)?;
    let mut file = File::create(filename)?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}