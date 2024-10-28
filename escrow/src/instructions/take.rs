use pinocchio::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::state::Escrow;

use crate::pinocchio_spl::{accounts::TokenAccount, CloseAccount, Transfer};

/// # Take
///
/// -- Data scheme --
/// > Bump [u8; 1]
///
/// -- Instruction Logic --
/// We introduce an authority account (that is the owner of the Vault), that has 
/// defined Seeds derived from the Escrow PublicKey and this authority will be used 
/// to sign the CPIs that are going to transfer the funds to the taker_ta_a and close 
/// the vault Token Account.
/// 
/// Using the authority account permits to skip on a CPI for the creation of the TA 
/// as owner of itself since it's a system account with deterministc seeds.
/// 
/// We then transfer the right amount of mint_b by getting it from the Escrow data to the 
/// maker_ata_b that is ownerd by the maker.
/// 
/// We created a new macro to deserialize both the Token Account and the Escrow using pointers 
/// and unsafe operation to optimize grabbing data from it. <T>::from_account_info(account)
///
/// Then we close the Escrow account by draining all the lamports and setting the data_len 
/// to 0 (data_len starts 8 bytes before the actual data of the account) to prevent
/// reinitalization attack.
/// 
/// -- Client Side Logic --
/// - Derive the authority account from the Escrow PublicKey and pass in the bump.
/// - Create a Token Account owned by Maker and with mint_b as Mint
/// 
/// -- Account Optimization Logic --
/// - 4 accounts from the Anchor Escrow (maker, mint_a, mint_b, system_program)
/// + 1 account from the Anchor Escrow (authority)
///
/// -- Checks --
/// + Check that the maker_ata_b has maker as authority to enforce that the receiver of 
///   the tokens is really the maker that created the escrow.
/// + Check that the vault has mint_a as the mint to enforce that we're transferring from
///   the right vault (we can't skip this since somebody could send the authority to a 
///   worthless Token Account and the instruction will pass).
/// + Check that the maker_ta_b is the same as the one saved in the Escrow

pub fn take(accounts: &[AccountInfo], bump: [u8;1]) -> ProgramResult {
    let [taker, taker_ta_a, taker_ta_b, maker_ta_b, escrow, vault, authority, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Get the escrow_account data for future checks
    let escrow_account = Escrow::from_account_info(escrow);

    // Check maker_ata_a ownership
    assert_eq!(&TokenAccount::from_account_info(maker_ta_b).authority(), &escrow_account.maker());

    // Check vault mint
    assert_eq!(&TokenAccount::from_account_info(vault).mint(), &escrow_account.mint_a());

    // Check maker_ata_b mint
    assert_eq!(maker_ta_b.key(), &escrow_account.maker_ta_b());

    // Transfer out the Funds from the vault to the taker_ata_b to the maker_ata_b
    Transfer {
        from: taker_ta_b,
        to: maker_ta_b,
        authority: taker,
        amount: escrow_account.amount_b(),
    }
    .invoke()?;

    // Derive the signer
    let seeds = [
        Seed::from(escrow.key().as_ref()),
        Seed::from(&bump),
    ];
    let signer = [Signer::from(&seeds)];

    // Transfer out the Funds from the vault to the vault to the taker_ata_a
    Transfer {
        from: vault,
        to: taker_ta_a,
        authority,
        amount: TokenAccount::from_account_info(vault).amount(),
    }
    .invoke_signed(&signer.clone())?;

    // Close vault
    CloseAccount {
        from: vault,
        to: taker,
        authority,
    }
    .invoke_signed(&signer.clone())?;

    // Close the Escrow account by draining the lamports and setting the data_len to 0
    unsafe {
        *taker.borrow_mut_lamports_unchecked() += *escrow.borrow_lamports_unchecked();
        *escrow.borrow_mut_lamports_unchecked() = 0;

        escrow.assign(&Pubkey::default());

        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().sub(8) as *mut u64) = 0;
    }

    Ok(())
}
