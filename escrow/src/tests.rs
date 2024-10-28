#[cfg(test)]
mod tests {
    use std::{mem, u64};

    use mollusk_svm::{
        program::{self, create_program_account_loader_v3},
        Mollusk,
    };

    use solana_sdk::{
        account::{AccountSharedData, WritableAccount}, instruction::{AccountMeta, Instruction}, program_option::COption, program_pack::Pack, pubkey::Pubkey, system_program,
    };
    use spl_token::state::AccountState;

    use crate::state::Escrow;

    #[test]
    fn make() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mollusk = Mollusk::new(&program_id, "target/deploy/native_escrow");

        let maker = Pubkey::new_unique();
        let escrow = Pubkey::new_unique();
        let maker_ta_b = Pubkey::new_unique();
        let mint_a = Pubkey::new_unique();
        let mint_b = Pubkey::new_unique();
        
        let (system_program, system_program_account) = program::keyed_account_for_system_program();

        let data = [vec![0], maker_ta_b.to_bytes().to_vec(), mint_a.to_bytes().to_vec(), mint_b.to_bytes().to_vec(), 1_000_000u64.to_le_bytes().to_vec()].concat();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(maker, true),
                AccountMeta::new(escrow, true), // It should be a signer because this account shouldn't exist yet
                AccountMeta::new_readonly(system_program, false)
            ],
        );

        let lamports= mollusk.sysvars.rent.minimum_balance(136);

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (maker, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (escrow, AccountSharedData::new(lamports, 136, &program_id)),
                (system_program, system_program_account)
            ],
        );

        assert!(!result.program_result.is_err());
    }

    #[test]
    fn refund() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mut mollusk = Mollusk::new(&program_id, "target/deploy/native_escrow");

        mollusk.add_program(&spl_token::ID, "src/spl_token-3.5.0", &mollusk_svm::program::loader_keys::LOADER_V3);
        let (token_program, token_program_account) = (spl_token::ID, program::create_program_account_loader_v3(&spl_token::ID));
        let (system_program, system_program_account) = program::keyed_account_for_system_program();

        // Accounts
        let maker = Pubkey::new_unique();
        let maker_ta_a = Pubkey::new_unique();
        let escrow = Pubkey::new_unique();
        let vault = Pubkey::new_unique();
        let (authority, bump) =
            Pubkey::try_find_program_address(&[escrow.as_ref()], &program_id).unwrap();
        let mint_a = Pubkey::new_unique();
        let maker_ta_b = Pubkey::new_unique();
        let mint_b = Pubkey::new_unique();
       
        // Fill out our account data
        let mut mint_a_account = AccountSharedData::new(
            u64::MAX,
            spl_token::state::Mint::LEN,
            &token_program,
        );
        solana_program::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::None,
                supply: 100_000_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_a_account.data_as_mut_slice(),
        ).unwrap();

        let mut maker_ta_a_account = AccountSharedData::new(
            u64::MAX,
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_program::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: mint_a,
                owner: maker,
                amount: 0,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            maker_ta_a_account.data_as_mut_slice(),
        ).unwrap();

        let mut vault_account = AccountSharedData::new(
            u64::MAX,
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_program::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: mint_a,
                owner: authority,
                amount: 1_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            maker_ta_a_account.data_as_mut_slice(),
        ).unwrap();

        let mut escrow_account = AccountSharedData::new(
            u64::MAX,
            mem::size_of::<Escrow>(),
            &program_id,
        );
        let escrow_data = [maker.to_bytes().to_vec(), maker_ta_b.to_bytes().to_vec(), mint_a.to_bytes().to_vec(), mint_b.to_bytes().to_vec(), 1_000_000u64.to_le_bytes().to_vec()].concat();
        escrow_account.set_data_from_slice(&escrow_data);

        // Data
        let data = [1, bump];

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(maker, true),
                AccountMeta::new(maker_ta_a, false),
                AccountMeta::new(escrow, false),
                AccountMeta::new(vault, false),
                AccountMeta::new(authority, false),
                AccountMeta::new(token_program, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (maker, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (maker_ta_a, maker_ta_a_account),
                (escrow, escrow_account),
                (vault, vault_account),
                (authority, AccountSharedData::new(0, 0, &Pubkey::default())),
                (token_program, token_program_account)
            ],
        );

        assert!(!result.program_result.is_err());
    }
}
