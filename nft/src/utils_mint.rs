use borsh::BorshSerialize;
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2, update_metadata_accounts_v2, verify_collection};
use mpl_token_metadata::state::{Creator, DataV2, Metadata};
use mpl_token_metadata::utils::{spl_token_burn, TokenBurnParams};
use solana_program::{msg, system_instruction};
use solana_program::account_info::AccountInfo;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::instruction::{initialize_mint, mint_to};

use crate::state::{ConfigureData, GameConfig, MAX_BATTLE_HISTORY_BODIES_LENGTH, MAX_BATTLE_HISTORY_LENGTH, MAX_MONSTER_LENGTH, Monster, now_timestamp, QuickMintArgs, SEED_BATTLE, SEED_BATTLE_HISTORY, SEED_BATTLE_HISTORY_BODIES, SEED_MONSTER, SEED_TOKEN_ADMIN};
use crate::utils::{assert_derivation, assert_pda_creator, create_or_allocate_account_raw, get_random_u8};
use crate::utils_config::{get_monster_feature_by_index, handle_monster_feature_config};

pub fn handle_init_mint<'a>(
    _program_id: &Pubkey,
    signer_info: &AccountInfo<'a>,
    mint_info: &AccountInfo<'a>,
    ata_info: &AccountInfo<'a>,
    token_program_info: &AccountInfo<'a>,
    ass_token_program_info: &AccountInfo<'a>,
    rent_info: &AccountInfo<'a>,
    system_info: &AccountInfo<'a>,
) -> Result<(), ProgramError> {
    let size = 82;
    let rent = &Rent::from_account_info(&rent_info)?;
    let required_lamports = rent.minimum_balance(size);

    msg!("Create Account");
    invoke(
        &system_instruction::create_account(
            signer_info.key,
            mint_info.key,
            required_lamports,
            size as u64,
            token_program_info.key,
        ),
        &[signer_info.clone(), mint_info.clone()],
    )?;

    msg!("Initialize Mint");
    invoke(
        &initialize_mint(
            token_program_info.key,
            mint_info.key,
            signer_info.key,
            Some(signer_info.key),
            0,
        )?,
        &[signer_info.clone(), mint_info.clone(), rent_info.clone(), token_program_info.clone()],
    )?;

    msg!("Create Associated Token Account");
    invoke(
        &create_associated_token_account(
            signer_info.key,
            signer_info.key,
            mint_info.key,
        ),
        &[
            signer_info.clone(),
            ata_info.clone(),
            ass_token_program_info.clone(),
            mint_info.clone(),
            token_program_info.clone(),
            system_info.clone()
        ],
    )?;

    msg!("Mint To");
    invoke(
        &mint_to(
            token_program_info.key,
            mint_info.key,
            ata_info.key,
            signer_info.key,
            &[signer_info.key],
            1,
        )?,
        &[
            signer_info.clone(),
            ata_info.clone(),
            mint_info.clone(),
            token_program_info.clone(),
            system_info.clone()
        ],
    )?;

    Ok(())
}

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
    extend: bool,
) -> Result<(), ProgramError> {
    let mut seed = SEED_BATTLE_HISTORY;
    let mut max = MAX_BATTLE_HISTORY_LENGTH;
    if extend {
        seed = SEED_BATTLE_HISTORY_BODIES;
        max = MAX_BATTLE_HISTORY_BODIES_LENGTH;
    }
    let bump_seed = assert_derivation(
        program_id,
        battle_history_info,
        &[
            seed.as_bytes(),
            program_id.as_ref(),
            signer_info.key.as_ref()
        ],
    )?;
    let seeds = &[
        seed.as_bytes(),
        program_id.as_ref(),
        signer_info.key.as_ref(),
        &[bump_seed],
    ];
    create_or_allocate_account_raw(
        *program_id,
        battle_history_info,
        rent_info,
        system_info,
        signer_info,
        max,
        seeds,
    )?;

    Ok(())
}

pub fn create_token_admin_info<'a>(
    program_id: &Pubkey,
    token_admin_info: &AccountInfo<'a>,
    rent_info: &AccountInfo<'a>,
    system_info: &AccountInfo<'a>,
    signer_info: &AccountInfo<'a>,
) -> Result<(), ProgramError> {
    let bump_seed = assert_derivation(
        program_id,
        token_admin_info,
        &[
            SEED_TOKEN_ADMIN.as_bytes(),
            program_id.as_ref(),
        ],
    )?;
    let seeds = &[
        SEED_TOKEN_ADMIN.as_bytes(),
        program_id.as_ref(),
        &[bump_seed],
    ];
    create_or_allocate_account_raw(
        *program_id,
        token_admin_info,
        rent_info,
        system_info,
        signer_info,
        0,
        seeds,
    )?;

    Ok(())
}

pub fn mint_game_token_to_ata<'a>(
    program_id: &Pubkey,
    signer_info: &AccountInfo<'a>,
    signer_ata_info: &AccountInfo<'a>,
    mint_info: &AccountInfo<'a>,
    token_admin_info: &AccountInfo<'a>,
    ass_token_program_info: &AccountInfo<'a>,
    token_program_info: &AccountInfo<'a>,
    system_info: &AccountInfo<'a>,
    amount: u64,
) -> Result<(), ProgramError> {
    msg!("Token Admin Seeds");
    let bump_seed = assert_derivation(
        program_id,
        token_admin_info,
        &[
            SEED_TOKEN_ADMIN.as_bytes(),
            program_id.as_ref(),
        ],
    )?;
    let token_admin_seeds = [
        SEED_TOKEN_ADMIN.as_bytes(),
        program_id.as_ref(),
        &[bump_seed],
    ];

    msg!("Create Signer Associated Token Account");
    if signer_ata_info.lamports() <= 0 {
        invoke(
            &create_associated_token_account(
                signer_info.key,
                signer_info.key,
                mint_info.key,
            ),
            &[
                signer_info.clone(),
                signer_ata_info.clone(),
                ass_token_program_info.clone(),
                mint_info.clone(),
                token_program_info.clone(),
                system_info.clone()
            ],
        )?;
    }

    msg!("Mint To Signer");
    invoke_signed(
        &mint_to(
            token_program_info.key,
            mint_info.key,
            signer_ata_info.key,
            token_admin_info.key,
            &[token_admin_info.key],
            amount,
        )?,
        &[
            signer_info.clone(),
            signer_ata_info.clone(),
            mint_info.clone(),
            token_program_info.clone(),
            system_info.clone(),
            token_admin_info.clone()
        ],
        &[&token_admin_seeds],
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
    collection_info: &AccountInfo<'a>,
    collection_metadata_info: &AccountInfo<'a>,
    collection_edition_info: &AccountInfo<'a>,
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
            share: 100,
        },
    ];

    let collection = mpl_token_metadata::state::Collection {
        key: *collection_info.key,
        verified: false,
    };

    //name and uri
    let name_id = String::from(" #") + &config_data.current_id.to_string();
    let metadata_uri = config_data.uri.clone() + &*mint_info.key.to_string().clone();

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
            metadata_uri.clone(),
            Some(creators),
            config_data.fee,
            true,
            true,
            Some(collection.clone()),
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
            Some(0),
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

    msg!("Verify Collection");
    invoke_signed(
        &verify_collection(
            *metadata_program_info.key,
            *metadata_info.key,
            *pda_creator_info.key,
            *signer_info.key,
            *collection_info.key,
            *collection_metadata_info.key,
            *collection_edition_info.key,
            None,
        ),
        &[
            signer_info.clone(),
            metadata_info.clone(),
            metadata_program_info.clone(),
            token_program_info.clone(),
            system_info.clone(),
            rent_info.clone(),
            pda_creator_info.clone(),
            collection_info.clone(),
            collection_metadata_info.clone(),
            collection_edition_info.clone(),
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

pub fn create_metadata_edition_create_collection<'a>(
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

    //name and uri
    let metadata_name = config_data.name.clone();
    let metadata_uri = config_data.uri.clone() + &String::from("collection.json");

    msg!("Create Metadata");
    invoke_signed(
        &create_metadata_accounts_v2(
            *metadata_program_info.key,
            *metadata_info.key,
            *mint_info.key,
            *signer_info.key,
            *signer_info.key,
            *pda_creator_info.key, //pda must be signer
            metadata_name.clone(),
            config_data.symbol.clone(),
            metadata_uri.clone(),
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
            Some(0),
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
    } else {
        args.attrs = get_monster_features_from_race(all_features.clone())?;
    }

    monster.level = 1;
    monster.gender = get_random_u8(0, 2)?;
    monster.race = args.race;
    monster.breed = 0;
    monster.fatigue = 0;
    monster.walk_target = get_random_walk_target()?;

    if monster.generation < 1 {
        monster.generation = 1;
    }

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

    monster.energy = 30000;
    monster.last_battle_time = 0;
    monster.hatch_time = now_timestamp() + 2 * 60 * 60;
    monster.monster_feature = args.attrs.clone();

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
    let r = get_random_u8(0, 3)?;
    let result;
    if r == 0 {
        result = 6;
    } else if r == 1 {
        result = 10;
    } else {
        result = 15;
    }
    Ok(result)
}