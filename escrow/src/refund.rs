use five8_const::decode_32_const;
use pinocchio::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_nostd_sha256::hashv;
use crate::state::Escrow;

const ID: [u8; 32] = decode_32_const("9HFegTZnvebYjf9kSa6k3WBm93hRfogWB5B1goUrq1oL"); // todo

const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

/// # Refund
/// 
/// -- Data scheme --
/// > Seed [u8; 8]
/// > bump [u8; 1]
/// 
/// -- Account & Instruction Optimization --
/// We don't need the System program since we're not creating accounts
/// and we're draining the lamports `borrow_mut_lamports_unchecked`
/// 
/// To close the account we drain all the lamports and set the data_len to 0
/// by setting the 8 bytes before the data (data_len is u64) to 0 to prevent
/// reinitialization attack
/// 
/// * Account Optimization == -1 accounts (system_program)
/// 
/// -- Checks --
/// + Check that Maker is a signer
/// + Check the ownership of maker_ta_a 
/// - No Check that the Escrow is derived correctly -> Cpi will fail

/// 
/// For 

pub fn refund(_program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {

    let [maker, mint_a, maker_ta_a, escrow, vault, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(maker.is_signer());

    // Check maker_ata_a ownership
    todo!();

    // Get vault amount
    todo!();

    // Derive the seeds for the PDA
    let seeds = &[&data[0..7], maker.key().as_ref(), &[data[8]]];

    // Transfer out the Funds from the vault to the maker_ata_a
    todo!();

    // Close vault
    todo!();

    // Close the Escrow account by draining the lamports and setting the data_len to 0
    unsafe {
        let lamports = escrow.borrow_lamports_unchecked();
        *escrow.borrow_mut_lamports_unchecked() -= lamports;
        *maker.borrow_mut_lamports_unchecked() += lamports;

        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().sub(8) as *mut u64) = 0;
    }

    Ok(())
}
