use actix_web::{
    get, web, HttpRequest, HttpResponse
};
use actix_session::Session;
use anyhow::Context;
use sea_orm::prelude::*;
use serde::Serialize;
use crate::{foreign_traits::*, app::AppState, paths::WebResult};


#[derive(Debug, Serialize)]
struct TemplatePit2022 {
    username: String,
    pass_hash: String,
}

#[get("/form/2022/pit")]
async fn pit(
    _req: HttpRequest,
    data: web::Data<AppState>,
    session: Session
) -> WebResult {
    let template = &data.templates;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(template.render("forms/pit2022.tera", &tera::Context::new())?))
}