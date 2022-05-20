# market contract interface

program_id: 

## Instruction Call

### Create

（固定拍）创建拍卖，需要提前将nft_creator加入白名单

| AccountInfo      | is_signer | is_writable |                                                           |
| ---------------- | --------- | ----------- | --------------------------------------------------------- |
| signer           | **true**  | false       | 交易发起者，即钱包地址                                    |
| config           | false     | false       | ['fallen', program_id, 'configure']       
| user_info        | false     | **true**    | [program_id, user, 'user_info']              
| nft_creator      | false     | false       | nft creator，通常是钱包地址                               |
| nft_creator_data | false     | false       | ['fallen', program_id, nft_creator,'creator_whitelist'] |
| auction          | **true**  | **true**    | 新建地址                                                  |
| authority        | false     | false       | ['fallen', program_id, auction,'authority']             |
| nft_mint         | false     | false       | nft mint 地址                                             |
| nft_metadata     | false     | false       | nft metadata 地址，是一个PDA                              |
| nft_account      | false     | **true**    | 拥有nft的地址，并且已经approve给authority，数量为1        |
| nft_store        | false     | **true**    | ['fallen', program_id, auction,'nft_store']             |
| bid_store        | false     | **true**    | ['fallen', program_id, auction,'bid_store']             |
| spl_token_id     | false     | false       | 标准spl token id                                          |
| rent             | false     | false       | 系统rent                                                  |
| system           | false     | false       | 系统 system                                               |

```
指令下标：2

指令参数：
pub struct CreateArgs {
    pub sale_type: u8,           -- 1：FixedPriceSale，2：EnglishAuction
    pub price: Option<u64>,		   -- FixedPriceSale 指定价，单位：lamports
    pub min_price: Option<u64>,  -- EnglishAuction 最低价，单位：lamports
    pub duration: Option<u64>,   -- EnglishAuction 持续时间，单位：秒；从创建拍卖开始计时
    pub begin_ts: Option<u64>,	 -- FixedPriceSale或EnglishAuction的拍卖开始时间戳
}
```

### PlaceBid

（固定拍）拍卖出价

| AccountInfo  | is_signer | is_writable |                                                              |
| ------------ | --------- | ----------- | ------------------------------------------------------------ |
| signer       | **true**  | false       | 同上                                                         |
| charge_addr  | false     | **true**    | 同上                                                         |
| team_addr    | false     | **true**    | 同上                                                         |
| config_info  | false     | **true**    |['fallen', program_id, 'configure']
| user_info    | false     | **true**        | [program_id, signer 'user_info']         
| auction_creator_user_info    | false     | **true**        | [program_id, auction_creator 'user_info']         
| auction      | false     | **true**    | 同上                                                         |
| authority    | false     | false       | 同上                                                         |
| bid_info     | false     | **true**    | ['fallen', program_id, auction,signer,'bid']               |
| bid_store    | false     | **true**    | 同上                                                         |
| auction_creator    | false     | **true**    | 同上                                                   |
| nft_store        | false     | **true**    | ['fallen', program_id, auction,'nft_store']             |
| nft_return   | false     | **true**    | 用于接收取消拍卖返回nft的地址；理论上是Create中nft_account |
| last_bidder | false     | **true**       | 同上                                                         |
| nft_metadata | false     | false       | 同上                                                         |
| spl_token_id | false     | false       | 同上                                                         |
| rent         | false     | false       | 同上                                                         |
| system       | false     | false       | 系统 system                                                  |
| nft_creators_arr       | false     | false       | nft 的创建者们                                     |

```
指令下标：3

指令参数：
pub struct PlaceBidArgs {
    pub price: u64,             -- 参拍价格，单位：lamports
}
```

### Cancel

（固定拍 / 英式拍）拍卖取消，固定拍只要无人出价，即可取消；英式拍，需要等拍卖时间结束后才可取消

| AccountInfo  | is_signer | is_writable |                                                              |
| ------------ | --------- | ----------- | ------------------------------------------------------------ |
| signer       | **true**  | false       | 同上                                                         |
| auction      | false     | **true**    | 同上                                                         |
| authority    | false     | false       | 同上                                                         |
| nft_store    | false     | **true**    | 同上                                                         |
| nft_return   | false     | **true**    | 用于接收取消拍卖返回nft的地址；owner为signer；理论上是Create中nft_account |
| spl_token_id | false     | false       | 同上                                                         |

```

指令下标：4
```
## 接口查询

### 查询全局配置

```
数据地址：Create 指令中 config

数据结构：
pub struct ConfigureArgs {
    pub is_initialized: bool,    -- false 不允许创建拍卖
    pub authority: Pubkey,       -- 管理员地址
    pub charge_rate: u64,        -- 手续费率，放大10000倍
    pub charge_addr: Pubkey,     -- 收取手续费地址的地址
}
```

### 查询是否白名单

```
数据地址：参考 Create 指令中 nft_creator_data，若无数据，则非白名单；有数据情况下也要看值是否为true

数据结构：
pub struct SetCreatorWhitelistArgs {
    pub is_activated: bool,      -- true：白名单，false：禁止创建拍卖
    pub total_supply: u64,       -- collection 下总nft数
    pub seller_fee: u64,         -- 版税
    pub symbol:  String,         -- symbol
}
```
数据结构：
pub struct ConfigureData {
    /// Initialized state.
    pub is_initialized: bool,
    /// Contract admin
    pub authority: Pubkey,
    /// Charge rate (* 10000) of auction deal
    pub charge_rate: u64,
    /// Charge address with mint of WSOL
    pub charge_addr: Pubkey,
}

数据结构：
pub struct UserInfo {
    pub total_trade: u64,     --总交易量
    pub daily_trade: u64,     --当前天交易量
}