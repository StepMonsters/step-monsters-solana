use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{state::*, utils::*};

pub fn process_create_game_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;
    let game_config_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

    msg!("Create Game Config");
    let bump_seed = assert_derivation(
        program_id,
        game_config_info,
        &[
            SEED_GAME_CONFIG.as_bytes(),
            program_id.as_ref(),
        ],
    )?;
    let game_config_seeds = &[
        SEED_GAME_CONFIG.as_bytes(),
        program_id.as_ref(),
        &[bump_seed],
    ];
    create_or_allocate_account_raw(
        *program_id,
        game_config_info,
        rent_info,
        system_info,
        signer_info,
        MAX_GAME_CONFIG_LENGTH,
        game_config_seeds,
    )?;

    let male_data: [[u32; 6]; 10] = [
        [100, 100, 120, 100, 120, 20],
        [100, 140, 110, 120, 150, 24],
        [350, 370, 340, 330, 360, 30],
        [480, 460, 450, 440, 450, 32],
        [500, 520, 490, 510, 500, 45],
        [630, 640, 620, 610, 650, 48],
        [700, 720, 690, 710, 700, 56],
        [830, 840, 820, 810, 850, 60],
        [940, 920, 890, 910, 940, 65],
        [1090, 1040, 1060, 1010, 1030, 80]
    ];

    let female_data: [[u32; 6]; 10] = [
        [100, 120, 100, 120, 100, 20],
        [90, 140, 130, 130, 130, 24],
        [370, 350, 340, 360, 330, 30],
        [450, 480, 440, 450, 460, 32],
        [520, 490, 510, 500, 500, 45],
        [650, 610, 620, 630, 640, 48],
        [720, 690, 710, 700, 700, 56],
        [850, 810, 820, 830, 840, 60],
        [900, 890, 940, 960, 910, 65],
        [1050, 1080, 1040, 1010, 1050, 80]
    ];

    let male = handle_game_config(male_data);
    let female = handle_game_config(female_data);

    let mut game_config = GameConfig::from_account_info(game_config_info)?;
    game_config.monster_male = male;
    game_config.monster_female = female;
    game_config.serialize(&mut *game_config_info.try_borrow_mut_data()?)?;

    Ok(())
}

pub fn process_update_game_config(
    _: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let game_config_info = next_account_info(account_info_iter)?;

    let game_config = GameConfig::from_account_info(game_config_info)?;
    game_config.serialize(&mut *game_config_info.try_borrow_mut_data()?)?;

    Ok(())
}