use borsh::BorshSerialize;
use mpl_token_metadata::instruction::update_metadata_accounts_v2;
use mpl_token_metadata::state::DataV2;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
};

use crate::{state::*, utils::*};
use crate::error::AppError::InvalidHatchTime;

pub fn process_hatch(
    _: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    let metadata_program_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let monster_info = next_account_info(account_info_iter)?;
    let game_config_info = next_account_info(account_info_iter)?;
    let monster_feature_config_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

    msg!("Update Monster Info");
    let mut monster = Monster::from_account_info(monster_info)?;

    msg!("Check Hatch Time");
    if monster.hatch_time > now_timestamp() {
        return Err(InvalidHatchTime.into());
    }

    msg!("Init Attributes");
    monster.level = 1;
    monster.gender = get_random_u8(0, 2)?;
    monster.race = 1;
    monster.breed = 0;
    monster.generation = 1;
    monster.fatigue = 0;

    msg!("Init Battle Attributes");
    let game_config = GameConfig::from_account_info(game_config_info)?;
    let mut basic = game_config.monster_male.clone()[monster.race as usize];
    if monster.gender != 1 {
        basic = game_config.monster_female.clone()[monster.race as usize];
    }
    monster.hp = basic[0];
    monster.attack = basic[1];
    monster.defense = basic[2];
    monster.speed = basic[3];
    monster.agility = basic[4];
    monster.efficiency = basic[5];

    msg!("Init Battle Attributes By Features");
    let monster_feature_config = MonsterFeatureConfig::from_account_info(monster_feature_config_info)?;
    let mut config = monster_feature_config.monster_0.clone();
    if monster.race == 0 {
        config = monster_feature_config.monster_0.clone();
    }
    let all_features = handle_monster_feature_config(config);
    for i in 0..all_features.len() {
        let features = &all_features[i];
        let index = monster.monster_feature[i];
        let feature = features[index as usize];

        monster.hp = monster.hp * (u32::from(feature[1]) + 1000 as u32) / 1000 as u32;
        monster.attack = monster.attack * (u32::from(feature[1]) + 1000 as u32) / 1000 as u32;
        monster.defense = monster.defense * (u32::from(feature[1]) + 1000 as u32) / 1000 as u32;
        monster.speed = monster.speed * (u32::from(feature[1]) + 1000 as u32) / 1000 as u32;
        monster.agility = monster.agility * (u32::from(feature[1]) + 1000 as u32) / 1000 as u32;
        monster.efficiency = monster.efficiency * (u32::from(feature[1]) + 1000 as u32) / 1000 as u32;
    }

    msg!("Monster Serialize");
    monster.serialize(&mut *monster_info.try_borrow_mut_data()?)?;

    msg!("Update Metadata Account");
    let creator = vec![
        mpl_token_metadata::state::Creator {
            address: *signer_info.key,
            verified: false,
            share: 100,
        },
    ];
    let name = String::from("my_name");
    let symbol = String::from("my_symbol");
    let uri = String::from("https://arweave.net/y5e5DJsiwH0s_ayfMwYk-SnrZtVZzHLQDSTZ5dNRUHA");
    let data = DataV2 {
        name,
        symbol,
        uri,
        seller_fee_basis_points: 1,
        creators: Some(creator),
        collection: None,
        uses: None,
    };
    invoke(
        &update_metadata_accounts_v2(
            *metadata_program_info.key,
            *metadata_info.key,
            *signer_info.key,
            Some(*signer_info.key),
            Some(data),
            Some(false),
            Some(false),
        ),
        &[
            metadata_info.clone(),
            signer_info.clone(),
            metadata_program_info.clone(),
            token_program_info.clone(),
            system_info.clone(),
        ],
    )?;

    Ok(())
}
