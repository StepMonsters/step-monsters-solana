use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{ferror, state::*, utils::*};

pub fn process_configure(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: ConfigureArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    
    let bump = assert_config(&program_id, &config_info)?;

    let mut is_created = true;
    if config_info.data_is_empty() {
        create_or_allocate_account_raw(
            *program_id,
            config_info,
            rent_info,
            system_info,
            signer_info,
            ConfigureData::LEN,
            &[
                SEED_BATTLE.as_bytes(),
                program_id.as_ref(),
                "configure".as_bytes(),
                &[bump],
            ],
        )?;
        is_created = false;
    }

    let mut config_data = ConfigureData::from_account_info(config_info)?;

    if is_created {
        if config_data.authority != *signer_info.key {
            return ferror!("invalid authority");
        }
        assert_owned_by(config_info, &program_id)?;
    }

    config_data.is_initialized = args.is_initialized;
    config_data.authority = args.authority;
    config_data.creator = args.creator;
    config_data.price = args.price;
    config_data.fee = args.fee;
    config_data.name = args.name;
    config_data.symbol = args.symbol;
    config_data.uri = args.uri;
    config_data.serialize(&mut &mut config_info.data.borrow_mut()[..])?;

    Ok(())
}
