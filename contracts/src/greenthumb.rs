use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub mod subscription;
pub mod referral;
pub mod marketplace;
pub mod token;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Entry point logic for handling instructions
    match instruction_data[0] {
        0 => token::mint_tokens(program_id, accounts, instruction_data),
        1 => subscription::handle_subscription(program_id, accounts, instruction_data),
        2 => referral::process_referral(program_id, accounts, instruction_data),
        3 => marketplace::handle_marketplace(program_id, accounts, instruction_data),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

