use actix_web::web;
use serde::{Deserialize, Serialize};

use crate::errors::MyError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Teacher {
    pub id: i32,
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateTeacher {
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateTeacher {
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

impl From<web::Json<Teacher>> for Teacher {
    fn from(teacher: web::Json<Teacher>) -> Self {
        Teacher {
            id: teacher.id,
            name: teacher.name.clone(),
            picture_url: teacher.picture_url.clone(),
            profile: teacher.profile.clone(),
        }
    }
}

impl TryFrom<web:: Json<CreateTeacher>> for CreateTeacher {
    type Error = MyError;
    
    fn try_from(value: web:: Json<CreateTeacher>) -> Result<Self, Self::Error> {
        Ok(CreateTeacher {
            name: value.name.clone(),
            picture_url: value.picture_url.clone(),
            profile: value.profile.clone(),
        })
    }
}

impl From<web::Json<UpdateTeacher>> for UpdateTeacher {

    fn from(value: web::Json<UpdateTeacher>) -> Self {
        UpdateTeacher {
            name: value.name.clone(),
            picture_url: value.picture_url.clone(),
            profile: value.profile.clone(),
        }
    }
}