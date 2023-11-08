pub mod annotation_parser;
pub mod fri_merkle_statement;
pub mod merkle_statement;

use ethers::types::U256;
use serde::{ser::SerializeSeq, Serialize, Serializer};
use serde_json::Value;

pub fn serialize_u256_as_number<S>(value: &U256, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let value_str = value.to_string();
    //todo: in order to align with the number format generated by the python code, for now u256 is serialized as a json number
    let json_value = Value::Number(value_str.parse::<serde_json::Number>().unwrap());

    json_value.serialize(serializer)
}

pub fn serialize_vec_u256_as_number<S>(vec: &[U256], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(vec.len()))?;
    for element in vec {
        // Use the previously defined serialize_u256_as_number for each element
        seq.serialize_element(&SerializeU256AsNumber(element))?;
    }
    seq.end()
}

// Wrapper type to use the serialize_u256_as_number function
pub struct SerializeU256AsNumber<'a>(&'a U256);

impl<'a> Serialize for SerializeU256AsNumber<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_u256_as_number(self.0, serializer)
    }
}
