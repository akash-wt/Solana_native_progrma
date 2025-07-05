use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

#[derive(BorshDeserialize, BorshSerialize)]
struct Counter {
    count: u32,
}

#[derive(BorshDeserialize, BorshSerialize)]
enum CounterInstruction {
    Increment(u32),
    Decrement(u32),
}

entrypoint!(counter_program);

pub fn counter_program(
    _progrma_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?;
    let mut counter = Counter::try_from_slice(&acc.data.borrow())?;

    match CounterInstruction::try_from_slice(instruction_data)? {
        CounterInstruction::Increment(amount) => {
            counter.count += amount;
        }
        CounterInstruction::Decrement(amount) => {
            counter.count -= amount;
        }
    }

    counter.serialize(&mut *acc.data.borrow_mut())?;
    msg!("Counter updated to {} ", counter.count);
    Ok(())
}


