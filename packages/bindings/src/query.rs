use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CustomQuery,Coin };

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ComdexQuery {
    

    TotalSupply { app_id:u64 ,asset_id : u64 },
  
    State { address: String ,height :String , denom:String,target :String},

    GetApp{app_mapping_id: u64},

    GetAssetData{asset_id:u64},
    
    WhitelistAppIdLockerRewards{app_id :u64, asset_id:Vec<u64>},
    WhitelistAppIdVaultInterest{app_id :u64},
    RemoveWhiteListAsset{app_mapping_id:u64,
        asset_id:Vec<u64>}
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



