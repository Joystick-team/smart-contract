
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 115 120'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M101.328 93.6259L78.5501 106.777L55.7722 119.927L32.9942 106.777L10.2173 93.6259V82.7107C4.21536 80.183 0 74.2464 0 67.3248C0 60.4032 4.2144 54.4666 10.2163 51.9389V41.0237L32.9942 27.8726L55.7722 14.7226L78.5491 27.8726L101.327 41.0237V67.3248V93.6259H101.328ZM96.3312 22.417C98.9587 22.417 101.089 24.5472 101.089 27.1747C101.089 29.8022 98.9587 31.9325 96.3312 31.9325C93.7037 31.9325 91.5734 29.8022 91.5734 27.1747C91.5734 24.5472 93.7037 22.417 96.3312 22.417ZM96.3312 23.0976C98.5834 23.0976 100.408 24.9235 100.408 27.1747C100.408 27.7008 100.308 28.2029 100.127 28.6646C100.148 28.488 100.16 28.3075 100.16 28.1251C100.16 25.6579 98.16 23.6573 95.6928 23.6573C94.9046 23.6573 94.1635 23.8618 93.5203 24.2208C94.2518 23.5238 95.2416 23.0966 96.3312 23.0966V23.0976ZM108.444 11.0333C112.063 11.0333 114.997 13.968 114.997 17.5872C114.997 21.2064 112.063 24.1411 108.444 24.1411C104.824 24.1411 101.89 21.2064 101.89 17.5872C101.89 13.968 104.824 11.0333 108.444 11.0333ZM108.444 11.9702C111.545 11.9702 114.06 14.4845 114.06 17.5872C114.06 18.311 113.923 19.0042 113.674 19.6397C113.703 19.3958 113.719 19.1482 113.719 18.8966C113.719 15.4973 110.964 12.7421 107.565 12.7421C106.478 12.7421 105.458 13.0243 104.572 13.5187C105.58 12.5597 106.943 11.9702 108.444 11.9702ZM89.8022 0C95.3338 0 99.8189 4.48416 99.8189 10.0166C99.8189 15.5482 95.3338 20.0323 89.8022 20.0323C84.2707 20.0323 79.7856 15.5482 79.7856 10.0166C79.7856 4.48416 84.2707 0 89.8022 0V0ZM89.8022 1.43232C94.5427 1.43232 98.3866 5.2752 98.3866 10.0166C98.3866 11.1235 98.1763 12.1814 97.7952 13.153C97.8403 12.7805 97.8634 12.4013 97.8634 12.0173C97.8634 6.82272 93.6528 2.61216 88.4592 2.61216C86.7984 2.61216 85.2394 3.0432 83.8848 3.79776C85.4246 2.33184 87.5088 1.43136 89.8022 1.43136V1.43232ZM14.2656 69.7478V73.9661H19.1126V69.7478H23.3309V64.9008H19.1126V60.6826H14.2656V64.9008H10.0474V69.7478H14.2656ZM15.9504 68.063H14.2656H11.7322V66.5856H14.2656H15.9504V64.9008V62.3674H17.4278V64.9008V66.5856H19.1126H21.6461V68.063H19.1126H17.4278V69.7478V72.2813H15.9504V69.7478V68.063ZM23.16 82.7098V86.1542L49.2346 101.208V63.5482L81.8486 44.7178L55.7741 29.664L23.159 48.4934V51.9379C29.161 54.4656 33.3754 60.4022 33.3754 67.3238C33.3754 74.2454 29.161 80.1821 23.159 82.7098H23.16ZM16.6886 55.1645C23.4048 55.1645 28.849 60.6086 28.849 67.3248C28.849 74.041 23.4048 79.4851 16.6886 79.4851C9.97248 79.4851 4.52832 74.041 4.52832 67.3248C4.52832 60.6086 9.97248 55.1645 16.6886 55.1645ZM62.3136 71.1005V101.209L88.3882 86.1552V56.0467L62.3136 71.1005Z";

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "Joystick Token".to_string(),
                symbol: "JSK".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(
        owner_id: AccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
        };
        this.token.internal_register_account(&owner_id);
        this.token.internal_deposit(&owner_id, total_supply.into());
        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &owner_id,
            amount: &total_supply,
            memo: Some("Initial tokens supply is minted"),
        }
        .emit();
        this
    }

    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, Balance};

    use super::*;

    const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
        assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(2).into(), TOTAL_SUPPLY.into());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());
        let transfer_amount = TOTAL_SUPPLY / 3;
        contract.ft_transfer(accounts(1), transfer_amount.into(), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert_eq!(contract.ft_balance_of(accounts(2)).0, (TOTAL_SUPPLY - transfer_amount));
        assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
    }
}