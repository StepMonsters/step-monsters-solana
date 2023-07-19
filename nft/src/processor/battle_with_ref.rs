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
use crate::utils_config::calculate_battle_receive_game_token;
use crate::utils_mint::{create_battle_history_info, mint_game_token_to_ata, mint_game_token_to_ata_with_ref};

pub fn process_battle_with_ref(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: BattleArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let signer_ata_info = next_account_info(account_info_iter)?;
    let father_info = next_account_info(account_info_iter)?;
    let father_ata_info = next_account_info(account_info_iter)?;
    let grandfather_info = next_account_info(account_info_iter)?;
    let grandfather_ata_info = next_account_info(account_info_iter)?;
    let father_ref_info = next_account_info(account_info_iter)?;
    let grandfather_ref_info = next_account_info(account_info_iter)?;

    let game_token_info = next_account_info(account_info_iter)?;
    let token_admin_info = next_account_info(account_info_iter)?;
    let ass_token_program_info = next_account_info(account_info_iter)?;

    let _config_info = next_account_info(account_info_iter)?;
    let _pda_creator_info = next_account_info(account_info_iter)?;
    let monster_mint_info = next_account_info(account_info_iter)?;
    let monster_info_attacker = next_account_info(account_info_iter)?;

    let _mint_info = next_account_info(account_info_iter)?;
    let _mint_ata_info = next_account_info(account_info_iter)?;
    let _metadata_info = next_account_info(account_info_iter)?;
    let _edition_info = next_account_info(account_info_iter)?;

    let _battle_mint_monster_info = next_account_info(account_info_iter)?;
    let _game_config_info = next_account_info(account_info_iter)?;
    let battle_history_info = next_account_info(account_info_iter)?;

    let metadata_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    let admin_fund_info = next_account_info(account_info_iter);

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
    monster.energy -= 10000;

    //battle
    let (mut win, history) = battle_round(monster.clone(), args.clone());

    //after battle logic do  mint_nft
    //monster add fatigue
    let capture = get_random_u8(0, 20)? == 0;
    if win > 0 && capture {
        win = 2;
    } else if win > 0 && !capture {
        win = 1;
    } else {
        win = 0;
    }

    msg!("Receive Game Token");
    let mut token: u64 = calculate_battle_receive_game_token(win, monster.race, monster.level);
    token = token * 90 / 100;
    if signer_ata_info.lamports() <= 0 {
        send_fund_to_target(program_id, admin_fund_info.as_ref().cloned(), &signer_info, MAX_ASSOCIATED_TOKEN_ACCOUNT_LENGTH)?;
    }
    mint_game_token_to_ata(
        program_id,
        signer_info,
        signer_ata_info,
        game_token_info,
        token_admin_info,
        ass_token_program_info,
        token_program_info,
        system_info,
        token,
    )?;

    msg!("Father Token");
    if signer_ata_info.key.to_string() != father_ata_info.key.to_string() {
        let father_token = token * 500 / 10000;
        if father_ata_info.lamports() <= 0 {
            send_fund_to_target(program_id, admin_fund_info.as_ref().cloned(), &signer_info, MAX_ASSOCIATED_TOKEN_ACCOUNT_LENGTH)?;
        }
        mint_game_token_to_ata_with_ref(
            program_id,
            signer_info,
            father_info,
            father_ata_info,
            game_token_info,
            token_admin_info,
            ass_token_program_info,
            token_program_info,
            system_info,
            father_token,
        )?;
        let mut father_ref = ReferralInfo::from_account_info(father_ref_info)?;
        father_ref.reward += father_token;
        father_ref.serialize(&mut *father_ref_info.try_borrow_mut_data()?)?;
    }

    msg!("Grandfather Token");
    if signer_ata_info.key.to_string() != grandfather_ata_info.key.to_string() {
        let grandfather_token = token * 200 / 10000;
        if grandfather_ata_info.lamports() <= 0 {
            send_fund_to_target(program_id, admin_fund_info.as_ref().cloned(), &signer_info, MAX_ASSOCIATED_TOKEN_ACCOUNT_LENGTH)?;
        }
        mint_game_token_to_ata_with_ref(
            program_id,
            signer_info,
            grandfather_info,
            grandfather_ata_info,
            game_token_info,
            token_admin_info,
            ass_token_program_info,
            token_program_info,
            system_info,
            grandfather_token,
        )?;
        if grandfather_ref_info.key != father_ref_info.key {
            let mut grandfather_ref = ReferralInfo::from_account_info(grandfather_ref_info)?;
            grandfather_ref.reward += grandfather_token;
            grandfather_ref.serialize(&mut *grandfather_ref_info.try_borrow_mut_data()?)?;
        }
    }

    if battle_history_info.lamports() <= 0 {
        send_fund_to_target(program_id, admin_fund_info.as_ref().cloned(), &signer_info, MAX_BATTLE_HISTORY_LENGTH)?;
        create_battle_history_info(
            &program_id,
            &battle_history_info,
            &rent_info,
            &system_info,
            &signer_info,
            false,
        )?;
    };
    let mut battle_history = BattleHistory::from_account_info(battle_history_info)?;
    battle_history.win = win;
    battle_history.token = token;
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

    if win > 1 {
        let mut all = battle_history.bodies.clone();
        let mut body = Vec::new();
        body.push(args.race);
        body.push(args.level);
        body.push(args.gender);
        body.append(&mut args.enemy_feature.clone());
        all.push(body);
        battle_history.bodies = all.clone();
    }

    battle_history.serialize(&mut *battle_history_info.try_borrow_mut_data()?)?;

    //if need hatch then do hatch
    monster.fatigue += 2;
    monster.last_battle_time = now_timestamp();
    monster.serialize(&mut *monster_info_attacker.try_borrow_mut_data()?)?;

    //send fund
    send_fund_to_target(program_id, admin_fund_info.as_ref().cloned(), &signer_info, 0)?;

    Ok(())
}
