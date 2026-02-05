

use crate::{ models::teacher::{Teacher, CreateTeacher, UpdateTeacher}};

use sqlx::postgres::PgPool;
use crate::errors::MyError;

pub async fn get_all_teacher_db() {
    unimplemented!()
}

pub async fn get_teacher_details_db() {
    unimplemented!()
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

pub async fn update_teacher_details_db() {
    unimplemented!()
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

