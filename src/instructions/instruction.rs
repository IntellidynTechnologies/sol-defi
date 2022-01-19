use solana_program::{
    instruction::Instruction,
    instruction::AccountMeta,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
};
use std::mem::size_of;
use spl_token;

use borsh::{ BorshSerialize, BorshDeserialize };

use crate::state::Amm;
use super::init_instruction::InitInstruction;
use super::faucet_instruction::FaucetInstruction;

pub enum AmmInstruction {
    Init(InitInstruction),
    Faucet(FaucetInstruction),
    // Provide,
    // Withdraw,
    // Swap
}

impl AmmInstruction {

    pub fn unpack(input:&[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        match tag {
            0 => {
                msg!("Instruction: InitAccount");
                let fees = rest
                    .get(..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(ProgramError::InvalidInstructionData)?;
                Ok(Self::Init(InitInstruction {
                    fees
                }))
            },
            1 => {
                msg!("Instruction: Faucet");
                let amount_token_1 = rest
                    .get(0..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(ProgramError::InvalidInstructionData)?;
                let amount_token_2 = rest
                    .get(8..16)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(ProgramError::InvalidInstructionData)?;
                Ok(Self::Faucet(FaucetInstruction {
                    amount_token_1,
                    amount_token_2
                }))
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        }
    }

    // Create a new AMM instance
    pub fn new(_fees: u64) -> Amm {
        Amm {
            fees: if _fees >= 1000 { 0 } else { _fees },
            ..Default::default()
        }
    }
}

pub fn unpack_rest(input: &[u8]) -> Result<u64, ProgramError> {
    let amount = input
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(ProgramError::InvalidInstructionData)?;
    Ok(amount)
}