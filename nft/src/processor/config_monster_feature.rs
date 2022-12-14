use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{state::*, utils::*};
use crate::utils_config::get_monster_feature_by_index;

pub fn process_create_monster_feature_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;
    let feature_config_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

    msg!("Create Config Monster Feature");
    let bump_seed = assert_derivation(
        program_id,
        feature_config_info,
        &[
            SEED_MONSTER_FEATURE_CONFIG.as_bytes(),
            program_id.as_ref(),
        ],
    )?;
    let feature_config_seeds = &[
        SEED_MONSTER_FEATURE_CONFIG.as_bytes(),
        program_id.as_ref(),
        &[bump_seed],
    ];
    create_or_allocate_account_raw(
        *program_id,
        feature_config_info,
        rent_info,
        system_info,
        signer_info,
        MAX_MONSTER_FEATURE_CONFIG_LENGTH,
        feature_config_seeds,
    )?;

    let monster_0 = get_monster_feature_by_index(0);
    let monster_1 = get_monster_feature_by_index(1);
    let monster_2 = get_monster_feature_by_index(2);

    let mut feature_config = MonsterFeatureConfig::from_account_info(feature_config_info)?;
    feature_config.monster_0 = feature_config_to_vector(monster_0);
    feature_config.monster_1 = feature_config_to_vector(monster_1);
    feature_config.monster_2 = feature_config_to_vector(monster_2);
    feature_config.serialize(&mut *feature_config_info.try_borrow_mut_data()?)?;

    Ok(())
}