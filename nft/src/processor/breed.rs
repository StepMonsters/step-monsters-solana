use borsh::BorshSerialize;
use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, msg, program::invoke, program_error::ProgramError, pubkey::Pubkey, system_instruction, sysvar};

use crate::{ferror, utils::*};
use crate::state::{ConfigureData, Monster, QuickMintArgs};
use crate::utils_config::calculate_breed_spend_game_token;
use crate::utils_mint::{calculate_breed_attrs, create_metadata_edition, create_monster_info, init_monster_attributes};

pub fn process_breed(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let pda_creator_info = next_account_info(account_info_iter)?; //nft creator: pda
    let fee_receiver_info = next_account_info(account_info_iter)?; // fee_receiver: wallet
    let mint_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let edition_info = next_account_info(account_info_iter)?;
    let monster_info = next_account_info(account_info_iter)?;
    let _program_info = next_account_info(account_info_iter)?;
    let signer_ata_info = next_account_info(account_info_iter)?;
    let program_ata_info = next_account_info(account_info_iter)?;
    let game_config_info = next_account_info(account_info_iter)?;

    let father_mint_info = next_account_info(account_info_iter)?;
    let mother_mint_info = next_account_info(account_info_iter)?;
    let father_info = next_account_info(account_info_iter)?;
    let mother_info = next_account_info(account_info_iter)?;
    let collection_info = next_account_info(account_info_iter)?;
    let collection_metadata_info = next_account_info(account_info_iter)?;
    let collection_edition_info = next_account_info(account_info_iter)?;

    let metadata_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

    let mut father = Monster::from_account_info(father_info)?;
    let mut mother = Monster::from_account_info(mother_info)?;
    if father.race != mother.race {
        return ferror!("require same race");
    }

    if father.breed >= 5 || mother.breed >= 5 {
        return ferror!("reach max breed times");
    }

    msg!("Breed LST spending");
    let spend = calculate_breed_spend_game_token(father.breed, mother.breed);
    spl_token_transfer_invoke(
        token_program_info.clone(),
        signer_ata_info.clone(),
        program_ata_info.clone(),
        signer_info.clone(),
        spend,
    )?;

    let breed_attrs = calculate_breed_attrs(
        father.monster_feature.clone(),
        mother.monster_feature.clone(),
    )?;

    let mut breed_generation = father.generation + 1;
    if father.generation > mother.generation {
        breed_generation = mother.generation + 1;
    }

    msg!("Assert Public Key");
    assert_eq_pubkey(&metadata_program_info, &mpl_token_metadata::id())?;
    assert_eq_pubkey(&token_program_info, &spl_token::id())?;
    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;
    assert_config(&program_id, &config_info)?;

    msg!("Assert Fee Receiver");
    let mut config_data = ConfigureData::from_account_info(config_info)?;
    assert_eq_pubkey(&fee_receiver_info, &config_data.fee_receiver)?;

    msg!("Check Config Initialized");
    if !config_data.is_initialized {
        return ferror!("invalid mint state");
    }

    msg!("Transfer Mint Fee");
    if config_data.price > 0 {
        invoke(
            &system_instruction::transfer(&signer_info.key, &config_data.fee_receiver, config_data.price),
            &[
                signer_info.clone(),
                fee_receiver_info.clone(),
                system_info.clone(),
            ],
        )?;
    }

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
        &collection_info,
        &collection_metadata_info,
        &collection_edition_info,
    )?;
    config_data.current_id += 1;
    config_data.serialize(&mut *config_info.try_borrow_mut_data()?)?;

    msg!("Create Monster Info");
    create_monster_info(
        &program_id,
        &monster_info,
        &mint_info,
        &rent_info,
        &system_info,
        &signer_info,
    )?;

    let args = QuickMintArgs { race: father.race.clone(), attrs: breed_attrs.clone() };

    msg!("Init Monster Attributes");
    let mut monster = Monster::from_account_info(monster_info)?;
    let init_attrs = init_monster_attributes(
        monster.clone(),
        &game_config_info,
        true,
        false,
        args,
    )?;
    monster = init_attrs.clone();
    monster.father_mint = *father_mint_info.key;
    monster.mother_mint = *mother_mint_info.key;
    monster.generation = breed_generation.clone();
    monster.level = 0;
    monster.serialize(&mut *monster_info.try_borrow_mut_data()?)?;

    //update father mother breed amount
    father.breed += 1;
    mother.breed += 1;
    father.serialize(&mut *father_info.try_borrow_mut_data()?)?;
    mother.serialize(&mut *mother_info.try_borrow_mut_data()?)?;

    Ok(())
}
