use std::str::FromStr;

use crate::{
    error::StreamError,
    instruction::StreamInstruction,
    state::{CreateStreamInput, StreamData, WithdrawInput},
};

use borsh::{BorshDeserialize,BorshSerialize};
use solana_program::{
    account_info::{next_account_info,AccountInfo, self},
    clock::Clock,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent,Sysvar},
};
pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    )-> ProgramResult{
        let instruction = StreamInstruction::unpack(instruction_data)?;
        match instruction {
            StreamInstruction:: CreateStream(data) => {
                Self::process_create_stream(program_id, accounts, data)
            }
            StreamInstruction::WithdrawFromStream(data) => {
                Self::process_withdraw(program_id, accounts, data)
            }
            StreamInstruction::CloseStream => {
                Self::process_close(program_id, accounts)
            }

        }
    }

    fn process_create_stream(
        _program_id: &Pubkey,
        accounts : &[AccountInfo],
        data : CreateStreamInput,
        ) -> ProgramResult {
            let admin_pub_key = match Pubkey::from_str("DGqXoguiJnAy8ExJe9NuZpWrnQMCV14SdEdiMEdCfpmB"){
                Ok(key)=> key,
                Err(_)=> return Err(StreamError::PubKeyParseError.into()),
            };            
            let account_info_iter= &mut accounts.iter();
            let escrow_account = next_account_info(account_info_iter)?;
            let sender_account= next_account_info(account_info_iter)?;
            let receiver_account= next_account_info(account_info_iter)?;
            let admin_account = next_account_info(account_info_iter)?;
            
            if *admin_account.key != admin_pub_key {
                return Err(StreamError::AdminAccountInvalid.into())
            }
            **escrow_account.try_borrow_mut_lamports()? -=  30000000;
            **admin_account.try_borrow_mut_lamports()? +=  30000000;
            if data.end_time <= data.start_time|| data.start_time < Clock::get()?.unix_timestamp {
                return  Err(StreamError::InvalidStartOrEndTime.into());
            }
            if data.amount_second * ((data.end_time - data.start_time) as u64)
            != **escrow_account.lamports.borrow() - Rent::get()?.minimum_balance(escrow_account.data_len())
        {
            return Err(StreamError::NotEnoughLamports.into());
        }
        if !sender_account.is_signer{
            return Err(ProgramError::MissingRequiredSignature);
        }
        if *receiver_account.key != data.recevier {
            return Err(ProgramError::InvalidAccountData);
        }
        let escrow_data = StreamData::new(data, *sender_account.key);
        escrow_data.serilalize(&mut &mut escrow_account.data.borrow_mut()[..]?);
        Ok(())
    }
    
    fn process_withdraw(
        _program_id: &Pubkey,
        accounts : &[AccountInfo],
        data: WithdrawInput,
    ) -> ProgramResult{
        Ok(())
    }

    fn process_close(
        _program_id: &Pubkey,
        accounts : &[AccountInfo]        
    )-> ProgramResult{
        Ok(())
    }
    
}
