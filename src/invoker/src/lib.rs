use solana_program::{
    msg,
    entrypoint,
    program::invoke,
    program::invoke_signed,
    pubkey::Pubkey,
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

pub mod HelloId {
    use solana_program::declare_id;
    declare_id!("FaeesaSwyFsh6Q2WLac68fpQAudNCrkFpwM38cAojQuj");
}

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Invoker entrypoint");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // hello program account
    let hello_prog_ai = next_account_info(accounts_iter)?;

    // Get the account to say hello to
    let greeted_account_ai = next_account_info(accounts_iter)?;

    // Get the PDA account
    let pda_signer_ai = next_account_info(accounts_iter)?;

    // Check if the public key of hello account equals to hello program Id
    // Make sure that this program invokes hello program
    assert_eq!(*hello_prog_ai.key, HelloId::id());

    // Check if PDA is correct
    let pda_key = Pubkey::create_program_address(&[b"hello002"], program_id)?;
    assert_eq!(pda_key, *pda_signer_ai.key);    

    let instruction = Instruction {
        program_id: *hello_prog_ai.key,
        data: vec![],
        accounts: vec![
            AccountMeta::new(*greeted_account_ai.key, false),
            AccountMeta::new_readonly(*pda_signer_ai.key, true),
        ],
    };

    // Invoke hello program signed with PDA
    let hello_infos = [
        hello_prog_ai.clone(),
        greeted_account_ai.clone(),
        pda_signer_ai.clone(),
    ];

    invoke_signed(&instruction, &hello_infos, &[&[b"hello002"]])?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
