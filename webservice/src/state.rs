use std::sync::Mutex;
use super::models::Course;
use sqlx::PgPool;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<i32>,
    #[allow(dead_code)]
    // pub courses: Mutex<Vec<Course>>,
    pub db: PgPool,
}

