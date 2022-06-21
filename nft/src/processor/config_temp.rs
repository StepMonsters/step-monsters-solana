use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::{state::*, utils::*};

pub fn process_configure_temp(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: ConfigTempArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    let path = &[SEED_BATTLE.as_bytes(), program_id.as_ref(), "configure_temp".as_bytes()];
    let bump = assert_derivation(&program_id, &config_info, path)?;

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
                "configure_temp".as_bytes(),
                &[bump],
            ],
        )?;
    }

    let mut config_data = ConfigTempData::from_account_info(config_info)?;
    config_data.name = args.name;
    config_data.symbol = args.symbol;
    config_data.uri = args.uri;
    config_data.serialize(&mut &mut config_info.data.borrow_mut()[..])?;

    Ok(())
}
