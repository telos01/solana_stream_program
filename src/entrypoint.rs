use std::thread::AccessError;

use solana_program :: {
    account_info::AccountInfo,entrypoint,entrypoint::ProgramResult,
    program_error::PrintProgramError,pubkey::Pubkey,
};

use crate::instruction;

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
)-> ProgramResult{
    Ok(())
}
entrypoint!(process_instruction);
