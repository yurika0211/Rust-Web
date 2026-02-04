use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use crate::errors::MyError;

// use create::models::course::Course;
#[derive(Serialize, Debug, Clone)]
pub struct Course {
    pub teacher_id: i32,
    pub id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,

    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
    pub teacher_id: i32,
    pub name: String,

    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateCourse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl From<web::Json<Course>> for Course {
    // From trait：实现 From trait，将 web::Json<T> 转换为 T
    fn from(course: web::Json<Course>) -> Self {
        Course {
            teacher_id: course.teacher_id,
            id: course.id,
            name: course.name.clone(),
            time: course.time,
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price,
            language: course.language.clone(),
            level: course.level.clone(),
            description: course.description.clone(),
            format: course.format.clone(),
        }
    }
}


impl TryFrom<web:: Json<CreateCourse>> for CreateCourse {
    type Error = MyError;
    // TryFrom trait：实现 TryFrom trait，将 web::Json<T> 尝试转换为 T
    fn try_from(course: web::Json<CreateCourse>) -> Result<Self, Self::Error> {
        Ok(CreateCourse {
            teacher_id: course.teacher_id,
            name: course.name.clone(),
            language: course.language.clone(),
            level: course.level.clone(),
            description: course.description.clone(),
            format: course.format.clone(),
            duration: course.duration.clone(),
            price: course.price.clone(),
            structure: course.structure.clone(),
        })
    }
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {

    // From trait：实现 From trait，将 web::Json<T> 转换为 T
    fn from(value: web::Json<UpdateCourse>) -> Self {
        UpdateCourse {
            name: value.name.clone(),
            language: value.language.clone(),
            level: value.level.clone(),
            description: value.description.clone(),
            format: value.format.clone(),
            duration: value.duration.clone(),
            price: value.price.clone(),
            structure: value.structure.clone(),
        }
    }
}