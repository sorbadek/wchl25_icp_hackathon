use candid::{CandidType, Deserialize};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
pub struct Content {
    pub content_id: u64,
    pub creator: String,
    pub title: String,
    pub ipfs_link: String,
    pub required_xp: u64,
    pub created_at: u64,
}

thread_local! {
    static CONTENTS: RefCell<HashMap<u64, Content>> = RefCell::new(HashMap::new());
    static NEXT_CONTENT_ID: RefCell<u64> = RefCell::new(0);
}

#[ic_cdk::update]
pub fn upload_content(creator: String, title: String, ipfs_link: String, required_xp: u64) -> u64 {
    let content_id = NEXT_CONTENT_ID.with(|id| {
        let mut id = id.borrow_mut();
        let current = *id;
        *id += 1;
        current
    });
    let content = Content {
        content_id,
        creator,
        title,
        ipfs_link,
        required_xp,
        created_at: ic_cdk::api::time(),
    };
    CONTENTS.with(|contents| {
        contents.borrow_mut().insert(content_id, content);
    });
    content_id
}
