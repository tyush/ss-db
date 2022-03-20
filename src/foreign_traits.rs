use std::fmt::Display;



/// Allow usage of ? in Actix services (impl Responder)
/// Transparent in console.
#[derive(Debug)]
pub enum ErrToActix {
    Anyhow(anyhow::Error),
    SeaORM(sea_orm::DbErr),
    Tera(tera::Error),
    SerdeJSON(serde_json::error::Error)
}

impl Display for ErrToActix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Anyhow(e) => write!(f, "{}", e),
            Self::SeaORM(e) => write!(f, "{}", e),
            Self::Tera(e) => write!(f, "{}", e),
            Self::SerdeJSON(e) => write!(f, "{}", e)
        }
    }
}

impl actix_web::error::ResponseError for ErrToActix {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

impl From<anyhow::Error> for ErrToActix {
    fn from(rhs: anyhow::Error) -> Self {
        Self::Anyhow(rhs)
    }
}

impl From<sea_orm::DbErr> for ErrToActix {
    fn from(rhs: sea_orm::DbErr) -> Self {
        Self::SeaORM(rhs)
    }
}

impl From<tera::Error> for ErrToActix {
    fn from(rhs: tera::Error) -> Self {
        Self::Tera(rhs)
    }
}

impl From<serde_json::Error> for ErrToActix {
    fn from(rhs: serde_json::Error) -> Self {
        Self::SerdeJSON(rhs)
    }
}