use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{ferror, utils::*};
use crate::instruction::mint;
use crate::state::{MintArgs, Monster};
use crate::utils_config::calculate_breed_spend_game_token;
use crate::utils_mint::calculate_breed_attrs;

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
    let program_info = next_account_info(account_info_iter)?;
    let signer_ata_info = next_account_info(account_info_iter)?;
    let token_admin_info = next_account_info(account_info_iter)?;

    let father_mint_info = next_account_info(account_info_iter)?;
    let mother_mint_info = next_account_info(account_info_iter)?;
    let father_info = next_account_info(account_info_iter)?;
    let mother_info = next_account_info(account_info_iter)?;

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
    let spend = calculate_breed_spend_game_token(father.breed,mother.breed);
    spl_token_transfer_invoke(
        token_program_info.clone(),
        signer_ata_info.clone(),
        token_admin_info.clone(),
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

    let mint_args = MintArgs {
        race: father.race,
        attrs: breed_attrs,
        generation: breed_generation,
        father_mint: *father_mint_info.key,
        mother_mint: *mother_mint_info.key,
    };

    invoke(
        &mint(
            program_id,
            signer_info.key,
            config_info.key,
            pda_creator_info.key,
            fee_receiver_info.key,
            mint_info.key,
            metadata_info.key,
            edition_info.key,
            monster_info.key,
            metadata_program_info.key,
            token_program_info.key,
            mint_args,
        )?,
        &[
            signer_info.clone(),
            config_info.clone(),
            pda_creator_info.clone(),
            fee_receiver_info.clone(),
            mint_info.clone(),
            metadata_info.clone(),
            edition_info.clone(),
            monster_info.clone(),
            program_info.clone(),
            metadata_program_info.clone(),
            token_program_info.clone(),
            rent_info.clone(),
            system_info.clone()
        ],
    )?;

    father.breed += 1;
    mother.breed += 1;
    father.serialize(&mut *father_info.try_borrow_mut_data()?)?;
    mother.serialize(&mut *mother_info.try_borrow_mut_data()?)?;

    Ok(())
}
