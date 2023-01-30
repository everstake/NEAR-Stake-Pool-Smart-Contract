use near_sdk::{env, StorageUsage, AccountId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, LookupMap};
use super::get_account_id_with_maximum_length;
use super::investor_investment::InvestorInvestment;
use super::staking_contract_version::StakingContractVersion;
use super::storage_key::StorageKey;
use super::validator::Validator;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Validating {
    /// Storage.
    /// AccountId - validator account id.
    pub validator_registry: UnorderedMap<AccountId, Validator>,
    /// Registry of investors who are allowed to make an deposit/withdrawal directly on/from the validator.
    pub investor_investment_registry: LookupMap<AccountId, InvestorInvestment>,
    pub validators_quantity: u64,
    pub preffered_validator: Option<AccountId>,
    pub quantity_of_validators_updated_in_current_epoch: u64,
    /// In bytes.
    pub storage_usage_per_validator: StorageUsage,
    /// In bytes.
    pub storage_usage_per_investor_investment: StorageUsage,
    /// In bytes.
    pub storage_usage_per_distribution: StorageUsage
}

impl Validating {
    pub fn new() -> Self {
        Self {
            validator_registry: Self::initialize_validator_registry(),
            investor_investment_registry: Self::initialize_investor_investment_registry(),
            validators_quantity: 0,
            preffered_validator: None,
            quantity_of_validators_updated_in_current_epoch: 0,
            storage_usage_per_validator: Self::calculate_storage_usage_per_additional_validator(),
            storage_usage_per_investor_investment: Self::calculate_storage_usage_per_additional_investor_investment(),
            storage_usage_per_distribution: Self::calculate_storage_usage_per_additional_distribution()
        }
    }

    fn calculate_storage_usage_per_additional_validator() -> StorageUsage {
        let mut validator_registry = Self::initialize_validator_registry();

        let initial_storage_usage = env::storage_usage();

        let account_id = get_account_id_with_maximum_length();

        validator_registry.insert(
            &account_id, &Validator::new(StakingContractVersion::Core, false)
        );

        env::storage_usage() - initial_storage_usage
    }

    fn calculate_storage_usage_per_additional_investor_investment() -> StorageUsage {
        let mut investor_investment_registry = Self::initialize_investor_investment_registry();

        let initial_storage_usage = env::storage_usage();

        let account_id = get_account_id_with_maximum_length();

        investor_investment_registry.insert(&account_id, &InvestorInvestment::new(account_id.clone()));

        env::storage_usage() - initial_storage_usage
    }

    fn calculate_storage_usage_per_additional_distribution() -> StorageUsage {
        let account_id = get_account_id_with_maximum_length();

        let mut distribution_registry = InvestorInvestment::initialize_distribution_registry(account_id.clone());

        let initial_storage_usage = env::storage_usage();

        distribution_registry.insert(&account_id, &0);

        env::storage_usage() - initial_storage_usage
    }

    fn initialize_validator_registry() -> UnorderedMap<AccountId, Validator> {
        UnorderedMap::new(StorageKey::Validator)
    }

    fn initialize_investor_investment_registry() -> LookupMap<AccountId, InvestorInvestment> {
        LookupMap::new(StorageKey::InvestorInvestment)
    }
}