use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use super::fee_registry_light::FeeRegistryLight;
use super::fund::Fund;
use super::requested_to_withdrawal_fund::RequestedToWithdrawalFund;
use super::storage_staking_price::StorageStakingPrice;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Full {
    pub storage_staking_price: StorageStakingPrice,
    pub fund: Fund,
    pub total_token_supply: U128,
    pub requested_to_withdrawal_fund: RequestedToWithdrawalFund,
    pub fee_registry_light: FeeRegistryLight,
    pub minimum_deposit_amount: U128
}