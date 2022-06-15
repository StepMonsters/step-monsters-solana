use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar,
};

use crate::{ferror, state::*, utils::*};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};

pub fn process_battle(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: BattleArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let pda_creator_info = next_account_info(account_info_iter)?; //nft creator: pda
    let admin_info = next_account_info(account_info_iter)?; //admin signer
    let mint_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let edition_info = next_account_info(account_info_iter)?;
    let metadata_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    // let monster_info_attacker = next_account_info(account_info_iter)?;
    // let monster_info_defender = next_account_info(account_info_iter)?;
    // let battle_info = next_account_info(account_info_iter)?;

    assert_signer(&admin_info)?;
    assert_signer(&signer_info)?;

    assert_eq_pubkey(&metadata_program_info, &mpl_token_metadata::id())?;
    assert_eq_pubkey(&token_program_info, &spl_token::id())?;
    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;

    let pda_bump = assert_pda_creator(&program_id, pda_creator_info)?;

    let pda_seed = [
        SEED_BATTLE.as_bytes(),
        program_id.as_ref(),
        "pda_creator".as_bytes(),
        &[pda_bump],
    ];

    let mut state = true;
    // todo battle logic
    // let attacker = Monster::from_account_info(monster_info_attacker)?;
    // let defender = Monster::from_account_info(monster_info_defender)?;

    // let mut battle = crate::state::Battle::from_account_info(battle_info)?;
    // if attacker.attack == defender.defense {
    //     battle.winner = 0;
    // } else if attacker.attack > defender.defense {
    //     battle.winner = 1;
    // } else {
    //     battle.winner = 2;
    // }

    //after battle logic do  mint_nft
    if state {
        let config_data = ConfigureData::from_account_info(config_info)?;
        let creators = vec![
            mpl_token_metadata::state::Creator {
                address: *pda_creator_info.key,
                verified: true,
                share: 0,
            },
            mpl_token_metadata::state::Creator {
                address: config_data.creator,
                verified: false,
                share: 100,
            },
        ];
        msg!("Create metadata");
        invoke_signed(
            &create_metadata_accounts_v2(
                *metadata_program_info.key,
                *metadata_info.key,
                *mint_info.key,
                *signer_info.key,
                *signer_info.key,
                *pda_creator_info.key, //pda must be signer
                config_data.name,
                config_data.symbol,
                config_data.uri,
                Some(creators),
                config_data.fee,
                true,
                true,
                None,
                None,
            ),
            &[
                metadata_info.clone(),
                mint_info.clone(),
                signer_info.clone(),
                metadata_program_info.clone(),
                token_program_info.clone(),
                system_info.clone(),
                rent_info.clone(),
                pda_creator_info.clone(),
            ],
            &[&pda_seed],
        )?;
        msg!("Create Master Edition");
        invoke_signed(
            &create_master_edition_v3(
                *metadata_program_info.key,
                *edition_info.key,
                *mint_info.key,
                *pda_creator_info.key,
                *signer_info.key,
                *metadata_info.key,
                *signer_info.key,
                Some(1),
            ),
            &[
                edition_info.clone(),
                mint_info.clone(),
                signer_info.clone(),
                metadata_info.clone(),
                metadata_program_info.clone(),
                token_program_info.clone(),
                system_info.clone(),
                rent_info.clone(),
                pda_creator_info.clone(),
            ],
            &[&pda_seed],
        )?;
    }

    //if need hatch then do hatch

    // battle.serialize(&mut *battle_info.try_borrow_mut_data()?)?;

    Ok(())
}
