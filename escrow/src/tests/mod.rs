#[cfg(test)]
mod tests {
    use mollusk_svm::Mollusk;
    use solana_sdk::{
        account::{AccountSharedData, ReadableAccount}, entrypoint::ProgramResult, instruction::{AccountMeta, Instruction}, pubkey::Pubkey
    };

    #[test]
    fn make() {
        let program_id = Pubkey::new_from_array([
            
        ]);

        let mollusk = Mollusk::new(&program_id, "target/deploy/native_vault");

        // todo: add spl_token program 

        let seed: u32 = u32::MAX;

        let maker = Pubkey::new_unique();
        let (escrow, bump) =
            Pubkey::try_find_program_address(&[seed, maker.as_ref()], &program_id).unwrap();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[seed.to_le_bytes(), Pubkey::new_unique(), Pubkey::new_unique(), 1_000_000u64.to_le_bytes(), &[bump]].concat(),
            vec![
                AccountMeta::new(maker, true),
                AccountMeta::new(escrow, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    maker,
                    AccountSharedData::new(0, 0, &Pubkey::default()),
                ),
                (escrow, AccountSharedData::new(1_000_000_000u64, 0, &program_id)),
            ],
        );

        assert_eq!(result.get_account(&maker).unwrap().lamports(), 1_000_000_000);
        assert_eq!(result.get_account(&escrow).unwrap().lamports(), 0);

        assert!(!result.program_result.is_err());
    }
}
