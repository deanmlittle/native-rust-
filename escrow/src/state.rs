use bytemuck::{Pod, Zeroable};
use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Escrow {
    pub bump: u32,
    pub seed: u32,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
}

impl Escrow {
    pub const LEN: usize = 112;
}

pub struct EscrowAccount(*const Escrow);

impl EscrowAccount {
    pub const LEN: usize = 112;

    pub fn from_account_info(account_info: &AccountInfo) -> Self {
        assert_eq!(account_info.owner(), &crate::ID);
        assert_eq!(account_info.data_len(), Self::LEN);
        unsafe {
            Self(account_info.borrow_data_unchecked().as_ptr() as *const Escrow)
        }
    }
}