use pinocchio::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey
};

/// # Make
///
/// -- Data scheme --
/// > maker_ta_b [u8; 32]
/// > mint_a [u8; 32]
/// > mint_b [u8; 32]
/// > receive [u8; 8]
/// 
/// -- Instruction Logic --
/// By using a keypair instead of a PDA for the Escrow, we don't need to CPI to allocate
/// space and assign it to the current program (needed because we're changing data).
/// 
/// To save data inside of the Escrow, we assign the first 32 bytes to the maker key, and
/// then we just parse in the rest of the data without deserializoing it (Saving CUs).
/// 
/// We don't need to "Deposit" in the `Make` instruction because of intent:
/// Vault get's created and deposited in an instruction of this transaction to avoid the 
/// transfer CPI -> This works because if the maker actually doesn't deposit any token, 
/// nobody will want to exchange it for the other token.
/// 
/// We don't need Mint B and Mint A accounts since we're not transferring tokens, we can
/// just pass it as data and save it in the Escrow directly.
/// 
/// Note: every CPI costs 1000 CUs, so we should avoid it as much as possible.
/// 
/// -- Client Side Logic --
/// The trade-off of saving all this CUs (from CPIs) is getting a more "Client-Heavy" approach, 
/// where this are some of the atomic instruction that the client should do:
/// - `create_account` with right Space, Lamports and ProgramId for Escrow
/// - `create` and `transfer` for the Vault
/// 
/// -- Account Optimization Logic --
/// - 5 accounts from the Anchor Escrow (mint_a, mint_b, maker_ata_a, vault, token_program)
///
/// -- Checks --
/// + Check that the Escrow is a Signer, so we know that that account hasn't been used before
///   avoid overwriting potential data inside of it.
/// - Skip ProgramId check for Escrow, it will fail when we're adding data inside of it
/// - Skip Space & Lamports check on the Escrow, it will fail on creation

pub fn make(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [maker, escrow, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure maker is signer
    assert!(maker.is_signer());

    // Copy maker key
    unsafe { 
        *(escrow.borrow_mut_data_unchecked().as_mut_ptr() as *mut Pubkey) = *maker.key()
    };
    

    // Copy everything after maker
    unsafe {
        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().add(32) as *mut [u8;104]) = *(data.as_ptr() as *const [u8;104]);
    }

    Ok(())
}
