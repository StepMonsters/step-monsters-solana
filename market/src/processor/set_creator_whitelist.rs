use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{ferror, state::*, utils::*, PREFIX};

pub fn process_set_creator_whitelist(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: SetCreatorWhitelistArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let creator_info = next_account_info(account_info_iter)?;
    let creator_data_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;
    assert_config(&program_id, &config_info)?;

    let config_account = ConfigureData::from_account_info(config_info)?;
    if config_account.authority != *signer_info.key {
        return ferror!("invalid authority");
    }

    let bump = assert_creator_data(&program_id, &creator_info, &creator_data_info)?;
    if creator_data_info.data_is_empty() {
        create_or_allocate_account_raw(
            *program_id,
            creator_data_info,
            rent_info,
            system_info,
            signer_info,
            SetCreatorWhitelistData::LEN,
            &[
                PREFIX.as_bytes(),
                program_id.as_ref(),
                creator_info.key.as_ref(),
                "creator_whitelist".as_bytes(),
                &[bump],
            ],
        )?;
    }

    let mut creator_data = SetCreatorWhitelistData::from_account_info(creator_data_info)?;
    creator_data.is_activated = args.is_activated;
    creator_data.total_supply = args.total_supply;
    creator_data.seller_fee = args.seller_fee;
    creator_data.symbol = args.symbol;
    creator_data.serialize(&mut &mut creator_data_info.data.borrow_mut()[..])?;

    Ok(())
}
