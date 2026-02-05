use crate::models::course::{CreateCourse,UpdateCourse};
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use crate::db_access::course::*;
use crate::errors::MyError;

// 新建课程的处理逻辑
pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>
) -> Result<HttpResponse, MyError> {
    post_new_course_db(&app_state.db, new_course.try_into()?)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

/**
 * 获取某个老师课程的处理逻辑
*/
pub async fn get_courses_for_teacher (
    app_state: web::Data<AppState>,
    params: web::Path<(usize,)>
) -> Result<HttpResponse, MyError> {
    let teacher_id = i32::try_from(params.0).unwrap();
    get_courses_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|courses|HttpResponse::Ok().json(courses))
}

/**
 *  获取某个课程详情的处理逻辑
*/
pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>
) -> Result<HttpResponse, MyError> {
    let teacher_id = i32::try_from(params.0).unwrap();
    let course_id = i32::try_from(params.1).unwrap();
    get_all_courses_details_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course|HttpResponse::Ok().json(course))
}

/**
 * 更新课程详情的处理逻辑
*/
pub async fn update_course_details(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = path.into_inner();
    update_course_details_db(&app_state.db, teacher_id,course_id, update_course.into())
    .await
    .map(|course|HttpResponse::Ok().json(course))
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
)-> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = path.into_inner();
    delete_course_db(&app_state.db, teacher_id, course_id)
    .await
    .map_err(MyError::from)
    .map(|resp|HttpResponse::Ok().json(resp))
}   

#[cfg(test)]
mod tests { 

use dotenv::dotenv;
use sqlx::postgres::{PgPoolOptions};
use std::env;
use actix_web:: {http::StatusCode};
use std::sync::Mutex;
use crate::models::course::CreateCourse;
use crate::state::AppState;
use crate::handlers::course::*;


#[actix_rt::test]

async fn post_course_test() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();
    let app_state: web::Data<AppState> = web::Data::new(AppState {
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    let course = web::Json(
        CreateCourse {
            teacher_id: 1,
            name: "Test course".into(),
            description: Some("Test course description".into()),
            format: Some("Test course format".into()),
            structure: Some("Test course structure".into()),
            duration: Some("Test course duration".into()),
            price: Some(100),
            language: Some("Test course language".into()),
            level: Some("Test course level".into()),
        }
    );
    let resp = post_new_course(course, app_state).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn get_all_courses_success() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();
    let app_state: web::Data<AppState> = web::Data::new(AppState {
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    let teacher_id: web::Path<(usize,)> = web::Path::from((1,));
    let resp = get_courses_for_teacher(app_state, teacher_id).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn get_one_course_success() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();
    let app_state: web::Data<AppState> = web::Data::new(AppState {
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
    let resp = get_course_detail(app_state, params).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn get_one_course_failure() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();
    let app_state: web::Data<AppState> = web::Data::new(AppState {
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
    let resp = get_course_detail(app_state, params).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn update_course_success() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();
    let app_state: web::Data<AppState> = web::Data::new(AppState {
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    let update_course = UpdateCourse {
        name: Some("Test course".into()),
        description: Some("Test course description".into()),
        format: Some("Test course format".into()),
        structure: Some("Test course structure".into()),
        duration: Some("Test course duration".into()),
        price: Some(100),
        language: Some("Test course language".into()),
        level: Some("Test course level".into()),
    };
    let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
    let update_param = web::Json(update_course);
    let resp = update_course_details(app_state, update_param, params).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    }

#[actix_rt::test]
async fn delete_course_success() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();
    let app_state: web::Data<AppState> = web::Data::new(AppState {
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    let params: web::Path<(i32, i32)> = web::Path::from((1, 3));
    let resp = delete_course(app_state, params).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}
}

