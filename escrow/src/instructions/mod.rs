use pinocchio::program_error::ProgramError;

pub mod make;
pub mod take;
pub mod refund;

#[derive(Clone, Copy, Debug)]
pub enum EscrowInstruction {
    Make,
    Take,
    Refund,
}

impl TryFrom<&u8> for EscrowInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EscrowInstruction::Make),
            1 => Ok(EscrowInstruction::Take),
            2 => Ok(EscrowInstruction::Refund),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
