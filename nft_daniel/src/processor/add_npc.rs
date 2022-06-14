use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar,
};

use crate::{ferror, state::*, utils::*};

pub fn process_add_npc(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: AddNPCArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let npc_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;
    assert_config(&program_id, &config_info)?;
    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    let config_data = ConfigureData::from_account_info(config_info)?;

    if config_data.authority != *signer_info.key {
        return ferror!("invalid authority");
    }

    let path = args.npc_id.to_string();
    let bump = assert_derivation(
        program_id,
        npc_info,
        &[path.as_bytes(), program_id.as_ref()],
    )?;
    let npc_seed = &[
        path.as_bytes(),
        program_id.as_ref(),
        &[bump],
    ];

    msg!("Create Npc Info");
    if npc_info.data_is_empty() {
        create_or_allocate_account_raw(
            *program_id,
            npc_info,
            rent_info,
            system_info,
            signer_info,
            NPCMonsterData::LEN,
            npc_seed,
        )?;
    }

    let mut npc = NPCMonsterData::from_account_info(npc_info)?;
    npc.npc_id = args.npc_id;
    npc.hp = args.hp;
    npc.attack = args.attack;
    npc.defense = args.defense;

    npc.serialize(&mut *npc_info.try_borrow_mut_data()?)?;

    Ok(())
}
