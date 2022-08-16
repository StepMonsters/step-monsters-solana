use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{state::*, utils::*};
use crate::utils_mint::{init_monster_attributes, update_metadata};

pub fn process_hatch(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let monster_info = next_account_info(account_info_iter)?;
    let game_config_info = next_account_info(account_info_iter)?;
    let _monster_feature_config_info = next_account_info(account_info_iter)?;
    let incubator_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let pda_creator_info = next_account_info(account_info_iter)?;
    let metadata_program_info = next_account_info(account_info_iter)?;

    let nft_mint_info = next_account_info(account_info_iter)?; // NFT mint address
    let nft_account_info = next_account_info(account_info_iter)?; // account own the nft has been approve for authority
    let nft_store_info = next_account_info(account_info_iter)?; // owned by authority_info to keep NFT
    let authority_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

    msg!("Update Monster Info");
    let mint_args = QuickMintArgs { race: 0, attrs: Vec::new() };
    init_monster_attributes(
        &monster_info,
        game_config_info,
        false,
        false,
        mint_args,
    )?;

    msg!("Update Metadata Account");
    update_metadata(
        program_id,
        signer_info,
        metadata_info,
        pda_creator_info,
        metadata_program_info,
        String::from("null"),
    )?;

    msg!("Create Store");
    let nft_store_bump = assert_nft_store(&program_id, &nft_mint_info, &nft_store_info)?;
    let auth_bump = assert_monster_authority(&program_id, &authority_info)?;
    let authority_seed = [
        SEED_BATTLE.as_bytes(),
        program_id.as_ref(),
        "authority".as_bytes(),
        &[auth_bump],
    ];
    spl_token_create_account(
        &token_program_info,
        &signer_info,
        &nft_mint_info,
        &nft_store_info,
        &authority_info,
        &[
            SEED_BATTLE.as_bytes(),
            program_id.as_ref(),
            nft_mint_info.key.as_ref(),
            "nft_store".as_bytes(),
            &[nft_store_bump],
        ],
        &authority_seed,
        &rent_info,
    )?;

    msg!("Transfer Token To Store");
    spl_token_transfer_invoke(
        token_program_info.clone(),
        nft_account_info.clone(),
        nft_store_info.clone(),
        signer_info.clone(),
        1,
    )?;

    msg!("Create incubator Info");
    let bump_seed = assert_incubator(&program_id, &nft_mint_info, &incubator_info)?;
    let incubator_seeds = &[
        SEED_BATTLE.as_bytes(),
        program_id.as_ref(),
        &nft_mint_info.key.as_ref(),
        &[bump_seed],
    ];
    create_or_allocate_account_raw(
        *program_id,
        incubator_info,
        rent_info,
        system_info,
        signer_info,
        Incubator::LEN,
        incubator_seeds,
    )?;
    let mut incubator = Incubator::from_account_info(incubator_info)?;
    incubator.nft = nft_mint_info.key.clone();
    incubator.nft_return = nft_account_info.key.clone();
    incubator.nft_store = nft_store_info.key.clone();
    incubator.user = signer_info.key.clone();
    incubator.serialize(&mut *incubator_info.try_borrow_mut_data()?)?;
    Ok(())
}
