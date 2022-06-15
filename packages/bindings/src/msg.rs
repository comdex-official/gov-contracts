use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{CosmosMsg,CustomMsg,Decimal,Coin};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
/// A number of Custom messages that can call into the Comdex bindings
pub enum ComdexMessages {
    MsgWhiteListAssetLocker { app_mapping_id: u64 , asset_id : u64},
    MsgAddExtendedPairsVault{
        app_mapping_id : u64,
        pair_id : u64,
        liquidation_ratio :Decimal,
        stability_fee :Decimal,
        closing_fee :Decimal,
        liquidation_penalty :Decimal,
        draw_down_fee:Decimal,
        is_vault_active :bool,
        debt_ceiling :u64,
        debt_floor :u64,
        is_psm_pair :bool,
        min_cr :Decimal,
        pair_name :String,
        asset_out_oracle_price:bool ,
        asset_out_price:u64,
        min_usd_value_left:u64
    },
    MsgSetCollectorLookupTable{
        app_mapping_id : u64,
        collector_asset_id :u64,
        secondary_asset_id :u64,
        surplus_threshold :u64,
        debt_threshold:u64,
        locker_saving_rate :Decimal,
        lot_size :u64,
        bid_factor:Decimal,
        debt_lot_size:u64
    },
    MsgSetAuctionMappingForApp{
        app_mapping_id : u64,
        asset_id: Vec<u64>,
        is_surplus_auction : Vec<bool>,
        is_debt_auction : Vec<bool>,
        asset_out_oracle_price : Vec<bool>,
        asset_out_price : Vec<u64>,
    },

    MsgWhitelistAppIdVaultInterest{
        app_mapping_id:u64
    },
    MsgWhitelistAppIdLockerRewards
    {
        app_mapping_id:u64,
        asset_id:Vec<u64>
    },
    MsgUpdateLsrInPairsVault{
        app_mapping_id:u64,
        ext_pair_id:u64,
        liquidation_ratio:Decimal,
        stability_fee:Decimal,
        closing_fee:Decimal,
        liquidation_penalty:Decimal,
        draw_down_fee:Decimal,
        min_cr:Decimal,
        debt_ceiling:u64,
        debt_floor:u64,
        min_usd_value_left:u64
    }
    ,
    MsgUpdateLsrInCollectorLookupTable{
        app_mapping_id:u64,
        asset_id:u64,
        lsr:Decimal
    }
    ,
    MsgRemoveWhitelistAssetLocker
    {
        app_mapping_id: u64 , asset_id : u64
    }
    ,
    MsgRemoveWhitelistAppIdVaultInterest
    {
        app_mapping_id:u64
    }
    ,
    MsgWhitelistAppIdLiquidation
    {
        app_mapping_id:u64

    },
    MsgRemoveWhitelistAppIdLiquidation
    {
        app_mapping_id:u64

    },
    MsgAddAuctionParams{
        app_mapping_id: u64,
        auction_duration_seconds:u64,
        buffer:Decimal,
        cusp:Decimal,
        step:u64,
        price_function_type:u64,
        surplus_id:u64,
        debt_id:u64,
        dutch_id:u64,
        bid_duration_seconds:u64
    },
    MsgBurnToken{
        app_mapping_id:u64,
        module :String,
        amount :Coin,
        from_address: String
    }



}

impl From<ComdexMessages> for CosmosMsg<ComdexMessages> {
    fn from(msg: ComdexMessages) -> CosmosMsg<ComdexMessages> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for ComdexMessages {}
