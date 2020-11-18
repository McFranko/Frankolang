#![allow(dead_code)]

extern crate bincode;
extern crate rand;
extern crate serde;
extern crate typetag;

pub mod instructions;

use std::error::Error;

use instructions::FrankolangInstruction;

pub struct FrankolangScript {
    instructions: Vec<Box<dyn FrankolangInstruction>>,
    pub state: State,
}

impl FrankolangScript {
    pub fn from_instructions(
        instructions: Vec<Box<dyn FrankolangInstruction>>,
    ) -> FrankolangScript {
        FrankolangScript {
            instructions,
            state: State::new(),
        }
    }

    pub fn from_bytes(
        code: Vec<u8>,
    ) -> Result<FrankolangScript, Box<dyn Error>> {
        let code: Vec<Box<dyn FrankolangInstruction>> =
            bincode::deserialize(&code)?;
        let frankolang_code = FrankolangScript::from_instructions(code);

        Ok(frankolang_code)
    }
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let bytes = bincode::serialize(&self.instructions)?;

        Ok(bytes)
    }

    pub fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        for instruction in &self.instructions {
            instruction.execute_instruction(&mut self.state)?;
        }
        Ok(())
    }
}

/// The state consists of a list of accounts. The state gets changed by the frankolang that gets
/// executed.
pub struct State {
    accounts: Vec<Account>,
}

impl State {
    fn new() -> State {
        State {
            accounts: Vec::new(),
        }
    }

    fn raise_balance(
        &mut self,
        public_key: [u8; 32],
        change: u64,
    ) -> Result<(), Box<dyn Error>> {
        let account_index = self
            .accounts
            .iter()
            .position(|account| account.public_key == public_key)
            .ok_or(FrankolangError::new(
                FrankolangErrorKind::AccountDoesNotExist,
            ))?;
        let account = &mut self.accounts[account_index];

        account.balance += change;

        Ok(())
    }

    fn lower_balance(
        &mut self,
        public_key: [u8; 32],
        change: u64,
    ) -> Result<(), Box<dyn Error>> {
        let account_index = self
            .accounts
            .iter()
            .position(|account| account.public_key == public_key)
            .ok_or(FrankolangError::new(
                FrankolangErrorKind::AccountDoesNotExist,
            ))?;
        let account = &mut self.accounts[account_index];

        if account.balance < change {
            return Err(FrankolangError::new(
                FrankolangErrorKind::BalanceBelowZero,
            ));
        }

        account.balance -= change;

        Ok(())
    }

    fn add_account(&mut self, public_key: [u8; 32]) {
        let account_exists = self
            .accounts
            .iter()
            .position(|account| account.public_key == public_key)
            .is_some();
        if account_exists {
            return;
        }

        let new_account = Account {
            public_key,
            balance: 0,
        };

        self.accounts.push(new_account);
    }
}

struct Account {
    pub public_key: [u8; 32],
    pub balance: u64,
}

#[derive(Debug)]
struct FrankolangError {
    error_kind: FrankolangErrorKind,
}

impl FrankolangError {
    fn new(error_kind: FrankolangErrorKind) -> Box<FrankolangError> {
        Box::new(FrankolangError { error_kind })
    }
}

impl Error for FrankolangError {}
impl std::fmt::Display for FrankolangError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{:#?}", self.error_kind)?;
        Ok(())
    }
}

#[derive(Debug)]
enum FrankolangErrorKind {
    AccountDoesNotExist,
    AccountAlreadyExists,
    BalanceBelowZero,
}
