use pinocchio::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{Seed, Signer},
    program_error::ProgramError,
};

use crate::{make, pinocchio_spl::{accounts::TokenAccount, CloseAccount, Transfer}, state::Escrow};

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
pub fn refund(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [maker, maker_ta_a, escrow, vault, authority, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure maker is signer
    assert!(maker.is_signer());

    // Ensure maker matches escrow maker
    let escrow_account = Escrow::from_account_info(escrow);
    assert_eq!(&escrow_account.maker(), maker.key());

    // Cast Bump as u8 since we just need to save it in the Escrow
    let bump = [data[0]];

    // Derive the signer
    let seeds = [
        Seed::from(escrow.key().as_ref()),
        Seed::from(&bump),
    ];
    let signer = [Signer::from(&seeds)];

    // Transfer all funds from the vault to maker_ta_a
    Transfer {
        from: vault,
        to: maker_ta_a,
        authority,
        amount: TokenAccount::from_account_info_unchecked(vault).amount(),
    }
    .invoke_signed(&signer)?;

    // Close vault
    CloseAccount {
        from: vault,
        to: maker,
        authority: escrow,
    }
    .invoke_signed(&signer)?;

    // Close the Escrow account by draining the lamports and setting the data_len to 0
    unsafe {
        *maker.borrow_mut_lamports_unchecked() += *escrow.borrow_lamports_unchecked();
        *escrow.borrow_mut_lamports_unchecked() = 0;
        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().sub(8) as *mut u64) = 0;
    }

    Ok(())
}
