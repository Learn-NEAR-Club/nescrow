use near_sdk::json_types::U128;
use near_sdk::store::IterableMap;
use near_sdk::{env, log, near, AccountId, NearToken, Promise};
use crate::contract::USER_REGISTRATION_STORAGE_USAGE_DEPOSIT;
use crate::enums::storage_keys::StorageKeys;
use super::{NescrowExt, Nescrow};

#[near]
#[allow(dead_code)]
impl Nescrow {
    #[payable]
    pub fn register_customer(&mut self, username: String, account_id: AccountId) {
        if String::is_empty(&username) {
            panic!("Username should be provided");
        }

        if self.deposits.contains_key(&username) {
            panic!("Username already register");
        }

        let username_hash = env::sha256_array(&username.as_bytes());

        let mut account_balance_map = IterableMap::new(StorageKeys::AccountBalance { username_hash });
        account_balance_map.insert(account_id, U128(0));

        self.deposits.insert(username, account_balance_map);

        let attached_deposit = env::attached_deposit();

        assert!(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT <= attached_deposit.as_yoctonear(),
            "Attached deposit should be >= {} for registering on blockchain",
            NearToken::from_yoctonear(USER_REGISTRATION_STORAGE_USAGE_DEPOSIT)
        );

        let refund = attached_deposit.as_yoctonear() - USER_REGISTRATION_STORAGE_USAGE_DEPOSIT;

        log!("Deposit to return {}", refund);

        if refund > 0 {
            Promise::new(env::predecessor_account_id()).transfer(NearToken::from_yoctonear(refund));
        }
    }

    pub fn is_registered(self, sender_username: String) -> bool {
        let is_username_registered = self.deposits.contains_key(&sender_username);

        return is_username_registered;
    }
}
