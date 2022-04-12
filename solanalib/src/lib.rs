
 use solana_program::entrypoint::ProgramResult;
 use solana_program::pubkey::Pubkey;
 use  solana_program::msg;
 use solana_program::entrypoint;
 use solana_program::sysvar::slot_history::AccountInfo;
 use borsh::{BorshDeserialize, BorshSerialize};
 use solana_program::account_info::next_account_info;
entrypoint!(sqoin);

#[derive(BorshDeserialize , BorshSerialize , Debug)]
pub struct BacemAccount {
    pub data: [u8; 5]
}

pub fn sqoin(

    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instructions: &[u8]
) -> ProgramResult {

    msg!("welcome Sqoin");

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut bacemAccount = BacemAccount::try_from_slice(&account.data.borrow())?;
    
    for i in 0..5 {
        bacemAccount.data[i] = instructions[i];
    }
    bacemAccount.serialize(&mut &mut account.data.borrow_mut() [..]);

    Ok(())
}