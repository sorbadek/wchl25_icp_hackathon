use candid::{CandidType, Deserialize};
use ic_cdk::storage;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
pub struct Credential {
    pub credential_id: u64,
    pub user_id: String,
    pub session_id: u64,
    pub timestamp: u64,
}

thread_local! {
    static CREDENTIALS: storage::StableCell<HashMap<u64, Credential>> = storage::StableCell::new(HashMap::new());
    static NEXT_CREDENTIAL_ID: storage::StableCell<u64> = storage::StableCell::new(0);
}

#[ic_cdk::update]
pub fn mint_credential(user_id: String, session_id: u64) -> u64 {
    let credential_id = NEXT_CREDENTIAL_ID.with(|id| {
        let mut id = id.borrow_mut();
        let current = *id;
        *id += 1;
        current
    });
    let credential = Credential {
        credential_id,
        user_id,
        session_id,
        timestamp: ic_cdk::api::time(),
    };
    CREDENTIALS.with(|creds| {
        creds.borrow_mut().insert(credential_id, credential);
    });
    credential_id
}
