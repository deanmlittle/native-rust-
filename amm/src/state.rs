use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

/// # State
///
/// -- Config --
/// > Authority: Flag<Pubkey>
/// > MintX: Pubkey
/// > MintY: Pubkey
/// > MintLP: Pubkey
/// > VaultX: Pubkey
/// > VaultY: Pubkey
/// > Fee: u16
/// > AuthorityBump: u8
pub struct Config(*const u8);

impl Config {
    pub const LEN: usize = 1 + 32 + 32 + 32 + 32 + 32 + 32 + 2 + 1;

    #[inline(always)]
    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
        unsafe { Self(account_info.borrow_data_unchecked().as_ptr()) }
    }

    pub fn from_account_info(account_info: &AccountInfo) -> Self {
        assert_eq!(account_info.data_len(), Self::LEN);
        assert_eq!(account_info.owner(), &crate::ID);
        Self::from_account_info_unchecked(account_info)
    }

    pub fn get_status(&self) -> u8 {
        unsafe { *(self.0 as *const u8) }
    }

    pub fn update_authority(&self) -> Pubkey {
        unsafe { *(self.0 as *const [u8; 32]).add(1) }
    }

    pub fn mint_x(&self) -> Pubkey {
        unsafe { *(self.0 as *const [u8; 32]).add(33) }
    }

    pub fn mint_y(&self) -> Pubkey {
        unsafe { *(self.0 as *const [u8; 32]).add(65) }
    }

    pub fn mint_lp(&self) -> Pubkey {
        unsafe { *(self.0 as *const [u8; 32]).add(97) }
    }

    pub fn vault_x(&self) -> Pubkey {
        unsafe { *(self.0 as *const [u8; 32]).add(129) }
    }

    pub fn vault_y(&self) -> Pubkey {
        unsafe { *(self.0 as *const [u8; 32]).add(161) }
    }

    pub fn fee(&self) -> u16 {
        unsafe { *(self.0 as *const u16).add(193) }
    }

    pub fn authority_bump(&self) -> u8 {
        unsafe { *(self.0 as *const u8).add(195) }
    }
}
