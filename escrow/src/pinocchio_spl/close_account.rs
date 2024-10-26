use pinocchio::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction, Signer},
    program::invoke_signed,
};

/// Close token account.
///
/// ### Accounts:
///   0. `[WRITE]` Account to close
///   1. `[WRITE]` Destination account
///   2. `[SIGNER]` Authority account
pub struct CloseAccount<'a> {
    /// Sender account.
    pub from: &'a AccountInfo,

    /// Recipient account.
    pub to: &'a AccountInfo,

    /// Authority account.
    pub authority: &'a AccountInfo,
}

impl<'a> CloseAccount<'a> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMeta; 3] = [
            AccountMeta::writable(self.from.key()),
            AccountMeta::writable(self.to.key()),
            AccountMeta::readonly_signer(self.authority.key()),
        ];

        let instruction = Instruction {
            program_id: &super::ID,
            accounts: &account_metas,
            data: &[],
        };

        invoke_signed(&instruction, &[self.from, self.to, self.authority], signers)
    }
}
