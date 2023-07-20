use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{ferror, state::*, utils::*};

pub fn process_claim_monster(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _args: ClaimMonsterArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let monster_info = next_account_info(account_info_iter)?;
    let _metadata_info = next_account_info(account_info_iter)?;
    let incubator_info = next_account_info(account_info_iter)?;
    let _pda_creator_info = next_account_info(account_info_iter)?; //nft creator: pda

    let nft_mint_info = next_account_info(account_info_iter)?; // NFT mint address
    let nft_account_info = next_account_info(account_info_iter)?; // account own the nft has been approve for authority
    let nft_store_info = next_account_info(account_info_iter)?; // owned by authority_info to keep NFT
    let authority_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let _metadata_program_info = next_account_info(account_info_iter)?;

    let admin_fund_info = next_account_info(account_info_iter);
    let _program_info = next_account_info(account_info_iter);

    assert_signer(&signer_info)?;

    msg!("Assert Public Key");
    let mut monster = Monster::from_account_info(monster_info)?;
    assert_incubator(&program_id, &nft_mint_info, &incubator_info)?;
    let mut incubator = Incubator::from_account_info(incubator_info)?;
    assert_eq_pubkey(&signer_info, &incubator.user)?;
    assert_eq_pubkey(&nft_account_info, &incubator.nft_return)?;

    msg!("Check Monster Hatch Time");
    if monster.hatch_time > now_timestamp() {
        return ferror!("hatching");
    };
    monster.hatch_time = now_timestamp();
    monster.level = 1;
    monster.serialize(&mut *monster_info.try_borrow_mut_data()?)?;

    msg!("Assert Store Authority");
    assert_nft_store(&program_id, &nft_mint_info, &nft_store_info)?;
    let auth_bump = assert_monster_authority(&program_id, &authority_info)?;

    msg!("Claim Token");
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

    msg!("Update Incubator");
    incubator.is_done = true;
    incubator.serialize(&mut *incubator_info.try_borrow_mut_data()?)?;

    //send fund
    send_fund_to_target(program_id, admin_fund_info.as_ref().cloned(), &signer_info, 0)?;

    Ok(())
}
