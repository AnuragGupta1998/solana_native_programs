
use borsh::{BorshSerialize,BorshDeserialize};

// acount_info:- Function for fetching the next account from input accounts
// next_account_info:- Function for fetching the next account from input accounts, which is useful for iterating through the accounts passed to the program.
// AccountInfo:- Struct that contains information about an account, such as its public key, lamports, data, etc.
// entrypoint:- Macro that defines the entry point of the Solana program, which is the function that will be called when the program is invoked.
// entrypoint::ProgramResult:- Type alias for the result of a Solana program, which can be either Ok(()) for success or Err(ProgramError) for failure.
// msg:- Macro for logging messages in a Solana program, which can be useful for debugging and tracking the program's execution.
// pubkey::Pubkey:- Struct that represents a public key in Solana, which is used to identify accounts and programs on the blockchain.

use solana_program::{
    account_info::{AccountInfo, next_account_info}, 
    entrypoint,
    entrypoint::ProgramResult,
    msg, 
    pubkey::Pubkey
};


#[derive(BorshSerialize,BorshDeserialize,Debug)]
struct Counter{
    count:u32,  //state or data we want to store in the account (blockchain)   
}


#[derive(BorshSerialize,BorshDeserialize,Debug)]
enum CounterInstruction{
    Increament(u32),  //function signature for incrementing the counter by a specified value (u32)
    Decreament(u32), //function signature for decrementing the counter by a specified value (u32)
    // GetCount(), //function signature for getting the current count value (no parameters)
}

//entrypoint of the program
entrypoint!(counter_contract );

pub fn counter_contract(
    _program_id: &Pubkey,   
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    
    // let acc = next_account_info(&mut accounts.iter());
    // match acc{
    //     Ok(a) =>{
    //         msg!("Account found: {}", a.key);
    //     },
    //     Err(e)=>{
    //         msg!("Error fetching account: {}", e);
    //         return Err(solana_program::program_error::ProgramError::InvalidAccountData);
    //     }
    // }

    //fetching account in which we want to stored data using next_account_info function, which returns a Result type, so we can use ? operator to handle error if account is not found or result is not Ok 
    let acc = next_account_info(&mut accounts.iter())?; //? return error if account is not found and result is not Ok
    
    //convert acc.data from "bytes" to "Counter" struct using borsh deserialization
    let mut counter_data = Counter::try_from_slice(& acc.data.borrow())?; //? return error if account data is not valid and result is not Ok
    
    //converting instruction data from "bytes" to "CounterInstruction" enum using borsh deserialization
    let instruction_type =  CounterInstruction::try_from_slice(instruction_data)?; //? return error if instruction data is not valid and result is not Ok 

    //match the instruction type and perform the corresponding operation on the counter data
    match instruction_type{
        CounterInstruction::Increament(value) => {
            counter_data.count += value; //increment the counter by the specified value
            msg!("Counter incremented by {}. New count: {}", value, counter_data.count); 
        },
        CounterInstruction::Decreament(value)=>{
            counter_data.count -= value; //decrement the counter by the specified value
            msg!("Counter decremented by {}. New count: {}", value, counter_data.count);
        },
        //  
    }

    //convert the updated counter data back to "bytes" and store it in the account data using borsh serialization
    counter_data.serialize(&mut *acc.data.borrow_mut())?; //serialize the updated counter data back into bytes and store it in the account data, ? return error if serialization fails and result is not Ok
    
    msg!("Counter updated successfully. Current count: {}", counter_data.count); //log a message indicating that the counter has been updated successfully and display the current count
    //return Ok(()) to indicate that the program executed successfully without any errors
    Ok(())

}