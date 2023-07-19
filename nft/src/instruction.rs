use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use crate::state::*;

#[repr(C)]
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum GameInstruction {
    Configure(ConfigureArgs),
    InitMint,
    Mint(MintArgs),
    QuickHatch,
    Hatch,
    ClaimMonster(ClaimMonsterArgs),
    Breed,
    Synthesis,
    Upgrade,
    Battle(BattleArgs),
    CreateGameConfig(),
    UpdateGameConfig(),
    Cure(CureArgs),
    TransferToSpending(TransferSpendingArgs),
    TransferFromSpending(TransferSpendingArgs),
    TransferFromSpendingTemp(TransferSpendingArgs),
    CreateMonsterFeatureConfig,
    QuickMint(QuickMintArgs),
    CreateToken,
    MintToken,
    Recycle(RecycleArgs),
    Revive(ReviveArgs),
    CreateCollection(QuickMintArgs),
    UpdateMetadata,
    CreateReferralInfo(CreateReferralInfoArgs),
    BattleWithRef(BattleArgs),
    CreateAdminFundAccount,
    SendFund(SendFundArgs)
}

pub fn call_send_fund(
    program_id: &Pubkey,
    signer: &Pubkey,
    fund_info: &Pubkey,
    args: SendFundArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(*fund_info, false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: GameInstruction::SendFund(args).try_to_vec().unwrap(),
    })
}

