use pinocchio::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::pinocchio_spl::{accounts::TokenAccount, CloseAccount, Transfer};

/// # Take
///
/// -- Data scheme --
/// > Seed [u8; 8]
/// > Bump [x; 1]
///
/// -- Account & Instruction Optimization --
///
/// -- Instruction Checks --

pub fn take(accounts: &[AccountInfo], bump: [u8;1]) -> ProgramResult {
    let [taker, taker_ta_a, taker_ta_b, maker_ta_b, escrow, vault, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Cast the maker from the Escrow
    let maker = unsafe { *(escrow.borrow_data_unchecked().as_ptr().add(8) as *const Pubkey) };

    // // Cast the mintA from the Escrow
    let mint_a = unsafe { *(escrow.borrow_data_unchecked().as_ptr().add(40) as *const Pubkey) };

    // // Cast the mintB from the Escrow
    let mint_b = unsafe { *(escrow.borrow_data_unchecked().as_ptr().add(72) as *const Pubkey) };

    // Cast the receive as u64 since we just need to save it in the Escrow
    let amount_b = unsafe { *(escrow.borrow_data_unchecked().as_ptr().add(104) as *const u64) };

    // Check maker_ata_a ownership
    assert_eq!(
        &TokenAccount::from_account_info(maker_ta_b).authority(),
        &maker
    );

    // Check vault mint
    assert_eq!(&TokenAccount::from_account_info(vault).mint(), &mint_a);

    // Check taker_ata_b mint
    assert_eq!(&TokenAccount::from_account_info(taker_ta_b).mint(), &mint_b);

    // Get vault amount
    let amount_a = TokenAccount::from_account_info(vault).amount();

    // Cast Seeds as [u8; 8] because we need it in the PDA derivation
    let seed = unsafe { *(escrow.borrow_data_unchecked().as_ptr() as *const [u8; 4]) };

    // Cast Bump as u8 since we just need to save it in the Escrow
    let bump = unsafe { [*escrow.borrow_data_unchecked().as_ptr()] };

    // Derive the signer
    let seeds = [
        Seed::from(escrow.key().as_ref()),
        Seed::from(&bump),
    ];

    let signer = Signer::from(&seeds);

    // Transfer out the Funds from the vault to the vault to the taker_ata_a
    Transfer {
        from: vault,
        to: taker_ta_a,
        authority: escrow,
        amount: amount_a,
    }
    .invoke_signed(&[signer.clone()])?;

    // Close vault
    CloseAccount {
        from: vault,
        to: taker,
        authority: escrow,
    }
    .invoke_signed(&[signer])?;

    // Transfer out the Funds from the vault to the taker_ata_b to the maker_ata_b
    Transfer {
        from: taker_ta_b,
        to: maker_ta_b,
        authority: escrow,
        amount: amount_b,
    }
    .invoke()?;

    // Close the Escrow account by draining the lamports and setting the data_len to 0
    unsafe {
        let lamports = escrow.borrow_lamports_unchecked();
        *escrow.borrow_mut_lamports_unchecked() -= lamports;
        *taker.borrow_mut_lamports_unchecked() += lamports;

        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().sub(8) as *mut u64) = 0;
    }

    Ok(())
}
