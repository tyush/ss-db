use actix_web::{
    get, web::{self, ServiceConfig}, HttpRequest, HttpResponse
};
use sea_orm::prelude::*;
use crate::{foreign_traits::ErrToActix as _, app::AppState};

use super::WebResult;

pub fn add_paths(
    conf: &mut ServiceConfig
) {
    conf.service(get_all_by_team_pit)
        .service(get_all_by_team_match)
        .service(get_all_pit)
        .service(get_all_match);
}

#[get("/api/2022/all/pit/{team}")]
async fn get_all_by_team_pit(
    _req: HttpRequest,
    path: web::Path<(u32,)>,
    data: web::Data<AppState>
) -> WebResult {
    use entities::pit_responses_2022 as pit_resp;

    let responses: Vec<pit_resp::Model> = pit_resp::Entity::find()
        .filter(pit_resp::Column::Team.eq(path.0))
        .all(&data.conn)
        .await?;

    Ok(HttpResponse::Ok().json(responses))
}

#[get("/api/2022/all/pit")]
async fn get_all_pit(
    _req: HttpRequest,
    data: web::Data<AppState>
) -> WebResult {
    use entities::pit_responses_2022 as pit_resp;

    let responses: Vec<pit_resp::Model> = pit_resp::Entity::find()
        .all(&data.conn)
        .await?;

    Ok(HttpResponse::Ok().json(responses))
}

#[get("/api/2022/all/match/{team}")]
async fn get_all_by_team_match(
    _req: HttpRequest,
    path: web::Path<(u32,)>,
    data: web::Data<AppState>
) -> WebResult {
    use entities::match_responses_2022 as match_resp;

    let responses: Vec<match_resp::Model> = match_resp::Entity::find()
        .filter(match_resp::Column::TeamNumber.eq(path.0))
        .all(&data.conn)
        .await?;

    Ok(HttpResponse::Ok().json(responses))
}

#[get("/api/2022/all/match")]
async fn get_all_match(
    _req: HttpRequest,
    data: web::Data<AppState>
) -> WebResult {
    use entities::match_responses_2022 as match_resp;

    let responses: Vec<match_resp::Model> = match_resp::Entity::find()
        .all(&data.conn)
        .await?;

    Ok(HttpResponse::Ok().json(responses))
}
