mod instructions;
use instructions::*;
mod state;
mod pinocchio_spl;

use pinocchio::account_info::AccountInfo;
use pinocchio::entrypoint;
use pinocchio::pubkey::Pubkey;
use pinocchio::{entrypoint::ProgramResult, program_error::ProgramError};
use make::make;
use take::take;
use refund::refund;

mod tests;

entrypoint!(process_instruction);

pub const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

pub const ID: [u8; 32] =
    five8_const::decode_32_const("22222222222222222222222222222222222222222222");

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match EscrowInstruction::try_from(discriminator)? {
        EscrowInstruction::Make => make(accounts, data),
        EscrowInstruction::Take => take(accounts, [data[0]]),
        EscrowInstruction::Refund => refund(accounts, [data[0]]),
    }
}
