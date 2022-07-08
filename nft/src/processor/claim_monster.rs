use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{ferror, state::*, utils::*};
use crate::utils_mint::update_metadata;

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
        return ferror!("hatching...");
    }
    monster.hatch_time = now_timestamp();
    msg!("Monster Serialize");
    monster.serialize(&mut *monster_info.try_borrow_mut_data()?)?;

    // create nft store 
    assert_nft_store(&program_id, &nft_mint_info, &nft_store_info)?;
    let auth_bump = assert_monster_authority(&program_id, &authority_info)?;

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
            "authority".as_bytes(),
            &[auth_bump],
        ],
    )?;

    msg!("Update Metadata Account");
    update_metadata(
        program_id,
        signer_info,
        metadata_info,
        pda_creator_info,
        metadata_program_info,
        args.uri
    )?;

    msg!("Update Incubator");
    incubator.is_done = true;
    incubator.serialize(&mut *incubator_info.try_borrow_mut_data()?)?;
    Ok(())
}
