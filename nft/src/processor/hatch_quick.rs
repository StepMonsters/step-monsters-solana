use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{state::*, utils::*};
use crate::error::AppError::InvalidHatchTime;

pub fn process_hatch_quick(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    // let token_program_info = next_account_info(account_info_iter)?;
    // let system_info = next_account_info(account_info_iter)?;
    //
    // let metadata_program_info = next_account_info(account_info_iter)?;
    // let metadata_info = next_account_info(account_info_iter)?;
    let monster_info = next_account_info(account_info_iter)?;
    let game_config_info = next_account_info(account_info_iter)?;
    // let monster_feature_config_info = next_account_info(account_info_iter)?;
    //
    // let nft_mint_info = next_account_info(account_info_iter)?; // NFT mint address
    // let nft_account_info = next_account_info(account_info_iter)?; // account own the nft has been approve for authority
    // let nft_store_info = next_account_info(account_info_iter)?; // owned by authority_info to keep NFT
    // let authority_info = next_account_info(account_info_iter)?;
    // let rent_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

    msg!("Check Monster Info");
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

    monster.monster_feature = Vec::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

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

    //
    // msg!("Init Battle Attributes By Features");
    // let monster_feature_config = MonsterFeatureConfig::from_account_info(monster_feature_config_info)?;
    // let mut config = monster_feature_config.monster_0.clone();
    // if monster.race == 0 {
    //     config = monster_feature_config.monster_0.clone();
    // }
    // let all_features = handle_monster_feature_config(config);
    // for i in 0..all_features.len() {
    //     let features = &all_features[i];
    //     let index = monster.monster_feature[i];
    //     let feature = features[index as usize];
    //
    //     monster.hp = monster.hp * (u32::from(feature[1]) + 1000 as u32) / 1000 as u32;
    //     monster.attack = monster.attack * (u32::from(feature[1]) + 1000 as u32) / 1000 as u32;
    //     monster.defense = monster.defense * (u32::from(feature[1]) + 1000 as u32) / 1000 as u32;
    //     monster.speed = monster.speed * (u32::from(feature[1]) + 1000 as u32) / 1000 as u32;
    //     monster.agility = monster.agility * (u32::from(feature[1]) + 1000 as u32) / 1000 as u32;
    //     monster.efficiency = monster.efficiency * (u32::from(feature[1]) + 1000 as u32) / 1000 as u32;
    // }

    monster.energy = 100;
    monster.last_battle_time = 0;
    monster.hatch_time = 0;

    msg!("Monster Serialize");
    monster.serialize(&mut *monster_info.try_borrow_mut_data()?)?;

    Ok(())
}
