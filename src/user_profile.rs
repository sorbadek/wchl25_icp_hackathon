use candid::{CandidType, Deserialize};
use ic_cdk::storage;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
pub struct UserProfile {
    pub user_id: String,
    pub join_time: u64,
    pub role: String, // "Learner" by default
    pub is_verified: bool,
    pub last_login: u64,
}

thread_local! {
    static USER_PROFILES: storage::StableCell<HashMap<String, UserProfile>> = storage::StableCell::new(HashMap::new());
}

#[ic_cdk::update]
pub fn create_user_profile(user_id: String) {
    let profile = UserProfile {
        user_id: user_id.clone(),
        join_time: ic_cdk::api::time(),
        role: "Learner".to_string(),
        is_verified: false,
        last_login: ic_cdk::api::time(),
    };
    USER_PROFILES.with(|profiles| {
        profiles.borrow_mut().insert(user_id, profile);
    });
}
