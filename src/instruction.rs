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

impl StreamInstruction {
    pub fn unpack(instruction_data: &[u8])-> Result<Self, ProgramError>{
        let (tag,data)= instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
        match tag {
            1=> Ok(StreamInstruction::CreateStream(
                CreateStreamInput::try_from_slice(data)?,
            )),
            2=> Ok(StreamInstruction::WithdrawFromStream(
                WithdrawInput::try_from_slice(data)?,
            )),
            3=> Ok(StreamInstruction::CloseStream),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}