use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar,
};

use crate::{ferror, state::*, utils::*};
use crate::utils_mint::{create_metadata_edition, create_monster_info, init_monster_attributes};

pub fn process_revive(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: ReviveArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let pda_creator_info = next_account_info(account_info_iter)?;
    let fee_receiver_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let edition_info = next_account_info(account_info_iter)?;
    let monster_info = next_account_info(account_info_iter)?;
    let game_config_info = next_account_info(account_info_iter)?;
    let battle_history_info = next_account_info(account_info_iter)?;
    let collection_info = next_account_info(account_info_iter)?;
    let collection_metadata_info = next_account_info(account_info_iter)?;
    let collection_edition_info = next_account_info(account_info_iter)?;

    let metadata_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_eq_pubkey(&metadata_program_info, &mpl_token_metadata::id())?;
    assert_eq_pubkey(&token_program_info, &spl_token::id())?;
    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;
    assert_config(&program_id, &config_info)?;

    let mut config_data = ConfigureData::from_account_info(config_info)?;
    assert_eq_pubkey(&fee_receiver_info, &config_data.fee_receiver)?;

    if !config_data.is_initialized {
        return ferror!("invalid mint state");
    }

    msg!("Revive Monster");
    let mut battle_history = BattleHistory::from_account_info(battle_history_info)?;

    //check soul
    let check = check_soul_revive(args.clone());
    if check != args.soul {
        return ferror!("Invalid soul.");
    };
    if check > battle_history.soul {
        return ferror!("Insufficient soul.");
    }
    battle_history.soul -= check;

    //check args
    let index = args.index;
    let mut bodies = battle_history.bodies.clone();
    let body = &bodies[index as usize];
    if body[0] == args.race &&
        body[1] == args.level &&
        body[2] == args.gender &&
        check_body_array(body.clone(), args.enemy_feature.clone()) {
        bodies.remove(index as usize);
        battle_history.bodies = bodies;
    } else {
        return ferror!("Invalid body.");
    }
    battle_history.serialize(&mut *battle_history_info.try_borrow_mut_data()?)?;

    //mint args
    let mint_args = QuickMintArgs { race: args.race.clone(), attrs: args.enemy_feature.clone() };

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
        &collection_info,
        &collection_metadata_info,
        &collection_edition_info,
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

    msg!("Init Monster Attributes");
    let mut monster = Monster::from_account_info(monster_info)?;
    let init_attrs = init_monster_attributes(
        monster.clone(),
        &game_config_info,
        true,
        true,
        mint_args,
    )?;
    monster = init_attrs.clone();
    monster.serialize(&mut *monster_info.try_borrow_mut_data()?)?;

    Ok(())
}
