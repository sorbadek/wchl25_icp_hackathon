use candid::{CandidType, Deserialize};
use ic_cdk::storage;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
pub struct Session {
    pub session_id: u64,
    pub owner: String,
    pub title: String,
    pub description: String,
    pub required_xp: u64,
    pub created_at: u64,
}

thread_local! {
    static SESSIONS: storage::StableCell<HashMap<u64, Session>> = storage::StableCell::new(HashMap::new());
    static NEXT_SESSION_ID: storage::StableCell<u64> = storage::StableCell::new(0);
}

#[ic_cdk::update]
pub fn create_session(owner: String, title: String, description: String, required_xp: u64) -> u64 {
    let session_id = NEXT_SESSION_ID.with(|id| {
        let mut id = id.borrow_mut();
        let current = *id;
        *id += 1;
        current
    });
    let session = Session {
        session_id,
        owner,
        title,
        description,
        required_xp,
        created_at: ic_cdk::api::time(),
    };
    SESSIONS.with(|sessions| {
        sessions.borrow_mut().insert(session_id, session);
    });
    session_id
}
