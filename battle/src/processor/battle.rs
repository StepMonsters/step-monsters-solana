use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

use nft_daniel::{
    state::*,
    instruction::mint
};

use crate::{ferror, utils::*};

pub fn process_battle(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let pda_creator_info = next_account_info(account_info_iter)?; //nft creator: pda
    let fee_recevier_info = next_account_info(account_info_iter)?; // fee_recevier: wallet
    let admin_info = next_account_info(account_info_iter)?; //admin signer
    let mint_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let edition_info = next_account_info(account_info_iter)?;
    let nft_program_info = next_account_info(account_info_iter)?;
    let metadata_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    
    

    // let monster_info_attacker = next_account_info(account_info_iter)?;
    // let monster_info_defender = next_account_info(account_info_iter)?;
    // let battle_info = next_account_info(account_info_iter)?;

    assert_signer(&admin_info)?;
    assert_signer(&signer_info)?;

    // do battle logic
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
    //after battle logic do invoke mint_nft
    invoke(
        &mint(
            &nft_program_info.key,
            &signer_info.key,
            &config_info.key,
            &pda_creator_info.key,
            &fee_recevier_info.key,
            &mint_info.key,
            &metadata_info.key,
            &edition_info.key,
            &metadata_program_info.key,
            &token_program_info.key,
        )?,
        &[
            nft_program_info.clone(),
            signer_info.clone(),
            config_info.clone(),
            pda_creator_info.clone(),
            fee_recevier_info.clone(),
            mint_info.clone(),
            metadata_info.clone(),
            edition_info.clone(),
            metadata_program_info.clone(),
            token_program_info.clone(),
            rent_info.clone(),
            system_info.clone(),
        ],
    )?;

    //if need hatch then invoke hatch

    // battle.serialize(&mut *battle_info.try_borrow_mut_data()?)?;

    Ok(())
}
