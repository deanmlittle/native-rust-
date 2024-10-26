use solana_program::pubkey::Pubkey;
pub struct Mint(*const u8);

impl Mint {
    pub const LEN: usize = 82;

    pub unsafe fn has_mint_authority(&self) -> bool {
        *(self.0 as *const bool)
    }

    pub unsafe fn mint_authority(&self) -> Pubkey {
        *(self.0.add(4) as *const Pubkey)    
    }

    pub unsafe fn supply(&self) -> u64 {
        *(self.0.add(36) as *const u64)
    }

    pub unsafe fn decimals(&self) -> u8 {
        *(self.0.add(44) as *const u8)
    }

    pub unsafe fn is_frozen(&self) -> bool {
        *(self.0.add(45) as *const bool)
    }

    pub unsafe fn has_freeze_authority(&self) -> bool {
        *(self.0.add(46) as *const bool)
    }

    pub unsafe fn freeze_authority(&self) -> Pubkey {
        *(self.0.add(50) as *const Pubkey)
    }
}