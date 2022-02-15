use borsh::{BorshDeserialize,BorshSerialize};
use solana_program::{clock::UnixTimestamp,pubkey::Pubkey};

#[derive(Clone,Debug,PartialEq,BorshDeserialize,BorshSerialize)]
pub struct CreateStreamInput{
    pub start_time: UnixTimestamp,
    pub end_time: UnixTimestamp,
    pub recevier: Pubkey,
    pub lamports_withdrawn: u64,
    pub amount_second: u64,
}

#[derive(Clone,Debug,PartialEq,BorshDeserialize,BorshSerialize)]
pub struct WithdrawInput{
    pub amount: u64,
}