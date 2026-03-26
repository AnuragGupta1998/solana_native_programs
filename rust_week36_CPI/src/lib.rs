use borsh:: {BorshSerialize, BorshDeserialize};

use solana_program ::{
    account_info::{ AccountInfo, next_account_info },
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey
};

entrypoint!(process_instruction);

#[derive(BorshSerialize,BorshDeserialize,Debug)]
struct OnChainData{
    num:u32,
}

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data:&[u8]
) -> ProgramResult {

    let mut account_iter = accounts.iter();
    let data_account = next_account_info(&mut account_iter)?;

    //converting data_account data from "bytes" to "OnChainData" struct using borsh deserialization(try_from_slice function),)
    let mut on_chain_data = OnChainData::try_from_slice(&data_account.data.borrow())?;

    if on_chain_data.num ==0 {
        on_chain_data.num = 1;
    }

    else{
        on_chain_data.num *=2;
    }

    //converting on_chain_data from "OnChainData" struct to "bytes" using borsh serialization and storing it back in data_account data
    on_chain_data.serialize(&mut *data_account.data.borrow_mut());


    Ok(())
}