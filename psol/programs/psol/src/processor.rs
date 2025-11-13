use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::{
    error::PsolError,
    instruction::PsolInstruction,
    state::{NullifierSet, PrivateAccount, ShieldedPool},
    utils::verify_proof,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = PsolInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        PsolInstruction::InitializePrivateAccount => {
            msg!("Instruction: InitializePrivateAccount");
            process_initialize_private_account(program_id, accounts)
        }
        PsolInstruction::Deposit { amount } => {
            msg!("Instruction: Deposit");
            process_deposit(program_id, accounts, amount)
        }
        PsolInstruction::PrivateTransfer {
            amount_commitment,
            proof,
            nullifier,
        } => {
            msg!("Instruction: PrivateTransfer");
            process_private_transfer(
                program_id,
                accounts,
                amount_commitment,
                proof,
                nullifier,
            )
        }
        PsolInstruction::Withdraw { amount, proof } => {
            msg!("Instruction: Withdraw");
            process_withdraw(program_id, accounts, amount, proof)
        }
        PsolInstruction::RevealBalance => {
            msg!("Instruction: RevealBalance");
            process_reveal_balance(program_id, accounts)
        }
    }
}

fn process_initialize_private_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let owner_info = next_account_info(account_info_iter)?;
    let private_account_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;

    if !owner_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let rent = Rent::get()?;
    let required_lamports = rent.minimum_balance(PrivateAccount::LEN);

    // Create account
    invoke(
        &system_instruction::create_account(
            owner_info.key,
            private_account_info.key,
            required_lamports,
            PrivateAccount::LEN as u64,
            program_id,
        ),
        &[
            owner_info.clone(),
            private_account_info.clone(),
            system_program_info.clone(),
        ],
    )?;

    // Initialize private account
    let mut private_account = PrivateAccount::new(*owner_info.key);
    private_account.serialize(&mut &mut private_account_info.data.borrow_mut()[..])?;

    msg!("Private account initialized");
    Ok(())
}

fn process_deposit(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let owner_info = next_account_info(account_info_iter)?;
    let source_account_info = next_account_info(account_info_iter)?;
    let private_account_info = next_account_info(account_info_iter)?;
    let shielded_pool_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;

    if !owner_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Verify private account ownership
    let mut private_account =
        PrivateAccount::try_from_slice(&private_account_info.data.borrow())?;
    
    if private_account.owner != *owner_info.key {
        return Err(PsolError::InvalidAccountOwner.into());
    }

    // Transfer tokens to shielded pool
    let transfer_instruction = spl_token::instruction::transfer(
        token_program_info.key,
        source_account_info.key,
        shielded_pool_info.key,
        owner_info.key,
        &[],
        amount,
    )?;

    invoke(
        &transfer_instruction,
        &[
            source_account_info.clone(),
            shielded_pool_info.clone(),
            owner_info.clone(),
            token_program_info.clone(),
        ],
    )?;

    // Update private account commitment (simplified)
    private_account.nonce += 1;
    private_account.serialize(&mut &mut private_account_info.data.borrow_mut()[..])?;

    msg!("Deposited {} tokens to private account", amount);
    Ok(())
}

fn process_private_transfer(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount_commitment: [u8; 32],
    proof: Vec<u8>,
    nullifier: [u8; 32],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let sender_authority_info = next_account_info(account_info_iter)?;
    let sender_account_info = next_account_info(account_info_iter)?;
    let receiver_account_info = next_account_info(account_info_iter)?;
    let nullifier_set_info = next_account_info(account_info_iter)?;

    if !sender_authority_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Verify sender account ownership
    let mut sender_account =
        PrivateAccount::try_from_slice(&sender_account_info.data.borrow())?;
    
    if sender_account.owner != *sender_authority_info.key {
        return Err(PsolError::InvalidAuthority.into());
    }

    // Check nullifier hasn't been used
    let mut nullifier_set =
        NullifierSet::try_from_slice(&nullifier_set_info.data.borrow())?;
    
    if nullifier_set.contains(&nullifier) {
        return Err(PsolError::DoubleSpend.into());
    }

    // Verify zero-knowledge proof
    if !verify_proof(&proof, &amount_commitment) {
        return Err(PsolError::InvalidProof.into());
    }

    // Update accounts
    sender_account.nonce += 1;
    sender_account.serialize(&mut &mut sender_account_info.data.borrow_mut()[..])?;

    let mut receiver_account =
        PrivateAccount::try_from_slice(&receiver_account_info.data.borrow())?;
    receiver_account.nonce += 1;
    receiver_account.serialize(&mut &mut receiver_account_info.data.borrow_mut()[..])?;

    // Add nullifier to set
    nullifier_set.insert(nullifier)?;
    nullifier_set.serialize(&mut &mut nullifier_set_info.data.borrow_mut()[..])?;

    msg!("Private transfer completed");
    Ok(())
}

fn process_withdraw(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
    proof: Vec<u8>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let authority_info = next_account_info(account_info_iter)?;
    let private_account_info = next_account_info(account_info_iter)?;
    let destination_account_info = next_account_info(account_info_iter)?;
    let shielded_pool_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;

    if !authority_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Verify private account ownership
    let mut private_account =
        PrivateAccount::try_from_slice(&private_account_info.data.borrow())?;
    
    if private_account.owner != *authority_info.key {
        return Err(PsolError::InvalidAuthority.into());
    }

    // Verify withdrawal proof
    if !verify_proof(&proof, &[0u8; 32]) {
        return Err(PsolError::InvalidProof.into());
    }

    // Transfer tokens from shielded pool
    let transfer_instruction = spl_token::instruction::transfer(
        token_program_info.key,
        shielded_pool_info.key,
        destination_account_info.key,
        authority_info.key,
        &[],
        amount,
    )?;

    invoke(
        &transfer_instruction,
        &[
            shielded_pool_info.clone(),
            destination_account_info.clone(),
            authority_info.clone(),
            token_program_info.clone(),
        ],
    )?;

    // Update private account
    private_account.nonce += 1;
    private_account.serialize(&mut &mut private_account_info.data.borrow_mut()[..])?;

    msg!("Withdrew {} tokens from private account", amount);
    Ok(())
}

fn process_reveal_balance(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let authority_info = next_account_info(account_info_iter)?;
    let private_account_info = next_account_info(account_info_iter)?;

    if !authority_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let private_account =
        PrivateAccount::try_from_slice(&private_account_info.data.borrow())?;
    
    if private_account.owner != *authority_info.key {
        return Err(PsolError::InvalidAuthority.into());
    }

    msg!("Balance commitment: {:?}", private_account.balance_commitment);
    msg!("Account nonce: {}", private_account.nonce);
    
    Ok(())
}
