# market contract interface

program_id: 

## Instruction Call

```
pub enum AppInstruction {
    /// Setting auction global configuration
    Configure(ConfigureArgs),
    /// Set whitelist of nft creator, then those can call `Create`
    SetCreatorWhitelist(SetCreatorWhitelistArgs),
    /// Create fixed price sale or english auction
    Create(CreateArgs),
    /// Bid fixed price sale or english auction
    PlaceBid(PlaceBidArgs),
    ///change price
    ChangePrice(ChangePriceArgs),
    /// Cancel fixed price sale or english auction
    Cancel,
    ///make offer
    MakeOffer(MakeOfferArgs),
    ///cancel offer
    CancelOffer,
    ///accept offer
    AcceptOffer
}
```
### Create

Create Sale - the nft_creator need to be added into whitelist

| AccountInfo      | is_signer | is_writable |                                                           |
| ---------------- | --------- | ----------- | --------------------------------------------------------- |
| signer           | **true**  | false       | transaction signer                                       |
| config           | false     | false       | ['market', program_id, 'configure']                  
| nft_creator      | false     | false       | nft creator - usually a wallet address                   |
| nft_creator_data | false     | false       | ['market', program_id, nft_creator,'creator_whitelist']  |
| auction          | **true**  | **true**    | new auction address                                      |
| authority        | false     | false       | ['market', program_id, auction,'authority']              |
| nft_mint         | false     | false       | nft mint address                                         |
| nft_metadata     | false     | false       | nft metadata pda address                                 |
| nft_account      | false     | **true**    | address owned nft which is approved for authority        |
| nft_store        | false     | **true**    | ['market', program_id, auction,'nft_store']              |
| spl_token_id     | false     | false       | spl token id                                             |
| rent             | false     | false       | system rent                                              |
| system           | false     | false       | system                                                   |

```
Instruction Key: 2

Instruction Arguments:
pub struct CreateArgs {
    pub price: Option<u64>,		   -- FixedPriceSale, unit: lamports
    pub duration: Option<u64>,   -- EnglishAuction, auction duration, unit: second
    pub begin_ts: Option<u64>,	 -- FixedPriceSale or EnglishAuction start timestamp
}
```

### PlaceBid

Place Bid

| AccountInfo  | is_signer | is_writable |                                                              |
| ------------ | --------- | ----------- | ------------------------------------------------------------ |
| signer       | **true**  | false       | transaction signer                                           |
| charge_addr  | false     | **true**    | charger address                                              |
| config_info  | false     | **true**    |['market', program_id, 'configure']              
| auction      | false     | **true**    | auction pda address                                          |
| authority    | false     | false       | authority pda address                                        |
| bid_info     | false     | **true**    | ['market', program_id, auction,signer,'bid']                 |
| auction_creator    | false     | **true**    | auction creator pda address                            |
| nft_store        | false     | **true**    | ['market', program_id, auction,'nft_store']              |
| nft_return   | false     | **true**    | address for receive nft return - default nft_account         |
| nft_metadata | false     | false       | nft metadata pda address                                     |
| spl_token_id | false     | false       | spl token id                                                 |
| rent         | false     | false       | system rent                                                  |
| system       | false     | false       | system                                                       |
| nft_creators_arr       | false     | false       | all nft creators                                   |

```
Instruction Key：3

Instruction Arguments：
pub struct PlaceBidArgs {
    pub price: u64,             -- place bid price, unit: lamports
}
```

### ChangePrice

change sale price, if there are no bids.

| AccountInfo  | is_signer | is_writable |                                                              |
| ------------ | --------- | ----------- | ------------------------------------------------------------ |
| signer       | **true**  | false       | transaction signer                                           |
| auction      | false     | **true**    | auction pda address                                          |

```
Instruction Key：4

Instruction Arguments：
pub struct ChangePriceArgs {
    pub price: u64,             -- price, unit: lamports
}
```

### Cancel

cancel the sale, if there are no bids.

| AccountInfo  | is_signer | is_writable |                                                              |
| ------------ | --------- | ----------- | ------------------------------------------------------------ |
| signer       | **true**  | false       | transaction signer                                           |
| auction      | false     | **true**    | auction pda address                                          |
| authority    | false     | false       | authority pda address                                        |
| nft_store    | false     | **true**    | address where nft store                                      |
| nft_return   | false     | **true**    | address receive nft return, default nft_creator              |
| spl_token_id | false     | false       | spl token id                                                 |

```
Instruction Key: 5
```

### MakeOffer

make offer

| AccountInfo  | is_signer | is_writable |                                                              |
| ------------ | --------- | ----------- | ------------------------------------------------------------ |
| signer       | **true**  | false       | transaction signer                                           |
| nft           | false     | false       | nft mint address                                            |
| nft_return   | false     | **true**    | address receive nft return                                   |
| offer         | false     | **true**    | new offer - new keypair                                     |
| bid_store     | false     | **true**    | ['market', program_id, offer,'bid_store']                   |
| spl_token_id | false     | false       | spl token id                                                           |

```
Instruction Key: 6

Instruction Arguments:
pub struct MakeOfferArgs {
    /// Price of offer
    pub price: u64,
}

```


### CancelOffer

cancel the offer

| AccountInfo  | is_signer | is_writable |                                                              |
| ------------ | --------- | ----------- | ------------------------------------------------------------ |
| signer       | **true**  | false       | transaction signer                                           |
| nft      | false     | false          | nft mint address                                              |
| nft_return   | false     | **true**    | address receive nft return                                   |
| offer   | false     | **true**    | offer  |
| bid_store   | false     | **true**    | ['market', program_id, offer,'bid_store']                     |
| spl_token_id | false     | false       | spl token id                                                 |

```
Instruction Key：7
```


### AcceptOffer

accept the offer

| AccountInfo  | is_signer | is_writable |                                                              |
| ------------ | --------- | ----------- | ------------------------------------------------------------ |
| signer       | **true**  | false       | transaction signer                                           |
| config_info  | false     | **true**    |['market', program_id, 'configure']     
| charge_addr  | false     | **true**    | charger address                                              |
| nft_account_info      | false     | **true**   | seller nft account                                   |
| nft_return_info    | false     | **true**       | receiver nft account                                |
| nft_metadata | false     | false       | nft metadata pda address                                     |
| offerer    | false     | **true**    | buyer address                                                  |
| offer        | false     | **true**    | offer account                                                |
| bid_store   | false     | **true**    | ['market', program_id, offer,'bid_store']                     |
| spl_token_id | false     | false       | spl token id                                                 |
| rent         | false     | false       | system rent                                                  |
| system       | false     | false       | system                                                       |
| nft_creators_arr       | false     | false       | all nft creators                                   |


```
Instruction Key：8
```
## API

### Query Global Configure

```
Data Structure:
pub struct ConfigureData {
    /// Initialized state.
    pub is_initialized: bool,
    /// Contract admin
    pub authority: Pubkey,
    /// Charge rate (* 10000) of auction deal
    pub charge_rate: u64,
    /// Charge address 
    pub charge_addr: Pubkey,
}
```

### Query Whitelist

```
refer to nft_creator_data in Create Instruction.
if there is no data, it is not in whitelist.
if there is data, check the value is true.

Data Structure:
pub struct SetCreatorWhitelistArgs {
    pub is_activated: bool,      -- true: whitelist, false: cannot create sale
    pub total_supply: u64,       -- total supply of nft in collection
    pub seller_fee: u64,         -- seller fee or royalty
    pub symbol:  String,         -- symbol 
}
```

### Query Auction Data

```
Data Structure:
pub struct AuctionData {
    pub timestamp: u64,
    pub is_init: bool,
    /// creator
    pub creator: Pubkey,
    /// NFT mint address
    pub nft_mint: Pubkey,
    /// NFT store by auction
    pub nft_store: Pubkey,
    /// fixed price sale
    /// Price of sale
    pub price: Option<u64>,
    /// Auction begin at unix timestamp
    pub begin_ts: Option<u64>,
    /// Auction duration, unix seconds
    pub duration: Option<u64>,
    /// fixed price    : be true after creator cancel or claim token
    pub is_claim: bool,
    /// Last bid
    pub last_bid: Option<BidData>,
}
```
### Query Offer Data

```
Data Structure:
pub struct OfferData {
    pub timestamp: u64,
    pub offerer: Pubkey,
    pub nft: Pubkey,
    pub nft_return: Pubkey,
    pub price: u64,
    pub is_canceled: bool,
    pub is_done: bool,
}

```