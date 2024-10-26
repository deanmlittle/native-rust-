use pinocchio::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

/// # Take
///
/// -- Data scheme --
/// > Seed [u8; 8]
/// > Bump [x; 1]
/// 
/// -- Account & Instruction Optimization --
/// 
/// -- Instruction Checks --

pub fn take(_program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {

    let [taker, mint_a, mint_b, taker_ta_a, taker_ta_b, maker_ta_b, escrow, vault, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Cast the maker from the Escrow
   
    let maker =  unsafe { *(escrow.borrow_data_unchecked().as_ptr().add(8) as *const Pubkey) };

    // Check maker_ata_a ownership
    todo!();

    // Get vault amount
    todo!();

    // Derive the seeds for the PDA
    let seeds = &[&data[0..7], maker.as_ref(), &[data[8]]];

    // Transfer out the Funds from the vault to the vault to the taker_ata_a
    todo!();

    // Close vault
    todo!();

    // Transfer out the Funds from the vault to the taker_ata_b to the maker_ata_b
    todo!();

    // Close the Escrow account by draining the lamports and setting the data_len to 0
    unsafe {
        let lamports = escrow.borrow_lamports_unchecked();
        *escrow.borrow_mut_lamports_unchecked() -= lamports;
        *taker.borrow_mut_lamports_unchecked() += lamports;

        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().sub(8) as *mut u64) = 0;
    }

    Ok(())
}
