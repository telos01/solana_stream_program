use crate::state::{CreateStreamInput,WithdrawInput};
use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub enum StreamInstruction {
    CreateStream(CreateStreamInput),
    WithdrawFromStream(WithdrawInput),
    CloseStream,
}