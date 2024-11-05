use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Declare the public modules for the contract
pub mod subscription;
pub mod referral;
pub mod marketplace;
pub mod token;

// Define the entry point for the smart contract
entrypoint!(process_instruction);

// Entry point function that handles incoming instructions
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Determine which module to call based on the first byte of instruction_data
    match instruction_data[0] {
        0 => token::mint_tokens(program_id, accounts, instruction_data),
        1 => subscription::handle_subscription(program_id, accounts, instruction_data),
        2 => referral::process_referral(program_id, accounts, instruction_data),
        3 => marketplace::handle_marketplace(program_id, accounts, instruction_data),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
