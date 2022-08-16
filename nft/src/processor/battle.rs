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

pub fn process_battle(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: BattleArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let pda_creator_info = next_account_info(account_info_iter)?; //nft creator: pda
    let admin_info = next_account_info(account_info_iter)?; //admin signer
    let monster_mint_info = next_account_info(account_info_iter)?;
    let monster_info_attacker = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let edition_info = next_account_info(account_info_iter)?;
    let battle_mint_monster_info = next_account_info(account_info_iter)?;
    let game_config_info = next_account_info(account_info_iter)?;
    let metadata_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;


    assert_monster(&program_id, &monster_mint_info, &monster_info_attacker)?;
    assert_signer(&admin_info)?;
    assert_signer(&signer_info)?;

    assert_eq_pubkey(&metadata_program_info, &mpl_token_metadata::id())?;
    assert_eq_pubkey(&token_program_info, &spl_token::id())?;
    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;


    let mut state = false;
    // todo battle logic
    let mut monster = Monster::from_account_info(monster_info_attacker)?;
    // max monster fatigue 100, need 2 per battle
    if monster.fatigue > 98 {
        return ferror!("not enough fatigue");
    }
    monster.energy = monster.calculate_energy();
    //require at least 1 energy to battle
    if monster.energy < 10000 {
        return ferror!("not enough energy");
    }
    if monster.attack < args.defense {
        //lose 
    }

    if monster.attack == args.defense {
        if args.attack > monster.defense {
            //lose
        } else {
            // no winner
        }
    }

    //default monster attack first
    if monster.attack > args.defense {
        let arg_hp_lose = monster.attack - args.defense;
        let monster_hp_lose = args.attack - monster.defense;
        let monster_round = monster.hp / monster_hp_lose;
        let args_round = args.hp / arg_hp_lose;
        if monster_round >= args_round {
            //win 
            state = true;
        } else {
            //lose
        }
    }


    //after battle logic do  mint_nft
    //monster add fatigue
    if state {
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
        let mint_args = QuickMintArgs { race: args.race, attrs: args.attrs };
        init_monster_attributes(
            &battle_mint_monster_info,
            &game_config_info,
            true,
            true,
            mint_args,
        )?;
    }

    //if need hatch then do hatch
    monster.fatigue += 2;
    monster.energy -= 10000;
    monster.last_battle_time = now_timestamp();
    monster.serialize(&mut *monster_info_attacker.try_borrow_mut_data()?)?;

    Ok(())
}
