
use crate::{ models::teacher::{Teacher, CreateTeacher, UpdateTeacher}};
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use crate::db_access::teacher::{delete_teacher_db, post_new_teacher_db, get_teacher_details_db, get_all_teacher_db, update_teacher_details_db};
use crate::errors::MyError;


pub async fn get_all_teachers(
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    get_all_teacher_db(&app_state.db)
    .await
    .map(|teachers| HttpResponse::Ok().json(teachers))
}

pub async fn get_teacher_detail(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let id = i32::try_from(path.into_inner()).unwrap();
    get_teacher_details_db(&app_state.db, id)
    .await
    .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn post_new_teacher(
    new_teacher: web::Json<CreateTeacher>,
    app_state: web::Data<AppState>
) -> Result<HttpResponse, MyError> {
    post_new_teacher_db(&app_state.db, new_teacher.try_into()?)
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn delete_teacher(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let id = path.into_inner();
    delete_teacher_db(&app_state.db, id)
    .await
    .map_err(MyError::from)
    .map(|resp|HttpResponse::Ok().json(resp))
}

pub async fn update_teacher_details(
    app_state: web::Data<AppState>,
    update_teacher: web::Json<UpdateTeacher>,
    path: web::Path<i32>,) -> Result<HttpResponse, MyError> {
    let id = path.into_inner();
    update_teacher_details_db(id, &app_state.db, update_teacher.into())
    .await
    .map(|teacher|HttpResponse::Ok().json(teacher))

}


#[cfg(test)]
mod tests {

use dotenv::dotenv;
use actix_web:: {http::StatusCode};
use std::env;
use std::sync::Mutex;
use sqlx::postgres::PgPoolOptions;
use super::*;
use crate::models::teacher::{UpdateTeacher};

#[actix_rt::test]
async fn post_course_test() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();
    let app_state: web::Data<AppState> = web::Data::new( 
        AppState {
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
        }
    );
    let teacher = web::Json(
        CreateTeacher {
            name: "test".to_string(),
            picture_url: "test".to_string(),
            profile: "test".to_string(),
        }
    );
    let resp = post_new_teacher(teacher, app_state).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn delete_course_success () {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();
    let app_state: web::Data<AppState> = web::Data::new( 
        AppState {
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
        }
    );
    let params:web::Path<i32> = web::Path::from(1);
    let resp = delete_teacher(app_state, params).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn get_one_teacher_success() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();
    let app_state: web::Data<AppState> = web::Data::new( 
        AppState {
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
        }
    );
    let params:web::Path<i32> = web::Path::from(2);
    let resp = get_teacher_detail(app_state, params).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn get_all_teacher_success() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();
    let app_state: web::Data<AppState> = web::Data::new( 
        AppState {
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
        }
    );
    let resp = get_all_teachers(app_state).await.unwrap();
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
    let app_state: web::Data<AppState> = web::Data::new( 
        AppState {
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
        }
    );
    let update_teacher = UpdateTeacher {
        name: Some("Test course".into()),
        picture_url: Some("Test course description".into()),
        profile: Some("Test course format".into()),
    };
    let params:web::Path<i32> = web::Path::from(2);
    let update_param = web::Json(update_teacher);
    let resp = update_teacher_details(app_state, update_param, params).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}


}