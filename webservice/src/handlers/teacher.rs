
use crate::{ models::teacher::{Teacher, CreateTeacher, UpdateTeacher}};
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use crate::db_access::teacher::{delete_teacher_db, post_new_teacher_db};
use crate::errors::MyError;


pub async fn get_all_teachers() {
    unimplemented!()
}

pub async fn get_teacher_detail() {
    unimplemented!()
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

pub async fn update_teacher_details() {
    unimplemented!()
}


#[cfg(test)]
mod tests {

use dotenv::dotenv;
use actix_web:: {http::StatusCode};
use std::env;
use std::sync::Mutex;
use sqlx::postgres::PgPoolOptions;
use super::*;

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





}