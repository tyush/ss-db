use actix_web::{
    get, post, web, HttpRequest, HttpResponse, http::header
};
use actix_web_flash_messages::{
    FlashMessage, IncomingFlashMessages,
};
use actix_session::Session;
use anyhow::Context;
use entities::pit_responses_2022;
use log::debug;
use sea_orm::{Set, ActiveModelTrait};
use serde::{Serialize, Deserialize};
use crate::game::{Drivetrain, Bar};
use crate::tera_util::{Alert, AlertType};
use crate::{app::AppState, paths::WebResult};
use crate::foreign_traits::ErrToActix;


#[derive(Debug, Serialize)]
struct TemplatePit2022 {
    username: String,
    pass_hash: String,
}

pub(crate) async fn pit(
    req: HttpRequest,
    data: web::Data<AppState>,
    messages: IncomingFlashMessages,
    session: Session
) -> WebResult {
    let template = &data.templates;
    
    let mut context = tera::Context::new();

    context.insert("random_comment", crate::paths::get_random_comment());

    let flashes: Vec<Alert> = messages.iter().map(Alert::from).collect();

    debug!("flash messages for user at {:?}: {:?}", req.connection_info().peer_addr(), flashes);

    context.insert("messages", &flashes);

    Ok(HttpResponse::Ok().content_type("text/html").body(template.render("forms/pit2022.tera", &context)?))
}

#[derive(Deserialize, Debug)]

struct PitForm {
    pub team_number: u64,
    pub team_name: String,
    pub drivetrain: Drivetrain,
    pub size_x: f32,
    pub size_y: f32,
    pub size_z: f32,
    pub auto_shoot_upper: Option<()>,
    pub auto_shoot_lower: Option<()>,
    pub teleop_shoot_upper: Option<()>,
    pub teleop_shoot_lower: Option<()>,
    pub bar: Bar,
    pub comments: Option<String>,
    pub img: String
}

#[post("/form/2022/pit/submit")]
async fn pit_submit(
    req: HttpRequest,
    data: web::Data<AppState>,
    session: Session,
    form: web::Form<PitForm>
) -> WebResult {
    use std::time::{SystemTime, UNIX_EPOCH};
    // todo: ingest form
    debug!("Got form data from {:?}: {:?}", req.connection_info().peer_addr(), form.0);

    let entry = pit_responses_2022::ActiveModel {
        author: Set(session.get("uuid")?.unwrap_or(0)),
        timestamp: Set(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().try_into().unwrap()),
        team: Set(form.0.team_number.try_into().unwrap()),
        name: Set(Some(form.0.team_name)),
        drivetrain: Set(form.0.drivetrain.into()),
        size_x: Set(form.0.size_x.floor() as i32),
        size_y: Set(form.0.size_y.floor() as i32),
        size_z: Set(form.0.size_z.floor() as i32),
        climb: Set(form.0.bar.into()),
        auto_where_shoot: Set({ 
            let mut x = 0; // ripoff bitflag
            if form.0.auto_shoot_lower.is_some() {
                x += 1;
            }
            if form.0.auto_shoot_upper.is_some() {
                x += 2;
            }
            x
         }),
        auto_shots: Set(0), // todo: add to form
        auto_taxi: Set(0), // todo: add to form
        teleop_where_shoot: Set({ 
            let mut x = 0; // ripoff bitflag
            if form.0.teleop_shoot_lower.is_some() {
                x += 1;
            }
            if form.0.teleop_shoot_upper.is_some() {
                x += 2;
            }
            x
         }),
        image: Set(None),
        ..Default::default()
    };

    if let Err(e) = entry.insert(&data.conn).await {
        FlashMessage::error(format!("Unable to submit pit data for team {}: {:?}", form.0.team_number, e));
    } else {
        FlashMessage::success(format!("Successfully submitted pit data for team {}", form.0.team_number)).send();
    };

    Ok(
        HttpResponse::TemporaryRedirect()
        .insert_header((header::LOCATION, "/form/2022/pit"))
        .finish()
    )
}