use actix_web_flash_messages::FlashMessage;
use serde::Serialize;


#[derive(Serialize, Debug)]
pub struct Alert {
    variant: AlertType,
    text: String
}

impl From<&FlashMessage> for Alert {
    fn from(rhs: &FlashMessage) -> Self {
        Self::new(
            match rhs.level() {
                actix_web_flash_messages::Level::Debug | actix_web_flash_messages::Level::Info => AlertType::Info,
                actix_web_flash_messages::Level::Success => AlertType::Success,
                actix_web_flash_messages::Level::Warning => AlertType::Warning,
                actix_web_flash_messages::Level::Error => AlertType::Danger
            },
            rhs.content().to_owned(),
        )
    }
}

impl Alert {
    pub fn new(variant: AlertType, text: String) -> Self {
        Self {
            variant,
            text
        }
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(into = "&str")]
pub enum AlertType {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark
}

impl Into<&'static str> for AlertType {
    fn into(self) -> &'static str {
        match self {
            AlertType::Primary => "primary",
            AlertType::Secondary => "secondary",
            AlertType::Success => "success",
            AlertType::Danger => "danger",
            AlertType::Warning => "warning",
            AlertType::Info => "info",
            AlertType::Light => "light",
            AlertType::Dark => "dark",
        }
    }
}