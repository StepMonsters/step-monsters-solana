use borsh::BorshSerialize;
use mpl_token_metadata::instruction::update_metadata_accounts_v2;
use mpl_token_metadata::state::DataV2;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    program::invoke,
    pubkey::Pubkey,
};

use crate::{state::*, ferror, utils::*};

pub fn process_claim_monster(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    let metadata_program_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let monster_info = next_account_info(account_info_iter)?;

    let nft_mint_info = next_account_info(account_info_iter)?; // NFT mint address
    let nft_account_info = next_account_info(account_info_iter)?; // account own the nft has been approve for authority
    let nft_store_info = next_account_info(account_info_iter)?; // owned by authority_info to keep NFT
    let authority_info = next_account_info(account_info_iter)?;


    assert_signer(&signer_info)?;

    msg!("Update Monster Info");
    let mut monster = Monster::from_account_info(monster_info)?;

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
    let creator = vec![
        mpl_token_metadata::state::Creator {
            address: *signer_info.key,
            verified: false,
            share: 100,
        },
    ];
    let name = String::from("my_name");
    let symbol = String::from("my_symbol");
    let uri = String::from("https://arweave.net/y5e5DJsiwH0s_ayfMwYk-SnrZtVZzHLQDSTZ5dNRUHA");
    let data = DataV2 {
        name,
        symbol,
        uri,
        seller_fee_basis_points: 1,
        creators: Some(creator),
        collection: None,
        uses: None,
    };

    invoke(
        &update_metadata_accounts_v2(
            *metadata_program_info.key,
            *metadata_info.key,
            *signer_info.key,
            Some(*signer_info.key),
            Some(data),
            Some(false),
            Some(false),
        ),
        &[
            metadata_info.clone(),
            signer_info.clone(),
            metadata_program_info.clone(),
            token_program_info.clone(),
            system_info.clone(),
        ],
    )?;

    Ok(())
}
