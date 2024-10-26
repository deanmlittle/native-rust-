#[cfg(test)]
mod tests {
    use mollusk_svm::Mollusk;
    use solana_sdk::{
        account::{AccountSharedData, ReadableAccount}, entrypoint::ProgramResult, instruction::{AccountMeta, Instruction}, pubkey::Pubkey
    };

    #[test]
    fn withdraw() {
        let program_id = Pubkey::new_from_array([
            0x7b, 0x07, 0x5a, 0x4f, 0xca, 0x15, 0x61, 0x6e, 
            0xbe, 0x53, 0xc1, 0xa8, 0x43, 0x6f, 0x42, 0x89, 
            0x2b, 0x02, 0x1a, 0xb6, 0x62, 0x5a, 0x2a, 0x02, 
            0x2a, 0x68, 0x9a, 0xef, 0xbd, 0xed, 0x26, 0xef
        ]);

        let signer = Pubkey::new_unique();
        let (vault, bump) =
            Pubkey::try_find_program_address(&[signer.as_ref()], &program_id).unwrap();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[&1_000_000_000u64.to_le_bytes()[..], &[bump]].concat(),
            vec![
                AccountMeta::new(signer, true),
                AccountMeta::new(vault, false),
            ],
        );

        let mollusk = Mollusk::new(&program_id, "target/deploy/native_vault");

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    signer,
                    AccountSharedData::new(0, 0, &Pubkey::default()),
                ),
                (vault, AccountSharedData::new(1_000_000_000u64, 0, &program_id)),
            ],
        );

        assert_eq!(result.get_account(&signer).unwrap().lamports(), 1_000_000_000);
        assert_eq!(result.get_account(&vault).unwrap().lamports(), 0);

        assert!(!result.program_result.is_err());
    }
}
