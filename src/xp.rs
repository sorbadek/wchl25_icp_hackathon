// ...existing code...
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static XP_BALANCES: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
}

#[ic_cdk::update]
pub fn mint_xp(user_id: String, amount: u64) {
    XP_BALANCES.with(|balances| {
        let mut binding = balances.borrow_mut();
        let entry = binding.entry(user_id).or_insert(0);
        *entry += amount;
    });
}

#[ic_cdk::update]
pub fn burn_xp(user_id: String, amount: u64) {
    XP_BALANCES.with(|balances| {
        let mut binding = balances.borrow_mut();
        let entry = binding.entry(user_id).or_insert(0);
        if *entry >= amount {
            *entry -= amount;
        }
    });
}

#[ic_cdk::query]
pub fn get_xp(user_id: String) -> u64 {
    XP_BALANCES.with(|balances| *balances.borrow().get(&user_id).unwrap_or(&0))
}
