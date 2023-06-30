use borsh::{BorshDeserialize, BorshSerialize};
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
    CreateReferralInfo(CreateReferralInfoArgs)
}
