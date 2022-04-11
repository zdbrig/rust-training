
 use solana_program::entrypoint::ProgramResult;
 use solana_program::pubkey::Pubkey;
 use  solana_program::msg;
 use solana_program::entrypoint;
 use solana_program::sysvar::slot_history::AccountInfo;
 
entrypoint!(sqoin);


pub fn sqoin(

    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instructions: &[u8]
) -> ProgramResult {

    msg!("welcome sqoin");

    Ok(())
}