use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

// Declare the public modules for the contract
pub mod subscription;
pub mod referral;
pub mod marketplace;
pub mod token;

// Structure for maintaining the state of reentrancy protection
#[derive(Default)]
pub struct State {
    is_processing: bool,
}

// Define the entry point for the smart contract
entrypoint!(process_instruction);

// Entry point function that handles incoming instructions
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Ensure instruction_data has the minimum required length
    if instruction_data.is_empty() {
        msg!("Error: Instruction data is empty");
        return Err(ProgramError::InvalidInstructionData);
    }

    // Parse the instruction type from the first byte
    let instruction_type = instruction_data[0];

    // Set up an iterator to validate and process each account
    let accounts_iter = &mut accounts.iter();

    // Validate that the first account is a signer for access control
    let account_info = next_account_info(accounts_iter)?;
    if !account_info.is_signer {
        msg!("Error: First account must be a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Ensure the program ID is correct
    if account_info.owner != program_id {
        msg!("Error: Account does not have the correct program ID");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Initialize and check for reentrancy
    let mut state: State = Default::default();
    if state.is_processing {
        msg!("Error: Reentrant call detected");
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    state.is_processing = true;

    // Validate all accounts in the accounts array
    for account in accounts {
        // Validate account ownership and safety checks
        validate_account(account)?;
    }

    // Detailed instruction data validation
    match instruction_type {
        0 => {
            // Mint Tokens: Validate required number of accounts and instruction data format
            if accounts.len() < 2 {
                msg!("Error: Insufficient accounts for minting tokens");
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            validate_instruction_data(instruction_data, 10)?; // Ensure data length is 10 bytes

            // Check that the payer has enough balance to pay the minting fee (assuming the payer is the first account)
            let payer_info = account_info; // Payer account is the signer
            let payer_balance = get_account_balance(payer_info)?;
            let required_fee = 1_000_000; // Example fee in lamports

            if payer_balance < required_fee {
                msg!("Error: Insufficient balance to mint tokens");
                return Err(ProgramError::InsufficientFunds);
            }

            // Proceed with the minting process
            token::mint_tokens(program_id, accounts, instruction_data)
        }
        1 => {
            // Handle Subscription: Validate required number of accounts and instruction data
            if accounts.len() < 3 {
                msg!("Error: Insufficient accounts for handling subscription");
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            validate_instruction_data(instruction_data, 12)?; // Check length for subscription data
            subscription::handle_subscription(program_id, accounts, instruction_data)
        }
        2 => {
            // Process Referral: Validate required number of accounts
            if accounts.len() < 2 {
                msg!("Error: Insufficient accounts for processing referral");
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            validate_instruction_data(instruction_data, 8)?; // Check data length for referral
            referral::process_referral(program_id, accounts, instruction_data)
        }
        3 => {
            // Handle Marketplace: Validate accounts and data
            if accounts.len() < 2 {
                msg!("Error: Insufficient accounts for marketplace logic");
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            validate_instruction_data(instruction_data, 15)?; // Check length for marketplace data
            marketplace::handle_marketplace(program_id, accounts, instruction_data)
        }
        _ => {
            msg!("Error: Invalid instruction type");
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    // Reset the reentrancy protection state
    state.is_processing = false;
    Ok(())
}

// Helper function to validate instruction data length
fn validate_instruction_data(instruction_data: &[u8], expected_length: usize) -> ProgramResult {
    if instruction_data.len() < expected_length {
        msg!(
            "Error: Instruction data length is too short. Expected at least {} bytes.",
            expected_length
        );
        return Err(ProgramError::InvalidInstructionData);
    }
    Ok(())
}

// Helper function to validate accounts
fn validate_account(account: &AccountInfo) -> ProgramResult {
    if account.data_is_empty() {
        msg!("Error: Account data is empty");
        return Err(ProgramError::InvalidAccountData);
    }
    if !account.is_writable {
        msg!("Error: Account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }
    // Additional security checks can be added here
    Ok(())
}

// Helper function to get account balance (dummy implementation for demonstration)
fn get_account_balance(account_info: &AccountInfo) -> Result<u64, ProgramError> {
    // Implement your logic to retrieve the account's balance
    // This might involve checking account data to see the lamports available
    Ok(account_info.lamports()) // Placeholder for actual balance retrieval logic
}
