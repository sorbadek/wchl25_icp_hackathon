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

thread_local! {
    static COURSES: RefCell<HashMap<String, Course>> = RefCell::new(HashMap::new());
    static RESOURCES: RefCell<HashMap<String, Resource>> = RefCell::new(HashMap::new());
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Course {
    pub id: String,
    pub title: String,
    pub description: String,
    pub creator: String,
    pub enrolled: Vec<String>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Resource {
    pub id: String,
    pub title: String,
    pub description: String,
    pub owner: String,
    pub price: u64,
}

#[ic_cdk::update]
pub fn create_course(id: String, title: String, description: String, creator: String) {
    COURSES.with(|courses| {
        courses.borrow_mut().insert(id.clone(), Course {
            id,
            title,
            description,
            creator,
            enrolled: vec![],
        });
    });
}

#[ic_cdk::update]
pub fn enroll_course(course_id: String, user_id: String) {
    COURSES.with(|courses| {
        if let Some(course) = courses.borrow_mut().get_mut(&course_id) {
            if !course.enrolled.contains(&user_id) {
                course.enrolled.push(user_id);
            }
        }
    });
}

#[ic_cdk::query]
pub fn get_course(course_id: String) -> Option<Course> {
    COURSES.with(|courses| courses.borrow().get(&course_id).cloned())
}

#[ic_cdk::update]
pub fn create_resource(id: String, title: String, description: String, owner: String, price: u64) {
    RESOURCES.with(|resources| {
        resources.borrow_mut().insert(id.clone(), Resource {
            id,
            title,
            description,
            owner,
            price,
        });
    });
}

#[ic_cdk::query]
pub fn get_resource(resource_id: String) -> Option<Resource> {
    RESOURCES.with(|resources| resources.borrow().get(&resource_id).cloned())
}
