use near_sdk::ONE_NEAR;
use near_sdk::{AccountId, Balance};

pub mod stake_pool;
mod account_balance;
mod account_registry;
mod cross_contract_call;
mod data_transfer_object;
mod delayed_withdrawal;
mod delayed_withdrawn_fund;
mod fee_registry;
mod fee;
mod fund;
mod fungible_token;
mod investment_withdrawal;
mod investor_investment;
mod reward;
mod shared_fee;
mod stake_decreasing_kind;
mod staking_contract_version;
mod storage_key;
mod validating;
mod validator_balance;
mod validator;

/// Quantity of epochs after passing which it is possible to withdraw fund.
const EPOCH_QUANTITY_FOR_DELAYED_WITHDRAWAL: u64 = 8;
/// Quantity of epochs after passing which it is possible to request unstake from validator.
const EPOCH_QUANTITY_FOR_VALIDATOR_UNSTAKE: u64 = 4;
/// Minimum Gas amount for common transaction.
const MINIMUM_NUMBER_OF_TGAS: u64 = 300;
/// The minimum Near amount that must be attached to a transaction.
const MINIMUN_DEPOSIT_AMOUNT: Balance = ONE_NEAR;
/// Maximum number of characters in account name, it is needed for storage staking calculating.
const MAXIMUM_NUMBER_OF_CHARACTERS_IN_ACCOUNT_NAME: usize = 64;

fn get_account_id_with_maximum_length() -> AccountId {
    AccountId::new_unchecked("a".repeat(MAXIMUM_NUMBER_OF_CHARACTERS_IN_ACCOUNT_NAME))
}