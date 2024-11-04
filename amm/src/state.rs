use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

/// # State
///
/// -- Config --
/// > Seed: u16
/// > Authority: Option<Pubkey>
/// > MintX: Pubkey - Superfluous but useful for the UI
/// > MintY: Pubkey - Superfluous but useful for the UI
/// > VaultX: Pubkey
/// > VaultY: Pubkey
/// < MintLP: Pubkey
pub struct Config(*const u8);

impl Config {
    pub const LEN: usize = 2 + 1 + 32 + 32 + 32 + 32 + 32; 

    #[inline(always)]
    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
        unsafe { Self(account_info.borrow_data_unchecked().as_ptr()) }
    }

    pub fn from_account_info(account_info: &AccountInfo) -> Self {
        assert_eq!(account_info.data_len(), Self::LEN);
        assert_eq!(account_info.owner(), &crate::ID);
        Self::from_account_info_unchecked(account_info)
    }

    pub fn seed(&self) -> u16 {
        unsafe { *(self.0 as *const u16) }
    }

    pub fn has_update_authority(&self) -> bool {
        unsafe { *(self.0 as *const u8).add(2) == 1 }
    }

    pub fn is_immutable(&self) -> bool {
        unsafe { *(self.0 as *const u8).add(2) == 1 }
    }

    pub fn is_frozen(&self) -> bool {
        unsafe { *(self.0 as *const u8).add(2) == 2 }
    }

    pub fn update_authority(&self) -> Option<Pubkey> {
        if self.has_update_authority() {
            Some(unsafe { *(self.0 as *const [u8; 32]).add(3) })
        } else {
            None
        }
    }

    pub fn lock_authority(&self) -> Option<Pubkey> {
        if self.is_frozen() {
            Some(unsafe { *(self.0 as *const [u8; 32]).add(3) })
        } else {
            None
        }
    }

    pub fn mint_x(&self) -> Pubkey {
        unsafe { *(self.0 as *const [u8; 32]).add(35) }
    }

    pub fn mint_y(&self) -> Pubkey {
        unsafe { *(self.0 as *const [u8; 32]).add(67) }
    }

    pub fn vault_x(&self) -> Pubkey {
        unsafe { *(self.0 as *const [u8; 32]).add(99) }
    }

    pub fn vault_y(&self) -> Pubkey {
        unsafe { *(self.0 as *const [u8; 32]).add(131) }
    }

    pub fn mint_lp(&self) -> Pubkey {
        unsafe { *(self.0 as *const [u8; 32]).add(163) }
    }

}