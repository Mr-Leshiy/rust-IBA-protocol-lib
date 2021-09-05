use std::convert::TryInto;

use parity_scale_codec::{Decode, Encode};
use sha2::{Digest, Sha256};

#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode)]
pub struct Transaction {
    #[codec(compact)]
    version: u32,
    #[codec(compact)]
    timestamp: u64,
    executed_data: Vec<u8>,
    condition_data: Vec<u8>,
}

impl Transaction {
    /// Calculates a Sha256 hash of the function
    pub fn hash(&self) -> [u8; 32] {
        Sha256::new()
            .chain(self.encode())
            .finalize()
            .try_into()
            .map_err(|_| "Expected length of the array is 32")
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_transaction() -> Transaction {
        Transaction {
            version: 1,
            timestamp: 2,
            executed_data: vec![9, 9],
            condition_data: vec![9, 9],
        }
    }

    #[test]
    fn ser_test() {
        let transaction = default_transaction();
        let expected_serialized = hex::decode("0408080909080909").unwrap();
        let serialized = transaction.encode();
        assert_eq!(serialized, expected_serialized);
    }

    #[test]
    fn de_test() {
        let expected_deserialized = default_transaction();
        let serialized = hex::decode("0408080909080909").unwrap();
        let deserialized = Transaction::decode(&mut serialized.as_ref()).unwrap();
        assert_eq!(deserialized, expected_deserialized);
    }

    #[test]
    fn ser_de_test() {
        let transaction = default_transaction();
        let serialized = transaction.encode();
        let deserialized = Transaction::decode(&mut serialized.as_ref()).unwrap();
        assert_eq!(transaction, deserialized);
    }

    #[test]
    fn hash_test() {
        let transaction = default_transaction();
        let expected_hash: [u8; 32] =
            hex::decode("7d8ca5ced41078e1802eb01026f2c301a73281f6b993c0d702d382449b009bc2")
                .unwrap()
                .try_into()
                .unwrap();
        assert_eq!(transaction.hash(), expected_hash);
    }
}
