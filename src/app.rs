use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
    pub templates: tera::Tera,
    pub conn: DatabaseConnection
}