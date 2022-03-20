use actix_web::{
    get, web, HttpRequest, HttpResponse
};
use actix_session::Session;
use anyhow::Context;
use sea_orm::prelude::*;
use crate::foreign_traits::*;

pub(crate) type WebResult = Result<HttpResponse, ErrToActix>;
use entities::images::Entity as Image;

use crate::app::AppState;


pub mod forms2022;


#[derive(serde::Serialize)]
struct ImagePage {
    img_data: String
}

#[get("/")]
async fn index(
    _req: HttpRequest,
    data: web::Data<AppState>
) -> WebResult {
    let template = &data.templates;
    let conn = &data.conn;

    let image = Image::find_by_id(1).one(conn).await?.with_context(|| "Could not find image!")?;

    let html = template.render("dbtest_index.tera", &tera::Context::from_serialize(ImagePage { img_data: image.b64 })?)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}