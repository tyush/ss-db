use actix_web::{
    get, web::{self, ServiceConfig}, HttpRequest, HttpResponse
};
use anyhow::Context;
use sea_orm::prelude::*;
use crate::foreign_traits::*;
use rand::seq::SliceRandom;

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

pub fn add_paths(
    conf: &mut ServiceConfig
) {
    conf.service(index)
        .route("/form/2022/pit",  web::to(forms2022::pit))
        .service(forms2022::pit_submit);
        
}


lazy_static::lazy_static! {
    static ref COMMENTS: Vec<&'static str> = vec![
        "Great hats!",
        "they're bot is too purple",
        "lmao shaft collars",
        "lmao swerve",
        "their bot is angry at me",
        "fiesty auton",
        "i didn't know balls could legally go that fast",
        "their shooter embodies \"brrrrr\"",
        "their button board plays tetris!",
        "Atmosphere was great, but service was lacking.",
        "their robot runs rust!",
        "refused to tell me their bot's k/d",
        "they told me their robot's k/d before their team number",
        "their bot makes claptrap sounds!",
        "one guy kinda looked like amogus imposter",
        "9/10 needs more metal",
        "9/10 needs less metal",
        "10/10 perfect amount of metal",
        "coffe was excellent",
        "bot unusable as skateboard",
        "robot had weird yellow lights coming off of it when it moved",
        "driver went to go get a snack",
        "i swear it looked at me",
        "this coffee is excellent",
        "the thing that rotates and touches the floor broke",
        "robot was too loud",
        "when home",
        "i'm giving them a bad yelp review",
        "their bot makes funny noises when it bumps into stuff",
        "is smoke normal?",
        "their driver was playing minecraft",
        "Great scarfs!",
        "their robot is too fast",
        "what even is oatmeal",
        "bot too usable as skateboard",
        "i thought animal parts were illegal",
        "i swear i saw a person in there",
        "can i go home now?",
        "red2 was sus",
        "yo bots can move",
        "i can't believe it's not labview!"
    ];
}

pub fn get_random_comment() -> &'static str {
    COMMENTS.choose(&mut rand::thread_rng()).unwrap()
}