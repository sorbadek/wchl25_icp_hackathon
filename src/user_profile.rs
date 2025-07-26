use candid::{CandidType, Deserialize};
use ic_cdk::storage;
use std::collections::HashMap;
use std::cell::RefCell;

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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ExtendedUserProfile {
    pub name: String,
    pub bio: String,
    pub skills: Vec<String>,
    pub certificates: Vec<String>,
}

thread_local! {
    static EXTENDED_USER_PROFILES: RefCell<HashMap<String, ExtendedUserProfile>> = RefCell::new(HashMap::new());
}

#[ic_cdk::update]
pub fn register_user(user_id: String, name: String, bio: String) {
    EXTENDED_USER_PROFILES.with(|profiles| {
        profiles.borrow_mut().insert(user_id, ExtendedUserProfile {
            name,
            bio,
            skills: vec![],
            certificates: vec![],
        });
    });
}

#[ic_cdk::update]
pub fn add_skill(user_id: String, skill: String) {
    EXTENDED_USER_PROFILES.with(|profiles| {
        if let Some(profile) = profiles.borrow_mut().get_mut(&user_id) {
            profile.skills.push(skill);
        }
    });
}

#[ic_cdk::update]
pub fn add_certificate(user_id: String, cert_id: String) {
    EXTENDED_USER_PROFILES.with(|profiles| {
        if let Some(profile) = profiles.borrow_mut().get_mut(&user_id) {
            profile.certificates.push(cert_id);
        }
    });
}

#[ic_cdk::query]
pub fn get_user_profile(user_id: String) -> Option<ExtendedUserProfile> {
    EXTENDED_USER_PROFILES.with(|profiles| profiles.borrow().get(&user_id).cloned())
}
