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
use crate::utils_battle::battle_round;
use crate::utils_mint::{create_battle_history_info, create_metadata_edition, create_monster_info, init_monster_attributes};

pub fn process_battle(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: BattleArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let pda_creator_info = next_account_info(account_info_iter)?; //nft creator: pda
    let monster_mint_info = next_account_info(account_info_iter)?;
    let monster_info_attacker = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let edition_info = next_account_info(account_info_iter)?;
    let battle_mint_monster_info = next_account_info(account_info_iter)?;
    let game_config_info = next_account_info(account_info_iter)?;
    let battle_history_info = next_account_info(account_info_iter)?;
    let metadata_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_monster(&program_id, &monster_mint_info, &monster_info_attacker)?;
    assert_signer(&signer_info)?;

    assert_eq_pubkey(&metadata_program_info, &mpl_token_metadata::id())?;
    assert_eq_pubkey(&token_program_info, &spl_token::id())?;
    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;

    //monster attributes
    let mut monster = Monster::from_account_info(monster_info_attacker)?;

    //max monster fatigue 100 and need 2 per battle
    if monster.fatigue > 98 {
        return ferror!("not enough fatigue");
    }

    //require at least 1 energy to battle
    monster.energy = monster.calculate_energy();
    if monster.energy < 10000 {
        return ferror!("not enough energy");
    }

    //battle
    let (mut win, history) = battle_round(monster.clone(), args.clone());

    //after battle logic do  mint_nft
    //monster add fatigue
    let capture = get_random_u8(0, 10)? == 0;
    if win > 0 && capture {
        win = 2;
    } else if win > 0 && !capture {
        win = 1;
    } else {
        win = 0;
    }
    if win == 2 {
        let mut config_data = ConfigureData::from_account_info(config_info)?;
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
            &battle_mint_monster_info,
            &mint_info,
            &rent_info,
            &system_info,
            &signer_info,
        )?;

        msg!("Init Monster Attributes");
        let mint_args = QuickMintArgs { race: args.race.clone(), attrs: args.enemy_feature.clone() };
        init_monster_attributes(
            &battle_mint_monster_info,
            &game_config_info,
            true,
            true,
            mint_args,
        )?;
    }

    if battle_history_info.lamports() <= 0 {
        create_battle_history_info(
            &program_id,
            &battle_history_info,
            &rent_info,
            &system_info,
            &signer_info,
        )?;
    };
    let mut battle_history = BattleHistory::from_account_info(battle_history_info)?;
    battle_history.win = win;
    battle_history.me_race = monster.race;
    battle_history.me_hp = monster.hp;
    battle_history.me_attack = monster.attack;
    battle_history.me_defense = monster.defense;
    battle_history.me_speed = monster.speed;
    battle_history.me_agility = monster.agility;

    battle_history.enemy_hp = args.hp;
    battle_history.enemy_attack = args.attack;
    battle_history.enemy_defense = args.defense;
    battle_history.enemy_speed = args.speed;
    battle_history.enemy_agility = args.agility;

    battle_history.me_feature = monster.monster_feature.clone();
    battle_history.enemy_feature = args.enemy_feature.clone();
    battle_history.history = history;
    battle_history.serialize(&mut *battle_history_info.try_borrow_mut_data()?)?;

    //if need hatch then do hatch
    monster.fatigue += 2;
    monster.energy = 30000;
    monster.last_battle_time = now_timestamp();
    monster.serialize(&mut *monster_info_attacker.try_borrow_mut_data()?)?;

    Ok(())
}
