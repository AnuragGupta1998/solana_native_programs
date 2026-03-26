use borsh::{BorshDeserialize, BorshSerialize,};
use solana_program::{
    account_info ::{AccountInfo,next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    system_instruction::create_account,
}

entrypoint!(process_instruction);

// PDA creation in rust using SystemProgram::create_account
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    //create new PDA account onchain using SystemProgram::create_account
    let iter = &mut accounts.iter();

    let pda_account = next_account_info(&mut iter)?;
    let user_account = next_account_info(&mut iter)?;
    let system_program_account = next_account_info(&mut iter)?;
    
    //as.ref() is used to convert the string literal "pda" into a byte slice, which is necessary for the seeds used in the invoke_signed function. The seeds are used to derive the PDA (Program Derived Address) and must be in byte format.
    let seeds = &[user_account.key.as_ref(),b"pda"];

    //find the PDA public key and bump using the seeds and program id that is not on the ed25519 curve, which is a requirement for PDAs in Solana. The find_program_address function returns the PDA public key and the bump seed, which is used to ensure that the derived address is valid and does not collide with any existing accounts.
    let {pda_public_key,bump} = PublicKey::find_program_adress(seeds, _program_id);
    
    let instruction = create_account({
        from_pubkey: user_account.key,
        to_pubkey: pda_account.key,
        lamports: 1000_000_000, // 1 SOL
        space:8,
        owner: _program_id ,
    });

    //3D array seeds is used to pass the seeds and bump to the invoke_signed function, which is necessary for signing the transaction with the PDA's private key. The seeds and bump are used to derive the PDA's public key, and the invoke_signed function uses this information to sign the transaction on behalf of the PDA.
    invoke_signed(&instruction,accounts,&[&[seeds,&[bump]]])?;

 
    Ok(())  
}