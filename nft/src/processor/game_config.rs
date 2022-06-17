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

    let male_a: [u32; 6] = [100, 100, 120, 100, 120, 10];
    let male_b: [u32; 6] = [100, 140, 110, 120, 150, 14];
    let male_c: [u32; 6] = [350, 370, 340, 330, 360, 30];
    let male_d: [u32; 6] = [480, 460, 450, 440, 450, 32];
    let male_e: [u32; 6] = [500, 520, 490, 510, 500, 45];
    let male_f: [u32; 6] = [630, 640, 620, 610, 650, 48];
    let male_g: [u32; 6] = [700, 720, 690, 710, 700, 56];
    let male_h: [u32; 6] = [830, 840, 820, 810, 850, 60];
    let male_i: [u32; 6] = [940, 920, 890, 910, 940, 65];
    let male_j: [u32; 6] = [1090, 1040, 1060, 1010, 1030, 70];

    let male = [
        male_a, male_b, male_c, male_d, male_e,
        male_f, male_g, male_h, male_i, male_j
    ];

    let female_a: [u32; 6] = [100, 120, 100, 120, 100, 10];
    let female_b: [u32; 6] = [90, 140, 130, 130, 130, 14];
    let female_c: [u32; 6] = [370, 350, 340, 360, 330, 30];
    let female_d: [u32; 6] = [450, 480, 440, 450, 460, 32];
    let female_e: [u32; 6] = [520, 490, 510, 500, 500, 45];
    let female_f: [u32; 6] = [650, 610, 620, 630, 640, 48];
    let female_g: [u32; 6] = [720, 690, 710, 700, 700, 56];
    let female_h: [u32; 6] = [850, 810, 820, 830, 840, 60];
    let female_i: [u32; 6] = [900, 890, 940, 960, 910, 65];
    let female_j: [u32; 6] = [1050, 1080, 1040, 1010, 1050, 70];

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

    let male_a: [u32; 6] = [100, 100, 120, 100, 120, 10];
    let male_b: [u32; 6] = [100, 140, 110, 120, 150, 14];
    let male_c: [u32; 6] = [350, 370, 340, 330, 360, 30];
    let male_d: [u32; 6] = [480, 460, 450, 440, 450, 32];
    let male_e: [u32; 6] = [500, 520, 490, 510, 500, 45];
    let male_f: [u32; 6] = [630, 640, 620, 610, 650, 48];
    let male_g: [u32; 6] = [700, 720, 690, 710, 700, 56];
    let male_h: [u32; 6] = [830, 840, 820, 810, 850, 60];
    let male_i: [u32; 6] = [940, 920, 890, 910, 940, 65];
    let male_j: [u32; 6] = [1090, 1040, 1060, 1010, 1030, 70];

    let male = [
        male_a, male_b, male_c, male_d, male_e,
        male_f, male_g, male_h, male_i, male_j
    ];

    let female_a: [u32; 6] = [100, 120, 100, 120, 100, 10];
    let female_b: [u32; 6] = [90, 140, 130, 130, 130, 14];
    let female_c: [u32; 6] = [370, 350, 340, 360, 330, 30];
    let female_d: [u32; 6] = [450, 480, 440, 450, 460, 32];
    let female_e: [u32; 6] = [520, 490, 510, 500, 500, 45];
    let female_f: [u32; 6] = [650, 610, 620, 630, 640, 48];
    let female_g: [u32; 6] = [720, 690, 710, 700, 700, 56];
    let female_h: [u32; 6] = [850, 810, 820, 830, 840, 60];
    let female_i: [u32; 6] = [900, 890, 940, 960, 910, 65];
    let female_j: [u32; 6] = [1050, 1080, 1040, 1010, 1050, 70];

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