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

    let male_a: [u32; 5] = [100, 100, 120, 100, 120];
    let male_b: [u32; 5] = [100, 140, 110, 120, 150];
    let male_c: [u32; 5] = [350, 370, 340, 330, 360];
    let male_d: [u32; 5] = [480, 460, 450, 440, 450];
    let male_e: [u32; 5] = [500, 520, 490, 510, 500];
    let male_f: [u32; 5] = [630, 640, 620, 610, 650];
    let male_g: [u32; 5] = [700, 720, 690, 710, 700];
    let male_h: [u32; 5] = [830, 840, 820, 810, 850];
    let male_i: [u32; 5] = [940, 920, 890, 910, 940];
    let male_j: [u32; 5] = [1090, 1040, 1060, 1010, 1030];

    let male = [
        male_a, male_b, male_c, male_d, male_e,
        male_f, male_g, male_h, male_i, male_j
    ];

    let female_a: [u32; 5] = [100, 120, 100, 120, 100];
    let female_b: [u32; 5] = [90, 140, 130, 130, 130];
    let female_c: [u32; 5] = [370, 350, 340, 360, 330];
    let female_d: [u32; 5] = [450, 480, 440, 450, 460];
    let female_e: [u32; 5] = [520, 490, 510, 500, 500];
    let female_f: [u32; 5] = [650, 610, 620, 630, 640];
    let female_g: [u32; 5] = [720, 690, 710, 700, 700];
    let female_h: [u32; 5] = [850, 810, 820, 830, 840];
    let female_i: [u32; 5] = [900, 890, 940, 960, 910];
    let female_j: [u32; 5] = [1050, 1080, 1040, 1010, 1050];

    let female = [
        female_a, female_b, female_c, female_d, female_e,
        female_f, female_g, female_h, female_i, female_j
    ];

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

    let mut game_config = GameConfig::from_account_info(game_config_info)?;
    let mut male = game_config.monster_male.clone();
    let mut female = game_config.monster_female.clone();

    for i in 0..male.len() {
        for j in 0..male[i].len() {
            male[i][j] += 1;
        }
    }

    for i in 0..female.len() {
        for j in 0..female[i].len() {
            female[i][j] += 1;
        }
    }

    game_config.monster_male = male;
    game_config.monster_female = female;
    game_config.serialize(&mut *game_config_info.try_borrow_mut_data()?)?;

    Ok(())
}