use solana_nostd_sha256::hashv;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};
use solana_program::entrypoint;
mod tests;

const ID: [u8;32] = [0x7b, 0x07, 0x5a, 0x4f, 0xca, 0x15, 0x61, 0x6e, 0xbe, 0x53, 0xc1, 0xa8, 0x43, 0x6f, 0x42, 0x89, 0x2b, 0x02, 0x1a, 0xb6, 0x62, 0x5a, 0x2a, 0x02, 0x2a, 0x68, 0x9a, 0xef, 0xbd, 0xed, 0x26, 0xef];

const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

entrypoint!(withdraw);

/// # Withdraw
/// 
/// Handles withdrawing funds from a PDA that has previously had lamports deposited to it.
pub fn withdraw(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8]
) -> ProgramResult {
    let [signer, vault, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    let lamports: u64 = unsafe { *data.as_ptr() as u64 };
    let bump = data[8];
    let pda = hashv(&[signer.key.as_ref(), &[bump], crate::ID.as_ref(), PDA_MARKER]);

    assert_eq!(&pda, vault.key.as_ref());

    **vault.try_borrow_mut_lamports()? -= lamports;
    **signer.try_borrow_mut_lamports()? += lamports;

    Ok(())
}
