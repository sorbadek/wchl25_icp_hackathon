use candid::{CandidType, Deserialize};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
pub struct Credential {
    pub credential_id: u64,
    pub user_id: String,
    pub session_id: u64,
    pub timestamp: u64,
}

thread_local! {
    static CREDENTIALS: RefCell<HashMap<u64, Credential>> = RefCell::new(HashMap::new());
    static NEXT_CREDENTIAL_ID: RefCell<u64> = RefCell::new(0);
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

#[derive(CandidType, Deserialize, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Certificate {
    pub id: String,
    pub user_id: String,
    pub course_id: String,
    pub issued_at: u64,
}

thread_local! {
    static CERTIFICATES: RefCell<HashMap<String, Certificate>> = RefCell::new(HashMap::new());
}

#[ic_cdk::update]
pub fn issue_certificate(id: String, user_id: String, course_id: String, issued_at: u64) {
    CERTIFICATES.with(|certs| {
        certs.borrow_mut().insert(id.clone(), Certificate {
            id,
            user_id,
            course_id,
            issued_at,
        });
    });
}

#[ic_cdk::query]
pub fn get_certificate(cert_id: String) -> Option<Certificate> {
    CERTIFICATES.with(|certs| certs.borrow().get(&cert_id).cloned())
}
