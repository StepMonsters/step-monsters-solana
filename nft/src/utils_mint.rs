use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::state::{MAX_MONSTER_LENGTH, SEED_MONSTER};
use crate::utils::{assert_derivation, create_or_allocate_account_raw};

pub fn create_monster_info<'a>(
    program_id: &Pubkey,
    monster_info: &AccountInfo<'a>,
    mint_info: &AccountInfo<'a>,
    rent_info: &AccountInfo<'a>,
    system_info: &AccountInfo<'a>,
    signer_info: &AccountInfo<'a>
) -> Result<(), ProgramError> {
    let bump_seed = assert_derivation(
        program_id,
        monster_info,
        &[
            SEED_MONSTER.as_bytes(),
            program_id.as_ref(),
            &mint_info.key.as_ref(),
        ],
    )?;
    let monster_seeds = &[
        SEED_MONSTER.as_bytes(),
        program_id.as_ref(),
        &mint_info.key.as_ref(),
        &[bump_seed],
    ];
    create_or_allocate_account_raw(
        *program_id,
        monster_info,
        rent_info,
        system_info,
        signer_info,
        MAX_MONSTER_LENGTH,
        monster_seeds,
    )?;

    Ok(())
}