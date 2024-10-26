use solana_program::pubkey::Pubkey;

use super::AccountState;

pub struct TokenAccount(*const u8);

impl TokenAccount {
    pub const LEN: usize = 165;

    pub unsafe fn mint(&self) -> Pubkey {
        *(self.0 as *const Pubkey)
    }

    pub unsafe fn owner(&self) -> Pubkey {
        *(self.0.add(32) as *const Pubkey)
    }

    pub unsafe fn amount(&self) -> u64 {
        *(self.0.add(64) as *const u64)
    }

    pub unsafe fn has_delegate(&self) -> bool {
        *(self.0.add(72) as *const bool)
    }

    pub unsafe fn delegate(&self) -> Pubkey {
        *(self.0.add(76) as *const Pubkey)
    }

    pub unsafe fn optional_delegate(&self) -> Option<Pubkey> {
        if self.has_delegate() {
            Some(self.delegate())
        } else {
            None
        }
    }

    pub unsafe fn state(&self) -> AccountState {
        *(self.0.add(108) as *const AccountState)
    }

    pub unsafe fn is_native(&self) -> bool {
        *(self.0.add(109) as *const bool)
    }

    pub unsafe fn native_amount(&self) -> u64 {
        *(self.0.add(113) as *const u64)
    }

    pub unsafe fn optional_native_amount(&self) -> Option<u64> {
        if self.is_native() {
            Some(self.native_amount())
        } else {
            None
        }
    }

    pub unsafe fn delegated_amount(&self) -> u64 {
        *(self.0.add(121) as *const u64)
    }

    pub unsafe fn has_close_authority(&self) -> bool {
        *(self.0.add(129) as *const bool)
    }

    pub unsafe fn close_authority(&self) -> Pubkey {
        *(self.0.add(133) as *const Pubkey)
    }

    pub unsafe fn optional_close_authority(&self) -> Option<Pubkey> {
        if self.has_close_authority() {
            Some(self.close_authority())
        } else {
            None
        }
    }

}