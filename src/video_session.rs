use candid::{CandidType, Deserialize};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
pub struct VideoSession {
    pub session_id: u64,
    pub host: String,
    pub title: String,
    pub description: String,
    pub participants: Vec<String>,
    pub started_at: u64,
    pub ended_at: Option<u64>,
    pub xp_reward: u64,
}

thread_local! {
    static VIDEO_SESSIONS: RefCell<HashMap<u64, VideoSession>> = RefCell::new(HashMap::new());
    static NEXT_VIDEO_SESSION_ID: RefCell<u64> = RefCell::new(0);
}

#[ic_cdk::update]
pub fn create_video_session(host: String, title: String, description: String, xp_reward: u64) -> u64 {
    let session_id = NEXT_VIDEO_SESSION_ID.with(|id| {
        let mut id = id.borrow_mut();
        let current = *id;
        *id += 1;
        current
    });
    let session = VideoSession {
        session_id,
        host: host.clone(),
        title,
        description,
        participants: vec![],
        started_at: ic_cdk::api::time(),
        ended_at: None,
        xp_reward,
    };
    VIDEO_SESSIONS.with(|sessions| {
        sessions.borrow_mut().insert(session_id, session);
    });
    session_id
}

#[ic_cdk::update]
pub fn join_video_session(session_id: u64, user_id: String) {
    VIDEO_SESSIONS.with(|sessions| {
        if let Some(session) = sessions.borrow_mut().get_mut(&session_id) {
            if !session.participants.contains(&user_id) {
                session.participants.push(user_id);
            }
        }
    });
}

#[ic_cdk::update]
pub fn end_video_session(session_id: u64) {
    VIDEO_SESSIONS.with(|sessions| {
        if let Some(session) = sessions.borrow_mut().get_mut(&session_id) {
            session.ended_at = Some(ic_cdk::api::time());
        }
    });
}

#[ic_cdk::update]
pub fn reward_xp_for_video_session(session_id: u64) {
    use crate::xp::mint_xp;
    VIDEO_SESSIONS.with(|sessions| {
        if let Some(session) = sessions.borrow().get(&session_id) {
            // Reward host
            mint_xp(session.host.clone(), session.xp_reward);
            // Reward participants
            for user in &session.participants {
                mint_xp(user.clone(), session.xp_reward / 2); // Example: half XP for participants
            }
        }
    });
}

#[ic_cdk::query]
pub fn get_video_session(session_id: u64) -> Option<VideoSession> {
    VIDEO_SESSIONS.with(|sessions| sessions.borrow().get(&session_id).cloned())
}
