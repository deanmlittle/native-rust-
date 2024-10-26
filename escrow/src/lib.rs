use make::make;
use pinocchio::account_info::AccountInfo;
use pinocchio::pubkey::Pubkey;
use pinocchio::{entrypoint::ProgramResult, program_error::ProgramError};
use pinocchio::entrypoint;

mod instructions;
use instructions::*;
use refund::refund;
use take::take;
mod state;
mod make;
mod refund;
mod take;
mod pinocchio_spl;


entrypoint!(process_instruction);

pub const ID: [u8;32] = five8_const::decode_32_const("22222222222222222222222222222222222222222222");

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;

    match EscrowInstruction::try_from(discriminator)? {
        EscrowInstruction::Make => make(accounts, data),
        EscrowInstruction::Take => take(accounts),
        EscrowInstruction::Refund => refund(accounts),
    }
}