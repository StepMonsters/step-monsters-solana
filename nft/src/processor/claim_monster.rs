use borsh::BorshSerialize;
use mpl_token_metadata::instruction::update_metadata_accounts_v2;
use mpl_token_metadata::state::{ DataV2, Metadata};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    program::{ invoke_signed },
    pubkey::Pubkey,
};

use crate::{state::*, ferror, utils::*};

pub fn process_claim_monster(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: ClaimMonsterArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let monster_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let incubator_info = next_account_info(account_info_iter)?;
    let pda_creator_info = next_account_info(account_info_iter)?; //nft creator: pda

    let nft_mint_info = next_account_info(account_info_iter)?; // NFT mint address
    let nft_account_info = next_account_info(account_info_iter)?; // account own the nft has been approve for authority
    let nft_store_info = next_account_info(account_info_iter)?; // owned by authority_info to keep NFT
    let authority_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let metadata_program_info = next_account_info(account_info_iter)?;
    
    assert_signer(&signer_info)?;
    
    msg!("Update Monster Info");
    let mut monster = Monster::from_account_info(monster_info)?;
    assert_incubator(&program_id, &nft_mint_info, &incubator_info)?;
    let mut incubator = Incubator::from_account_info(incubator_info)?;
    assert_eq_pubkey(&signer_info, &incubator.user)?;
    assert_eq_pubkey(&nft_account_info, &incubator.nft_return)?;
    // hatch need one day
    if monster.hatch_time < now_timestamp() - 86400 {
        return ferror!("hatching...")
    }
    monster.hatch_time = now_timestamp();
    msg!("Monster Serialize");
    monster.serialize(&mut *monster_info.try_borrow_mut_data()?)?;

    // create nft store 
    assert_nft_store(&program_id, &nft_mint_info, &nft_store_info)?;
    let auth_bump = assert_monster_authority(&program_id, &nft_mint_info, &authority_info)?;
    
    //transfer token back
    spl_token_transfer(
        token_program_info.clone(),
        nft_store_info.clone(),
        nft_account_info.clone(),
        authority_info.clone(),
        1,
        &[
            SEED_BATTLE.as_bytes(),
            program_id.as_ref(),
            nft_mint_info.key.as_ref(),
            "authority".as_bytes(),
            &[auth_bump],
        ],
    )?;
    
    msg!("Update Metadata Account");

    let metadata = Metadata::from_account_info(metadata_info)?;
    let data = metadata.data;
    let datav2 = DataV2 {
        name: data.name,
        symbol: data.symbol,
        uri: args.uri,
        seller_fee_basis_points: data.seller_fee_basis_points,
        creators: data.creators,
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
            Some(datav2),
            Some(true),
            Some(true),
        ),
        &[
            metadata_info.clone(),
            signer_info.clone(),
            metadata_program_info.clone(),
            pda_creator_info.clone(),
        ],
        &[&pda_seed],
    )?;

    incubator.is_done = true;
    incubator.serialize(&mut *incubator_info.try_borrow_mut_data()?)?;
    Ok(())
}
