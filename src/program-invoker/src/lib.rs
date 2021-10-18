use solana_program::{
    msg,
    entrypoint,
    program::invoke_signed,
    pubkey::Pubkey,
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Invoker entrypoint");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    let hello_prog_ai = next_account_info(accounts_iter)?;

    // Get the account to say hello to
    let greeted_account_ai = next_account_info(accounts_iter)?;
    
    //TODO hello_account_ai.owner = hello_prog_ai.key
    let data: Vec<u8> = Vec::new();

    let instruction = Instruction {
        program_id: *hello_prog_ai.key,
        data,
        accounts: vec![
            AccountMeta::new(*greeted_account_ai.key, false),
        ],
    };

    let hello_infos = [
        hello_prog_ai.clone(),
        greeted_account_ai.clone(),
    ];

    invoke_signed(&instruction, &hello_infos, &[&[b"hello"]])?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
