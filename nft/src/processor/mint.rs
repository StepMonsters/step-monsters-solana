use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar,
};

use crate::{ferror, state::*, utils::*};
use crate::utils_mint::{create_metadata_edition, create_monster_info};

pub fn process_mint(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: MintArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let _config_info = next_account_info(account_info_iter)?;
    let pda_creator_info = next_account_info(account_info_iter)?; //nft creator: pda
    let fee_receiver_info = next_account_info(account_info_iter)?; // fee_receiver: wallet
    let mint_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let edition_info = next_account_info(account_info_iter)?;
    let monster_info = next_account_info(account_info_iter)?;

    let metadata_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    //check authority
    let config_info = next_account_info(account_info_iter)?;
    let config_data = ConfigureData::from_account_info(config_info)?;
    if config_data.authority != *signer_info.key {
        return ferror!("invalid authority");
    }

    msg!("Assert Public Key");
    assert_eq_pubkey(&metadata_program_info, &mpl_token_metadata::id())?;
    assert_eq_pubkey(&token_program_info, &spl_token::id())?;
    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;
    assert_config(&program_id, &config_info)?;

    msg!("Assert Fee Receiver");
    let mut config_data = ConfigureData::from_account_info(config_info)?;
    assert_eq_pubkey(&fee_receiver_info, &config_data.fee_receiver)?;

    msg!("Check Config Initialized");
    if !config_data.is_initialized {
        return ferror!("invalid mint state");
    }

    msg!("Transfer Mint Fee");
    if config_data.price > 0 {
        invoke(
            &system_instruction::transfer(&signer_info.key, &config_data.fee_receiver, config_data.price),
            &[
                signer_info.clone(),
                fee_receiver_info.clone(),
                system_info.clone(),
            ],
        )?;
    }

    msg!("Create Metadata Edition");
    create_metadata_edition(
        &program_id,
        &pda_creator_info,
        config_data.clone(),
        &signer_info,
        &mint_info,
        &metadata_info,
        &edition_info,
        &metadata_program_info,
        &token_program_info,
        &system_info,
        &rent_info,
    )?;
    config_data.current_id += 1;
    config_data.serialize(&mut *config_info.try_borrow_mut_data()?)?;

    msg!("Create Monster Info");
    create_monster_info(
        &program_id,
        &monster_info,
        &mint_info,
        &rent_info,
        &system_info,
        &signer_info,
    )?;
    let mut monster = Monster::from_account_info(monster_info)?;
    monster.race = args.race;
    monster.generation = args.generation;
    monster.monster_feature = Vec::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    monster.father_mint = args.father_mint;
    monster.mother_mint = args.mother_mint;
    monster.serialize(&mut *monster_info.try_borrow_mut_data()?)?;

    Ok(())
}
