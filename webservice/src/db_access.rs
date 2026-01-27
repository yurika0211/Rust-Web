use super::models::*;
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_teacher_db(pool: &PgPool, teacher_id: i32)-> Vec<Course> {
    let rows = sqlx::query!(
        r#"SELECT id, teacher_id, name, time FROM course WHERE teacher_id = $1"#,
        teacher_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    rows.iter()
        .map(
            |r| Course{
                id: Some(r.id),
                teacher_id: r.teacher_id,
                name: r.name.clone(),
                time: r.time.map(|t| NaiveDateTime::from(t)),
            }
        ).collect::<Vec<Course>>()
}

pub async fn get_all_courses_details_db(pool: &PgPool, teacher_id: i32, course_id :i32) -> Course {
    let row = sqlx::query!(
        r#"SELECT id, teacher_id, name, time FROM course where teacher_id = $1 and id = $2"#,
        teacher_id,
        course_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    Course{
        id: Some(row.id),
        teacher_id: row.teacher_id,
        name: row.name,
        time: Some(NaiveDateTime::from(row.time.unwrap())),
    }
    
}

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Course { 
    let row = sqlx::query!(
        r#"insert into course (id, teacher_id, name) values ($1, $2, $3) returning id, teacher_id, name"#,
            new_course.id,
            new_course.teacher_id,
            new_course.name,
    )
    .fetch_one(pool)
    .await
    .unwrap();
    
    Course {
        id: Some(row.id),
        teacher_id: row.teacher_id,
        name: row.name.clone(),
        time: None,
    }
}

