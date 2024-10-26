use five8_const::decode_32_const;
use pinocchio::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_nostd_sha256::hashv;
use crate::state::Escrow;

const ID: [u8; 32] = decode_32_const("9HFegTZnvebYjf9kSa6k3WBm93hRfogWB5B1goUrq1oL"); // todo

const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

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
/// This checks should be performed Client Side on the "Take" instruction!
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
/// used the wrong seed, we would lose the fund forever
/// - No Check on ProgramID since we're changing data (it needs to have our ProgramID)
/// - No Check on Space and Lamports, it will fail on creation

pub fn make(_program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {

    let [maker, escrow, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Cast seed as [u8; 8] because we need it in the PDA derivation
    // Then we can cast it to u64 when we save the data in the Escrow
    let seed = &data[0..7];

    // Cast mint_a as Pubkey since we need it in the Escrow
    let mint_a = unsafe { *(data[8..39].as_ptr() as *const Pubkey) };

    // Cast mint_b as Pubkey since we need it in the Escrow
    let mint_b = unsafe { *(data[40..71].as_ptr() as *const Pubkey) };

    // Cast mint_b as Pubkey since we need it in the Escrow
    let mint_b = unsafe { *(data[40..71].as_ptr() as *const Pubkey) };


    // Cast reveive as u64 since we just need to save it in the Escrow
    let receive = unsafe { *(data[72..79].as_ptr() as *const u64) };

    // We can just use the bump as it is since it's just a u8
    let bump = data[80];

    // Derive PDA using Hashv
    assert_eq!(hashv(&[seed, maker.key().as_ref(), &[bump], ID.as_ref(), PDA_MARKER]), escrow.key().as_ref());

    // Cast the data to Escrow and save it in the Account    
    unsafe { *(escrow.borrow_mut_data_unchecked().as_mut_ptr() as *mut Escrow) = Escrow {
            seed: *(seed.as_ptr() as *const u64),
            maker: *maker.key(),
            mint_a,
            mint_b,
            receive,  
        }
    }

    Ok(())
}
