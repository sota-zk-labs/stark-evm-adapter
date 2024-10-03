use crate::annotated_proof::AnnotatedProof;
use crate::annotation_parser::{split_fri_merkle_statements, SplitProofs};
use crate::default_prime;
use crate::oods_statement::FactTopology;

pub struct Proof {
    merkle_proofs: Vec<String>,
    fri_proofs: Vec<String>,
    memory_pages: Vec<String>,
    main_proof: String,
    layout: usize,
}

impl Proof{
    pub fn new(topology_json: serde_json::Value, annotated_proof: AnnotatedProof, layout: usize) -> Result<Self, String>{
        if layout == 6 {
            Ok(Self::generate_layout6_proof(topology_json, annotated_proof))
        } else {
            Err(String::from("Unsupported layout"))
        }
    }

    pub fn generate_layout6_proof(topology_json: serde_json::Value, annotated_proof: AnnotatedProof) -> Self{
        let mut merkle_proofs: Vec<String> = vec![];
        let mut fri_proofs: Vec<String> = vec![];
        let mut memory_pages: Vec<String> = vec![];
        let mut split_proofs: SplitProofs = split_fri_merkle_statements(annotated_proof.clone()).unwrap();
        let fact_topologies: Vec<FactTopology> =
            serde_json::from_value(topology_json.get("fact_topologies").unwrap().clone()).unwrap();

        for fri_statement in split_proofs.fri_merkle_statements {
            fri_proofs.push(fri_statement.to_json());
        }

        for i in 0..split_proofs.merkle_statements.len() {
            let key = format!("Trace {}", i);
            let trace_merkle = split_proofs.merkle_statements.get(&key).unwrap();
            merkle_proofs.push(trace_merkle.to_json());
        }

        let (_, continuous_pages) = split_proofs.main_proof.memory_page_registration_args();
        for page in continuous_pages {
            let page_json = page.to_json(
                split_proofs.main_proof.interaction_z,
                split_proofs.main_proof.interaction_alpha,
                default_prime(),
            );
            memory_pages.push(page_json);
        }

        let main_proof = split_proofs.main_proof.to_json(fact_topologies, 6);
        Self {
            merkle_proofs,
            fri_proofs,
            memory_pages,
            main_proof,
            layout: 6
        }


    }
}