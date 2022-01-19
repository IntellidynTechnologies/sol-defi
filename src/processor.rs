use solana_program::{
    pubkey::Pubkey,
    account_info::{next_account_info, AccountInfo },
    program_error::ProgramError,
    entrypoint::ProgramResult,
    msg
};
use borsh::{BorshSerialize, BorshDeserialize};
use crate::instructions::instruction::AmmInstruction;
use crate::instructions::init_instruction::InitInstruction;
use crate::instructions::faucet_instruction::FaucetInstruction;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Processor;

impl Processor {
    pub fn process_instruction(
        program_id:&Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8]
    ) -> ProgramResult {

        let instruction = AmmInstruction::unpack(instruction_data)?;

        match instruction {
            AmmInstruction::Init(InitInstruction {
                fees
            }) => {
                msg!("Calling initialize pool function");
                return Self::process_initialize_pool(fees, program_id, accounts);
            },
            AmmInstruction::Faucet(FaucetInstruction {
                amount_token_1,
                amount_token_2
            }) => {
                msg!("Calling Faucet function");
                return Self::process_faucet(amount_token_1, amount_token_2, program_id, accounts);
            }
        };
    }

    pub fn process_initialize_pool(
        fees: u64,
        program_id: &Pubkey,
        accounts: &[AccountInfo]
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
    
        let amm_id = next_account_info(accounts_iter)?;
        let amm_authority = next_account_info(accounts_iter)?;
        let amm_open_orders = next_account_info(accounts_iter)?;
        let lp_mint_address = next_account_info(accounts_iter)?;
        let coin_mint_address = next_account_info(accounts_iter)?;
        let pc_mint_address = next_account_info(accounts_iter)?;
        let pool_coin_token_account = next_account_info(accounts_iter)?;
        let pool_pc_token_account = next_account_info(accounts_iter)?;
        let pool_withdraw_queue = next_account_info(accounts_iter)?;
        let pool_target_orders_account = next_account_info(accounts_iter)?;
        let pool_lp_token_account = next_account_info(accounts_iter)?;
        let pool_temp_lp_token_account = next_account_info(accounts_iter)?;
        let nova_program_id = next_account_info(accounts_iter)?;
        let nova_market = next_account_info(accounts_iter)?;
        let user_wallet = next_account_info(accounts_iter)?;
        let nova_token_account = next_account_info(accounts_iter)?;
    
        Ok(())
    }

    pub fn process_faucet(
        amount_token_1: u64,
        amount_token_2: u64,
        program_id: &Pubkey,
        accounts: &[AccountInfo]
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        Ok(())
    }
}