use pinocchio::{
    account_info::AccountInfo, entrypoint::ProgramResult, instruction::{Seed, Signer}, program_error::ProgramError
};

use crate::pinocchio_spl::{Transfer, CloseAccount, accounts::TokenAccount};

/// # Refund
/// 
/// -- Data scheme --
/// > Seed [u8; 8]
/// > Bump [u8; 1]
/// 
/// -- Account & Instruction Optimization --
/// We don't need the System program since we're not creating accounts
/// and we're draining the lamports `borrow_mut_lamports_unchecked`
/// 
/// To close the account we drain all the lamports and set the data_len to 0
/// by setting the 8 bytes before the data (data_len is u64) to 0 to prevent
/// reinitialization attack
/// 
/// * Account Optimization == -2 accounts (mint_a, system_program)
/// 
/// -- Checks --
/// + Check that Maker is a signer
/// + Check the ownership of maker_ta_a 
/// - No Check that the Escrow is derived correctly -> Cpi will fail
pub fn refund(accounts: &[AccountInfo]) -> ProgramResult {

    let [maker, maker_ta_a, escrow, vault, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(maker.is_signer());

    // Check authority
    assert_eq!(&TokenAccount::from_account_info(maker_ta_a).authority(), escrow.key());

    // Get vault amount
    let amount = TokenAccount::from_account_info(vault).amount();

    // Cast Seeds as [u8; 8] because we need it in the PDA derivation
    let seed = unsafe { *(escrow.borrow_data_unchecked().as_ptr() as *const [u8;4]) };

    // Cast Bump as u8 since we just need to save it in the Escrow
    let bump = unsafe { [*(escrow.borrow_data_unchecked().as_ptr() as *const u8)] };

    // Derive the signer
    let seeds = [
        Seed::from(seed.as_ref()),
        Seed::from(maker.key().as_ref()),
        Seed::from(&bump),
    ];
    let signer = Signer::from(&seeds);

    // Transfer out the Funds from the vault to the maker_ata_a
    Transfer {
        from: vault,
        to: maker_ta_a,
        authority: escrow,
        amount,
    }.invoke_signed(&[signer.clone()])?;

    // Close vault
    CloseAccount {
        from: vault,
        to: maker,
        authority: escrow,
    }.invoke_signed(&[signer.clone()])?;

    // Close the Escrow account by draining the lamports and setting the data_len to 0
    unsafe {
        let lamports = escrow.borrow_lamports_unchecked();
        *escrow.borrow_mut_lamports_unchecked() -= lamports;
        *maker.borrow_mut_lamports_unchecked() += lamports;

        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().sub(8) as *mut u64) = 0;
    }

    Ok(())
}
