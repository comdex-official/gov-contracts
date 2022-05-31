use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{CosmosMsg,CustomMsg};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
/// A number of Custom messages that can call into the Comdex bindings
pub enum ComdexMessages {
    MsgWhiteListAssetLocker { app_mapping_id: u64 , asset_id : u64},
    // AddPairVault{
    //     app_mapping_id : u64,
    //     pair_id : u64,
    //     liquidation_ratio :String,
    //     stability_fee :String,
    //     closing_fee :String,
    //     liquidation_penalty :String,
    //     draw_down_fee:String,
    //     is_vault_active :bool,
    //     debt_cieling :String,
    //     debt_floor :String,
    //     is_psm_pair :bool,
    //     min_cr :String,
    //     pair_name :String,
    //     asset_out_oracle_price:bool ,
    //     assset_out_price:u64,
    // },
    // CollectorLookupParam{
    //     app_mapping_id : u64,
    //     collector_asset_id :u64,
    //     secondary_asset_id :u64,
    //     surplus_threshold :u64,
    //     debt_threshold:String,
    //     locker_saving_rate :String,
    //     lot_size :u64,
    //     bid_factor:String,
    // },
    // AuctionControl{
    //     app_mapping_id : u64,
    //     asset_id: u64,
    //     surplus_auction : bool,
    //     debt_auction : bool,
    // },
    // RewardWhiteListAsset{
    //     app_mapping_id : u64,
    //     asset_id: u64,
    // } 
    MsgWhitelistAppIdVaultInterest{
        app_mapping_id:u64
    },
    MsgWhitelistAppIdLockerRewards
    {
        app_mapping_id:u64,
        asset_id:Vec<u64>
    }
}

impl From<ComdexMessages> for CosmosMsg<ComdexMessages> {
    fn from(msg: ComdexMessages) -> CosmosMsg<ComdexMessages> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for ComdexMessages {}
