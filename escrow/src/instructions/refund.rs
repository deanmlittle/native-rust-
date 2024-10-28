use pinocchio::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{Seed, Signer},
    program_error::ProgramError,
};

use crate::{pinocchio_spl::{accounts::TokenAccount, CloseAccount, Transfer}, state::Escrow};

/// # Refund
///
/// -- Data scheme --
/// > bump [u8; 1]
/// 
/// -- Instruction Logic --
/// We introduce an authority account (that is the owner of the Vault), that has 
/// defined Seeds derived from the Escrow PublicKey and this authority will be used 
/// to sign the CPIs that are going to transfer the funds back to the maker_ta_a and
/// closing the vault Token Account.
/// 
/// Using the authority account permits to skip on a CPI for the creation of the TA 
/// as owner of itself since it's a system account with deterministc seeds.
/// 
/// We created a new macro to deserialize the Token Account using pointers and unsafe
/// operation to optimize grabbing the amount of token inside of it:
/// `TokenAccount::from_account_info_unchecked(vault).amount()`
/// 
/// Then we close the Escrow account by draining all the lamports and setting the data_len 
/// to 0 (data_len starts 8 bytes before the actual data of the account) to prevent
/// reinitalization attack.
/// 
/// -- Client Side Logic --
/// Derive the authority account from the Escrow PublicKey and pass in the bump.
///
/// -- Account Optimization Logic --
/// - 2 accounts from the Anchor Escrow (mint_a, system_program)
/// + 1 account from the Anchor Escrow (authority)
///
/// -- Checks --
/// + Check that Maker is a signer (since it's the owner of the tokens in the Vault)
/// + Check the ownership of maker_ta_a (since we're transferring the funds to it)

pub fn refund(accounts: &[AccountInfo], bump: [u8;1]) -> ProgramResult {
    let [maker, maker_ta_a, escrow, vault, authority, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure maker is signer
    assert!(maker.is_signer());

    // Ensure maker matches escrow maker
    let escrow_account = Escrow::from_account_info(escrow);
    assert_eq!(&escrow_account.maker(), maker.key());

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
