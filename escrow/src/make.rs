use crate::{state::Escrow, PDA_MARKER};
use pinocchio::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    memory::sol_memcpy,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::{rent::Rent, Sysvar},
};
use pinocchio_system::instructions::CreateAccount;
use solana_nostd_sha256::hashv;

/// # Make
///
/// -- Data scheme --
/// > Seed [u8; 8]
/// > MintA [u8; 32]
/// > MintB [u8; 32]
/// > Receive [x; 8]
/// > Bump [x; 1]
///
/// -- Account & Instruction Optimization --
/// We don't need to perform the "Deposit" in the Make instruction:
/// > Create Vault (ATA with Escrow as owner)
/// > Transfer x Token from maker_ata_a to Vault
/// Because:
/// > if they're not depositing token, nobody will "Take"
/// > if the Vault is not owned by the program, the "Take" will fail
/// 
///  This checks should be performed Client Side on the "Take" instruction!
/// No need for this checks on refund either since if user doesn't do it,
/// they're just losing their money
///
/// We don't need Mint B and Mint A accounts since we're not transferring tokens, we can
/// just pass it as data and save it in the Escrow directly.
///
/// * Account Optimization == -5 accounts (mint_a, mint_b, maker_ata_a, vault, token_program)
///
/// -- Escrow Checks --
/// + Check that there is not Data already inside of it, or we'll just overwrite it
/// + Check that the Escrow is derived correctly -> We could skip it, but if we
///   used the wrong seed, we would lose the fund forever
/// - No Check on ProgramID since we're changing data (it needs to have our ProgramID)
/// - No Check on Space and Lamports, it will fail on creation

pub fn make(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [maker, escrow, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(escrow.data_is_empty());

    // Derive PDA using Hashv
    let lamports = Rent::get()?.minimum_balance(Escrow::LEN);

    // Assert Escrow address is correct
    assert_eq!(
        hashv(&[
            &data[4..8],
            maker.key().as_ref(),
            &[data[0]],
            crate::ID.as_ref(),
            PDA_MARKER
        ]),
        escrow.key().as_ref()
    );

    CreateAccount {
        from: maker,
        to: escrow,
        lamports,
        space: Escrow::LEN as u64,
        owner: &crate::ID,
    }
    .invoke()?;

    // Copy everything except the maker key
    unsafe {
        sol_memcpy(escrow.borrow_mut_data_unchecked(), data, 80);
    }
    // Copy the maker key
    unsafe {
        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().add(80) as *mut Pubkey) = *maker.key();
    }

    Ok(())
}
