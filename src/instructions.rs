use super::State;
use crate::serde::{Deserialize, Serialize};
use std::error::Error;

/// An instructions that frankolang can execute.
#[typetag::serde(tag = "type")]
pub trait FrankolangInstruction {
    fn execute_instruction(
        &self,
        state: &mut State,
    ) -> Result<(), Box<dyn Error>>;
}

#[derive(Serialize, Deserialize)]
pub struct Payment {
    sender_public_key: [u8; 32],
    reciever_public_key: [u8; 32],
    amount: u64,
}

impl Payment {
    pub fn new(
        sender_public_key: [u8; 32],
        reciever_public_key: [u8; 32],
        amount: u64,
    ) -> Payment {
        Payment {
            sender_public_key,
            reciever_public_key,
            amount,
        }
    }
}

#[typetag::serde]
impl FrankolangInstruction for Payment {
    fn execute_instruction(
        &self,
        state: &mut State,
    ) -> Result<(), Box<dyn Error>> {
        state.lower_balance(self.sender_public_key, self.amount)?;
        state.add_account(self.reciever_public_key);
        state.raise_balance(self.reciever_public_key, self.amount)?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct CheckHash;

#[typetag::serde]
impl FrankolangInstruction for CheckHash {
    fn execute_instruction(
        &self,
        _state: &mut State,
    ) -> Result<(), Box<dyn Error>> {
        // TODO
        // Calculate hash of previous instructions

        // Ask node if these hashes are on the blockchain and have not been spent

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct CoinbaseTransaction {
    miner: [u8; 32],
}

impl CoinbaseTransaction {
    pub fn new(miner: [u8; 32]) -> CoinbaseTransaction {
        CoinbaseTransaction { miner }
    }
}

#[typetag::serde]
impl FrankolangInstruction for CoinbaseTransaction {
    fn execute_instruction(
        &self,
        state: &mut State,
    ) -> Result<(), Box<dyn Error>> {
        state.add_account(self.miner);
        state.raise_balance(self.miner, 20000)?;
        Ok(())
    }
}
