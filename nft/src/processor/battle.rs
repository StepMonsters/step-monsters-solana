use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    sysvar,
};

use crate::{state::*, ferror, utils::*};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};

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

    let pda_bump = assert_pda_creator(&program_id, pda_creator_info)?;

    let pda_seed = [
        SEED_BATTLE.as_bytes(),
        program_id.as_ref(),
        "pda_creator".as_bytes(),
        &[pda_bump],
    ];

    let mut state = false;
    // todo battle logic
    let mut monster = Monster::from_account_info(monster_info_attacker)?;
    // max monster fatigue 100, need 10 per battle
    monster.fatigue = monster.calculate_fatigue();
    if monster.calculate_fatigue() >= 90 {
        return ferror!("not enough fatigue");
    }
    if monster.attack < args.defense {
        //lose 
    }
    
    if monster.attack == args.defense {
        if args.attack > monster.defense{
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
        let config_data = ConfigureData::from_account_info(config_info)?;
        let creators = vec![
            mpl_token_metadata::state::Creator {
                address: *pda_creator_info.key,
                verified: true,
                share: 0,
            },
            mpl_token_metadata::state::Creator {
                address: config_data.creator,
                verified: false,
                share: 100,
            },
        ];
        msg!("Create metadata");
        invoke_signed(
            &create_metadata_accounts_v2(
                *metadata_program_info.key,
                *metadata_info.key,
                *mint_info.key,
                *signer_info.key,
                *signer_info.key,
                *pda_creator_info.key, //pda must be signer
                config_data.name,
                config_data.symbol,
                config_data.uri,
                Some(creators),
                config_data.fee,
                true,
                true,
                None,
                None,
            ),
            &[
                metadata_info.clone(),
                mint_info.clone(),
                signer_info.clone(),
                metadata_program_info.clone(),
                token_program_info.clone(),
                system_info.clone(),
                rent_info.clone(),
                pda_creator_info.clone(),
            ],
            &[&pda_seed],
        )?;
        msg!("Create Master Edition");
        invoke_signed(
            &create_master_edition_v3(
                *metadata_program_info.key,
                *edition_info.key,
                *mint_info.key,
                *pda_creator_info.key,
                *signer_info.key,
                *metadata_info.key,
                *signer_info.key,
                Some(1),
            ),
            &[
                edition_info.clone(),
                mint_info.clone(),
                signer_info.clone(),
                metadata_info.clone(),
                metadata_program_info.clone(),
                token_program_info.clone(),
                system_info.clone(),
                rent_info.clone(),
                pda_creator_info.clone(),
            ],
            &[&pda_seed],
        )?;
    }

    //if need hatch then do hatch
    monster.fatigue += 10;
    monster.last_battle_time = now_timestamp();
    monster.serialize(&mut *monster_info_attacker.try_borrow_mut_data()?)?;

    Ok(())
}
