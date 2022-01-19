//Entrypoint to the program

use solana_program::{
    pubkey::Pubkey,
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    msg!(
        "Process Instruction: {}: {} accounts, data = {:?}",
        program_id,
        accounts.len(),
        instruction_data
    );

    Ok(())
}