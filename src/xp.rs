use candid::{CandidType, Deserialize};
use ic_cdk::storage;
use std::collections::HashMap;

thread_local! {
    static XP_BALANCES: storage::StableCell<HashMap<String, u64>> = storage::StableCell::new(HashMap::new());
}

#[ic_cdk::update]
pub fn mint_xp(user_id: String, amount: u64) {
    XP_BALANCES.with(|balances| {
        let entry = balances.borrow_mut().entry(user_id).or_insert(0);
        *entry += amount;
    });
}

#[ic_cdk::update]
pub fn burn_xp(user_id: String, amount: u64) {
    XP_BALANCES.with(|balances| {
        let entry = balances.borrow_mut().entry(user_id).or_insert(0);
        if *entry >= amount {
            *entry -= amount;
        }
    });
}

#[ic_cdk::query]
pub fn get_xp(user_id: String) -> u64 {
    XP_BALANCES.with(|balances| *balances.borrow().get(&user_id).unwrap_or(&0))
}
