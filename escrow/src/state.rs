use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

/// # State
///
/// -- Data --
/// > Maker: Pubkey
/// > MakerTaB: Pubkey
/// > MintA: Pubkey
/// > MintB: Pubkey
/// > AmountB: u64
///
/// -- Data Logic --
/// [...]
///
pub struct Escrow(*const u8);

impl Escrow {
    pub const LEN: usize = 136;

    #[inline(always)]
    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
        unsafe { Self(account_info.borrow_data_unchecked().as_ptr()) }
    }

    pub fn from_account_info(account_info: &AccountInfo) -> Self {
        assert_eq!(account_info.data_len(), Self::LEN);
        assert_eq!(account_info.owner(), &crate::ID);
        Self::from_account_info_unchecked(account_info)
    }

    pub fn maker(&self) -> Pubkey {
        unsafe { *(self.0 as *const Pubkey) }
    }

    pub fn maker_ta_b(&self) -> Pubkey {
        unsafe { *(self.0.add(32) as *const Pubkey) }
    }

    pub fn mint_a(&self) -> Pubkey {
        unsafe { *(self.0.add(64) as *const Pubkey) }
    }

    // Mint B is only used for RPC lookups, so we can comment this out!
    // pub fn mint_b(&self) -> Pubkey {
    //     unsafe { *(self.0.add(96) as *const Pubkey) }
    // }

    pub fn amount_b(&self) -> u64 {
        unsafe { *(self.0.add(128) as *const u64) }
    }
}
