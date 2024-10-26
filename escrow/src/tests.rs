#[cfg(test)]
mod tests {
    use mollusk_svm::{Mollusk, program};
    use solana_sdk::{
        account::{AccountSharedData, ReadableAccount}, entrypoint::ProgramResult, instruction::{AccountMeta, Instruction}, pubkey::Pubkey
    };

    #[test]
    fn make() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mollusk = Mollusk::new(&program_id, "target/deploy/native_escrow");

        let seed: u32 = 1337;
        let maker = Pubkey::new_unique();
        let mint_a = Pubkey::new_unique();
        let mint_b = Pubkey::new_unique();
        
        let (system_program, system_program_account) = program::keyed_account_for_system_program();

        let (escrow, bump) = Pubkey::try_find_program_address(&[&seed.to_le_bytes(), maker.as_ref()], &program_id).unwrap();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[&[0], &u32::from(bump).to_le_bytes()[..], &seed.to_le_bytes()[..], mint_a.as_ref(), mint_b.as_ref(), &1_000_000u64.to_le_bytes(), &[bump]].concat(),
            vec![
                AccountMeta::new(maker, true),
                AccountMeta::new(escrow, false),
                AccountMeta::new_readonly(system_program, false)
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    maker,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (escrow, AccountSharedData::new(0, 0, &Pubkey::default())),
                (system_program, system_program_account)
            ],
        );

        println!("{:?}", result.get_account(&escrow));

        assert!(!result.program_result.is_err());
    }
}
