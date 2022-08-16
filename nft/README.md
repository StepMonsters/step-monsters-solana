# market contract interface

program_id: 

## Instruction Call

```
pub enum GameInstruction {
    Configure(ConfigureArgs),
    Mint,
    Hatch,
    Breed,
    Synthesis,
    Upgrade,
    Battle(BattleArgs),
    CreateGameConfig(),
    UpdateGameConfig(),
    CreateMonsterFeatureConfig(),
}
```
### Config

Configure Account Info

| AccountInfo      | is_signer | is_writable |                                                           |
| ---------------- | --------- | ----------- | --------------------------------------------------------- |
| signer           | **true**  | false       | transaction signer                                       |
| config           | false     | false       | ['battle', program_id, 'configure']                  
| rent             | false     | false       | system rent                                                  |
| system           | false     | false       | system                                               |

```
Instruction Key: 0

Instruction Arguments:
pub struct ConfigureArgs {
    /// Initialized state.
    pub is_initialized: bool,
    /// Contract admin
    pub authority: Pubkey,
    /// creator
    pub creator: Pubkey,
    /// fee_recevier
    pub fee_recevier: Pubkey,
    /// nft price
    pub price: u64,
    /// seller fee
    pub fee: u16,
    /// nft name
    pub name: String,
    /// nft symbol
    pub symbol: String,
    /// default uri
    pub uri: String,
}
```


## API

### Global Configure

```
data structure:
pub struct ConfigureData {
    /// Initialized state.
    pub is_initialized: bool,
    /// Contract admin
    pub authority: Pubkey,
    /// creator
    pub creator: Pubkey,
    /// fee_recevier
    pub fee_recevier: Pubkey,
    /// nft price
    pub price: u64,
    /// seller fee
    pub fee: u16,
    /// nft name
    pub name: String,
    /// nft symbol
    pub symbol: String,
    /// default uri
    pub uri: String,
}
```

