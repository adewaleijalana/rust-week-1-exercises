// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    hex::decode(raw_tx_hex)
        .map_err(|_| "Hex decode error".to_string())
        .and_then(|bytes| {
            if bytes.len() < 4 {
                Err("Transaction data too short".to_string())
            } else {
                let version = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                Ok(version)
            }
        })
}
