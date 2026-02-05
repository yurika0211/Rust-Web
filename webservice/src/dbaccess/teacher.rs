use crate::{ models::teacher::{Teacher, CreateTeacher, UpdateTeacher}};

use sqlx::postgres::PgPool;
use crate::errors::MyError;
use actix_web::{web, HttpResponse};
pub async fn get_all_teacher_db(
    pool: &PgPool,
) -> Result<Vec<Teacher>, MyError> {
    let row = sqlx::query_as!(
        Teacher,
        r#"select *
        from teacher"#
    )
    .fetch_all(pool)
    .await?;

    Ok(row)
}

pub async fn get_teacher_details_db(
    pool: &PgPool,
    id: i32
) -> Result<Teacher, MyError> {
    let row = sqlx::query_as!(
        Teacher,
        r#"select *
        from teacher
        where id = $1"#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn post_new_teacher_db(
    pool: &PgPool,
    new_teacher: CreateTeacher
) -> Result<Teacher, MyError> {
    let row: Teacher = sqlx::query_as!(
        Teacher,
        r#"insert into teacher (name, picture_url, profile)
        values ($1, $2, $3)
        returning id, name, picture_url, profile"#,
        new_teacher.name,
        new_teacher.picture_url,
        new_teacher.profile
    )
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn update_teacher_details_db(
    id: i32,
    pool: &PgPool,
    update_teacher: UpdateTeacher,
) -> Result<Teacher, MyError> {
    let row = sqlx::query_as!(
        Teacher,
        "select * from teacher
        where id = $1",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err|MyError::NotFound("Teacher Id not found".into()))?;

    let name: String = if let Some(name) = update_teacher.name {
        name
    } else {
        row.name.unwrap_or_default()
    };
    let picture_url: String = if let Some(picture_url) = update_teacher.picture_url {
        picture_url
    } else {
        row.picture_url.unwrap_or_default()
    };
    let profile: String = if let Some(profile) = update_teacher.profile {
        profile
    } else {
        row.profile.unwrap_or_default()
    };
    let row = sqlx::query_as!(
        Teacher,
        "update teacher
        set name = $1, picture_url = $2, profile = $3
        where id = $4
        returning id, name, picture_url, profile",
        name,
        picture_url,
        profile,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn delete_teacher_db(
    pool: &PgPool,
    id: i32,
)-> Result<String, sqlx::Error> {
    let row = sqlx::query!(
        "delete from teacher where id = $1",
        id,
    )
    .execute(pool)
    .await?;

    Ok(format!("Deleted {:?} record", row))
}

