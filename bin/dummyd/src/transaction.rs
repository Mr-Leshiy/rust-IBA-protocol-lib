use interpreter::gen_interpreter;
use parity_scale_codec::{Decode, Encode};
use script::{opcode::OpCode, Script};
use sha2::{Digest, Sha256};
use std::{
    convert::TryInto,
    fmt::{Debug, Display},
};
use transaction::Transaction as IbaTransaction;

#[derive(Debug)]
pub enum TransactionError {
    InvalidScript,
    InvalidEvaluation,
}

#[derive(Encode, Decode, PartialEq, Clone, Debug)]
pub struct Transaction {
    iba_tx: IbaTransaction,
}

struct OpEcho;

impl OpCode for OpEcho {
    type Args = ();
    type Res = ();
    const CODE: u32 = 5;

    fn handler(_args: Self::Args) -> Self::Res {
        println!("OpEcho !!!");
    }
}

impl Transaction {
    pub fn new(timestamp: u64) -> Self {
        let version = 0;
        let executed_script = Script::new().push_op_code_chain::<OpEcho>();
        let conditional_script = Script::new();

        Self {
            iba_tx: IbaTransaction::new(version, timestamp, executed_script, conditional_script),
        }
    }

    pub fn hash(&self) -> [u8; 32] {
        self.iba_tx.hash()
    }

    pub fn execute(&self) -> Result<(), TransactionError> {
        let mut script = self.iba_tx.executed_script().clone();

        let interpret = gen_interpreter!(OpEcho {});

        // TODO: process error from from the script execution
        match interpret(&mut script).map_err(|_| TransactionError::InvalidScript)? {
            Some(res) => res
                .get_value::<()>()
                .map_err(|_| TransactionError::InvalidEvaluation),
            None => Err(TransactionError::InvalidEvaluation),
        }
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "transaction hash: ({})", hex::encode(self.hash()))
    }
}

// calculate a sha256 hash from the transaction hashes
pub fn calculate_root_hash(transactions: &[Transaction]) -> [u8; 32] {
    let mut data = Vec::new();
    transactions.iter().for_each(|tx| {
        data.append(&mut tx.hash().to_vec());
    });

    Sha256::new()
        .chain(data)
        .finalize()
        .try_into()
        .map_err(|_| "Expected length of the array is 32")
        .unwrap()
}
