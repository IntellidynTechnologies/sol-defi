use solana_program::pubkey::Pubkey;
use solana_program::program_pack::{ IsInitialized, Sealed, Pack };
use borsh::BorshSerialize;
use borsh::BorshDeserialize;
use std::collections::HashMap;
use crate::error::CustomError;
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(BorshSerialize, BorshDeserialize, Default)]
pub struct Amm {
    pub is_initialized: bool,
    pub total_shares: u64,
    pub total_token_1: u64,
    pub total_token_2: u64,
    pub shares: HashMap<Pubkey, u64>,
    pub token_1_balance: HashMap<Pubkey, u64>,
    pub token_2_balance: HashMap<Pubkey, u64>,
    pub fees: u64
}

impl Sealed for Amm {}

impl IsInitialized for Amm {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Amm {
    pub fn valid_amount_check(
        &self,
        program_id: Pubkey,
        _balance: &HashMap<Pubkey, u64>,
        _qty: u64
    ) -> Result<(), CustomError> {
        let my_balance = *_balance.get(&program_id).unwrap_or(&0);

        match _qty {
            0 => Err(CustomError::ZeroAmount),
            _ if _qty >my_balance => Err(CustomError::InsufficientAmount),
            _ => Ok(())
        }
    }

    pub fn get_k(&self) -> u64 {
        self.total_token_1 * self.total_token_2
    }

    pub fn active_pool(&self) -> Result<(), CustomError> {
        match self.get_k() {
            0 => Err(CustomError::ZeroLiquidity),
            _ =>Ok(())
        }
    }
}