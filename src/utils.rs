
use sha2::{Digest, Sha256};

pub fn hash_to_binary(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }

    res
}

pub fn calculate_hash(id: u64, timestamp: i64, previous_hash: &str, data: &str, pow: u64) -> Vec<u8> {
    let data = serde_json::json!({
        "id": id,
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
        "pow": pow
    });

    Sha256::new()
        .chain_update(data.to_string())
        .finalize()
        .to_vec()
}
