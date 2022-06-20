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

配置设置

| AccountInfo      | is_signer | is_writable |                                                           |
| ---------------- | --------- | ----------- | --------------------------------------------------------- |
| signer           | **true**  | false       | 交易发起者，即钱包地址                                    |
| config           | false     | false       | ['battle', program_id, 'configure']                  
| rent             | false     | false       | 系统rent                                                  |
| system           | false     | false       | 系统 system                                               |

```
指令下标：0

指令参数：
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


## 接口查询

### 查询全局配置

```
数据结构：
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

