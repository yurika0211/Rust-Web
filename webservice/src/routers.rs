use actix_web::web;
use crate::handlers::course::*;
use crate::handlers::general::*;

// 健康检查的路由配置
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .route("/health", web::get()
    .to(health_check_handler));
}

// 课程的路由配置
pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(web::scope("/courses")
    .route("/", web::post().to(new_course))
    .route("/{teacher_id}", web::get().to(get_courses_for_teacher))
    .route("/{teacher_id}/{course_id}", web::get().to(get_course_detail))
    .route("/{teacher_id}/{course_id}", web::put().to(update_course_details))
    .route("/{teacher_id}/{course_id}", web::delete().to(delete_course))
);
}

