use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{ferror, state::*, utils::*};
use crate::utils_mint::create_referral_info;

pub fn process_create_referral_info(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: CreateReferralInfoArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let referral_info = next_account_info(account_info_iter)?;
    let father_info = next_account_info(account_info_iter)?;
    let father_ref_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

    let ref_code = args.ref_code;
    let user_addr = signer_info.key.to_string();
    let sub01 = &user_addr[0..2].to_string();
    let sub02 = &user_addr[38..44].to_string();
    let addr_code = sub01.to_owned() + sub02;
    if ref_code != addr_code {
        return ferror!("Invalid ref code.");
    }

    if referral_info.lamports() <= 0 {
        create_referral_info(
            &program_id,
            &ref_code,
            &referral_info,
            &rent_info,
            &system_info,
            &signer_info,
        )?;
    };

    msg!("User Referral Info");
    let mut ref_info = ReferralInfo::from_account_info(referral_info)?;
    ref_info.addr = *signer_info.key;
    ref_info.father_addr = *father_info.key;
    ref_info.ref_code = ref_code;
    ref_info.serialize(&mut *referral_info.try_borrow_mut_data()?)?;

    msg!("Father Referral Info");
    if father_ref_info.key != referral_info.key {
        let mut father_ref = ReferralInfo::from_account_info(father_ref_info)?;
        father_ref.invited += 1;
        father_ref.serialize(&mut *father_ref_info.try_borrow_mut_data()?)?;
    }

    Ok(())
}
