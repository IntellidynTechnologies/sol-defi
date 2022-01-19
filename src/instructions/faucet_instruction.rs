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

#[derive(BorshSerialize, BorshDeserialize)]
pub struct FaucetInstruction {
    pub amount_token_1: u64,
    pub amount_token_2: u64
}

impl FaucetInstruction {
    pub fn faucet(
        program_id: &Pubkey,
        amm_id: &Pubkey,
        amm_authority: &Pubkey,
        amm_open_orders: &Pubkey,
        lp_mint_address: &Pubkey,
        coin_mint_address: &Pubkey,
        pc_mint_address: &Pubkey,
        pool_coin_token_account: &Pubkey,
        pool_pc_token_account: &Pubkey,
        pool_withdraw_queue: &Pubkey,
        pool_target_orders_account: &Pubkey,
        pool_lp_token_account: &Pubkey,
        pool_temp_lp_token_account: &Pubkey,
        nova_program_id: &Pubkey,
        nova_market: &Pubkey,
        user_wallet: &Pubkey,
        nova_token_account: Option<Pubkey>,
        amount_token_1: u64,
        amount_token_2: u64
    ) -> Result<Instruction, ProgramError> {

        let mut buf = Vec::with_capacity(size_of::<Self>());
        buf.push(0);

        let mut accounts = vec![
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
            // amm
            AccountMeta::new(*amm_id, false),
            AccountMeta::new_readonly(*amm_authority, false),
            AccountMeta::new(*amm_open_orders, false),
            AccountMeta::new(*lp_mint_address, false),
            AccountMeta::new_readonly(*coin_mint_address, false),
            AccountMeta::new_readonly(*pc_mint_address, false),
            AccountMeta::new_readonly(*pool_coin_token_account, false),
            AccountMeta::new_readonly(*pool_pc_token_account, false),
            AccountMeta::new(*pool_withdraw_queue, false),
            AccountMeta::new(*pool_target_orders_account, false),
            AccountMeta::new(*pool_lp_token_account, false),
            AccountMeta::new_readonly(*pool_temp_lp_token_account, false),
            // nova
            AccountMeta::new_readonly(*nova_program_id, false),
            AccountMeta::new_readonly(*nova_market, false),
            // user wallet
            AccountMeta::new(*user_wallet, true),
        ];

        if let Some(nova_token_key) = nova_token_account {
            accounts.push(AccountMeta::new(nova_token_key, false),)
        }

        buf.push(amount_token_1);
        buf.push(amount_token_2);

        Ok(Instruction::new_with_borsh(
            *program_id,
            &buf,
            accounts,
        ))
    }
}