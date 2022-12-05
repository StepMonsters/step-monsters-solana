use borsh::BorshSerialize;
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2, update_metadata_accounts_v2};
use mpl_token_metadata::state::{Creator, DataV2, Metadata};
use mpl_token_metadata::utils::{spl_token_burn, TokenBurnParams};
use solana_program::account_info::AccountInfo;
use solana_program::msg;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::state::{ConfigureData, GameConfig, MAX_BATTLE_HISTORY_LENGTH, MAX_MONSTER_LENGTH, Monster, now_timestamp, QuickMintArgs, SEED_BATTLE, SEED_BATTLE_HISTORY, SEED_MONSTER};
use crate::utils::{assert_derivation, assert_pda_creator, create_or_allocate_account_raw, get_random_u8};
use crate::utils_config::{get_monster_feature_by_index, handle_monster_feature_config};

pub fn create_monster_info<'a>(
    program_id: &Pubkey,
    monster_info: &AccountInfo<'a>,
    mint_info: &AccountInfo<'a>,
    rent_info: &AccountInfo<'a>,
    system_info: &AccountInfo<'a>,
    signer_info: &AccountInfo<'a>,
) -> Result<(), ProgramError> {
    let bump_seed = assert_derivation(
        program_id,
        monster_info,
        &[
            SEED_MONSTER.as_bytes(),
            program_id.as_ref(),
            &mint_info.key.as_ref(),
        ],
    )?;
    let monster_seeds = &[
        SEED_MONSTER.as_bytes(),
        program_id.as_ref(),
        &mint_info.key.as_ref(),
        &[bump_seed],
    ];
    create_or_allocate_account_raw(
        *program_id,
        monster_info,
        rent_info,
        system_info,
        signer_info,
        MAX_MONSTER_LENGTH,
        monster_seeds,
    )?;

    Ok(())
}

pub fn create_battle_history_info<'a>(
    program_id: &Pubkey,
    battle_history_info: &AccountInfo<'a>,
    rent_info: &AccountInfo<'a>,
    system_info: &AccountInfo<'a>,
    signer_info: &AccountInfo<'a>,
) -> Result<(), ProgramError> {
    let bump_seed = assert_derivation(
        program_id,
        battle_history_info,
        &[
            SEED_BATTLE_HISTORY.as_bytes(),
            program_id.as_ref(),
        ],
    )?;
    let seeds = &[
        SEED_BATTLE_HISTORY.as_bytes(),
        program_id.as_ref(),
        &[bump_seed],
    ];
    create_or_allocate_account_raw(
        *program_id,
        battle_history_info,
        rent_info,
        system_info,
        signer_info,
        MAX_BATTLE_HISTORY_LENGTH,
        seeds,
    )?;

    Ok(())
}

pub fn create_metadata_edition<'a>(
    program_id: &Pubkey,
    pda_creator_info: &AccountInfo<'a>,
    config_data: ConfigureData,
    signer_info: &AccountInfo<'a>,
    mint_info: &AccountInfo<'a>,
    metadata_info: &AccountInfo<'a>,
    edition_info: &AccountInfo<'a>,
    metadata_program_info: &AccountInfo<'a>,
    token_program_info: &AccountInfo<'a>,
    system_info: &AccountInfo<'a>,
    rent_info: &AccountInfo<'a>,
) -> Result<(), ProgramError> {
    let pda_bump = assert_pda_creator(&program_id, pda_creator_info)?;
    let pda_seed = [
        SEED_BATTLE.as_bytes(),
        program_id.as_ref(),
        "pda_creator".as_bytes(),
        &[pda_bump],
    ];

    let creators = vec![
        mpl_token_metadata::state::Creator {
            address: *pda_creator_info.key,
            verified: true,
            share: 0,
        },
        mpl_token_metadata::state::Creator {
            address: *program_id,
            verified: false,
            share: 100,
        },
    ];

    let name_id = String::from(" #") + &config_data.current_id.to_string();
    msg!("Create Metadata");
    invoke_signed(
        &create_metadata_accounts_v2(
            *metadata_program_info.key,
            *metadata_info.key,
            *mint_info.key,
            *signer_info.key,
            *signer_info.key,
            *pda_creator_info.key, //pda must be signer
            config_data.name.clone() + &name_id,
            config_data.symbol.clone(),
            config_data.uri.clone(),
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

    Ok(())
}

pub fn update_metadata<'a>(
    program_id: &Pubkey,
    signer_info: &AccountInfo<'a>,
    metadata_info: &AccountInfo<'a>,
    pda_creator_info: &AccountInfo<'a>,
    metadata_program_info: &AccountInfo<'a>,
    uri_new: String,
) -> Result<(), ProgramError> {
    let metadata = Metadata::from_account_info(metadata_info)?;
    let data = metadata.data;
    let creators = check_creators(
        pda_creator_info,
        data.creators,
    );
    let mut creators_new = creators.clone();
    if uri_new == "null" {
        creators_new.push(
            mpl_token_metadata::state::Creator {
                address: *signer_info.key,
                verified: false,
                share: 0,
            }
        );
    }

    let data_v2 = DataV2 {
        name: data.name,
        symbol: data.symbol,
        uri: uri_new,
        seller_fee_basis_points: data.seller_fee_basis_points,
        creators: Some(creators_new),
        collection: None,
        uses: None,
    };
    let pda_bump = assert_pda_creator(&program_id, pda_creator_info)?;
    let pda_seed = [
        SEED_BATTLE.as_bytes(),
        program_id.as_ref(),
        "pda_creator".as_bytes(),
        &[pda_bump],
    ];
    invoke_signed(
        &update_metadata_accounts_v2(
            *metadata_program_info.key,
            *metadata_info.key,
            *pda_creator_info.key,
            Some(*pda_creator_info.key),
            Some(data_v2),
            Some(true),
            Some(true),
        ),
        &[
            signer_info.clone(),
            metadata_info.clone(),
            pda_creator_info.clone(),
            metadata_program_info.clone(),
        ],
        &[&pda_seed],
    )?;

    Ok(())
}

pub fn init_monster_attributes<'a>(
    monster_info: &AccountInfo<'a>,
    game_config_info: &AccountInfo<'a>,
    use_race: bool,
    use_attrs: bool,
    mut args: QuickMintArgs,
) -> Result<(), ProgramError> {
    msg!("Init Attributes");
    let mut monster = Monster::from_account_info(monster_info)?;

    if !use_race {
        args.race = monster.race;
    }

    let monster_feature_config = get_monster_feature_by_index(args.race as usize);
    let all_features = handle_monster_feature_config(monster_feature_config);
    if !use_attrs {
        args.attrs = get_monster_features_from_race(all_features.clone())?;
    };

    monster.level = 1;
    monster.gender = get_random_u8(0, 2)?;
    monster.race = args.race;
    monster.breed = 0;
    monster.generation = 1;
    monster.fatigue = 0;
    monster.walk_target = get_random_walk_target()?;

    msg!("Init Battle Attributes");
    let game_config = GameConfig::from_account_info(game_config_info)?;
    let male = game_config.monster_male.clone();
    let female = game_config.monster_female.clone();
    let mut basic: Vec<u32> = male[monster.race as usize].clone();
    if monster.gender != 1 {
        basic = female[monster.race as usize].clone();
    }
    monster.hp = basic[0];
    monster.attack = basic[1];
    monster.defense = basic[2];
    monster.speed = basic[3];
    monster.agility = basic[4];
    monster.efficiency = basic[5];

    monster.energy = 10000;
    monster.last_battle_time = 0;
    // monster.hatch_time = now_timestamp() + 3600;
    monster.hatch_time = now_timestamp() + 60;
    monster.monster_feature = args.attrs;

    msg!("Init Battle Attributes By Features");
    for i in 0..all_features.len() {
        let features = all_features[i].clone();
        let index = monster.monster_feature[i];
        let feature = features[index as usize];

        monster.hp = monster.hp * (u32::from(feature[1]) + 1000 as u32) / 1000 as u32;
        monster.attack = monster.attack * (u32::from(feature[2]) + 1000 as u32) / 1000 as u32;
        monster.defense = monster.defense * (u32::from(feature[3]) + 1000 as u32) / 1000 as u32;
        monster.speed = monster.speed * (u32::from(feature[4]) + 1000 as u32) / 1000 as u32;
        monster.agility = monster.agility * (u32::from(feature[5]) + 1000 as u32) / 1000 as u32;
        monster.efficiency = monster.efficiency * (u32::from(feature[6]) + 1000 as u32) / 1000 as u32;
    }

    msg!("Serialize Monster");
    monster.serialize(&mut *monster_info.try_borrow_mut_data()?)?;

    Ok(())
}

fn check_creators(pda_creator_info: &AccountInfo, creators: Option<Vec<Creator>>) -> Vec<Creator> {
    return match creators {
        Some(c) => c,
        None => {
            let creators = vec![
                mpl_token_metadata::state::Creator {
                    address: *pda_creator_info.key,
                    verified: true,
                    share: 0,
                }
            ];
            creators
        }
    };
}


pub fn calculate_breed_attrs(father: Vec<u8>, mother: Vec<u8>) -> Result<Vec<u8>, ProgramError> {
    let mut breed = Vec::new();
    for i in 0..father.len() {
        let random = get_random_u8(i as u8, 2)?;
        if random == 0 {
            breed.push(father[i].clone());
        } else {
            breed.push(mother[i].clone());
        }
    }
    Ok(breed)
}

pub fn get_monster_features_from_race(all_features: Vec<Vec<[u16; 7]>>) -> Result<Vec<u8>, ProgramError> {
    let mut monster = Vec::new();
    for i in 0..all_features.len() {
        let feature_amount = all_features[i].len();
        let random_feature = get_random_u8(i as u8, feature_amount as u64)?;
        monster.push(random_feature);
    }
    for _i in 0..(10 - all_features.len()) {
        monster.push(0);
    }
    Ok(monster)
}

pub fn spl_token_burn_quick<'a>(
    mint_info: AccountInfo<'a>,
    owner_info: AccountInfo<'a>,
    token_program_info: AccountInfo<'a>,
    token_account_info: AccountInfo<'a>,
) -> Result<(), ProgramError> {
    spl_token_burn(TokenBurnParams {
        mint: mint_info,
        amount: 1,
        authority: owner_info,
        token_program: token_program_info,
        source: token_account_info,
        authority_signer_seeds: None,
    })?;

    Ok(())
}

pub fn get_random_walk_target() -> Result<u8, ProgramError> {
    let r = get_random_u8(0, 100)?;
    let mut result: u8 = 30;
    if r < 10 {
        result = 5;
    } else if r < 25 {
        result = 10;
    } else if r < 50 {
        result = 15;
    } else if r < 75 {
        result = 20;
    } else if r < 90 {
        result = 25;
    }
    Ok(result)
}