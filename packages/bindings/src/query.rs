use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CustomQuery,Coin, Decimal };

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ComdexQuery {
    

    TotalSupply { app_mapping_id:u64 ,asset_id : u64 },
  
    State { address: String ,height :String , denom:String,target :String},

    GetApp{app_mapping_id: u64},

    GetAssetData{asset_id:u64},

    WhiteListedAssetQuery{app_mapping_id:u64 ,asset_id : u64 },

    
    WhitelistAppIdLockerRewards{app_mapping_id :u64, asset_id:Vec<u64>},
    
    WhitelistAppIdVaultInterest{app_mapping_id :u64},
    
    RemoveWhiteListAsset{app_mapping_id:u64,asset_id:Vec<u64>},

    CollectorLookupTableQuery{
        app_mapping_id:u64,
        collector_asset_id:u64,
        secondary_asset_id:u64
    },

    ExtendedPairsVaultRecordsQuery{
        app_mapping_id :u64,
        pair_id:u64,
        stability_fee:Decimal,
        closing_fee:Decimal,
        draw_down_fee:Decimal,
        debt_ceiling:u64,
        debt_floor:u64,
        pair_name:String
    },
    

    UpdateLsrInPairsVaultQuery{ 
        app_mapping_id:u64,
        ext_pair_id:u64
    },


    AuctionMappingForAppQuery{app_mapping_id:u64},

    UpdateLsrInCollectorLookupTableQuery{app_mapping_id:u64,asset_id:u64}
}

impl CustomQuery for ComdexQuery {}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct StateResponse {
    
    pub amount: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TotalSupplyResponse {
    
   
    pub current_supply:u64,

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetAppResponse {
    
    pub min_gov_deposit :u64,
    pub gov_time_in_seconds: u64,
    pub gov_token_id : u64,
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetAssetDataResponse {
    
    pub denom : String,
    
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MessageValidateResponse {
    
    pub found: bool,
    pub err : String
}



