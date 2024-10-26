use pinocchio::pubkey::Pubkey;

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
