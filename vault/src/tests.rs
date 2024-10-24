#[cfg(test)]
mod tests {
    use mollusk_svm::{program, Mollusk};
    use solana_sdk::{instruction::AccountMeta, pubkey, account::AccountSharedData, instruction::Instruction, pubkey::Pubkey};

    #[test]
    fn withdraw() {
        let program_id = pubkey!("9HFegTZnvebYjf9kSa6k3WBm93hRfogWB5B1goUrq1oL");

        let signer = Pubkey::new_unique();
        let (vault, bump) = Pubkey::try_find_program_address(&[signer.as_ref()], &program_id)
            .unwrap();
        let (system_program, system_program_account) = program::keyed_account_for_system_program();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[&100000u64.to_le_bytes()[..], &[bump]].concat(),
            vec![
                AccountMeta::new(signer, true),
                AccountMeta::new(vault, false),
                AccountMeta::new_readonly(system_program, false),
            ],
        );

        let mollusk = Mollusk::new(&program_id, "target/deploy/vault");

        let _: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (signer, AccountSharedData::new(1_000_000, 0, &Pubkey::default())),
                (
                    vault,
                    AccountSharedData::new(1_000_000_000, 0, &program_id),
                ),
                (system_program, system_program_account),
            ],
        );
    }
}