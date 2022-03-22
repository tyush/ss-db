use actix_web_flash_messages::Level;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub async fn main() -> std::io::Result<()> {
    // use migration::Migrator;
    use tera::Tera;
    use crate::app::AppState;
    use actix_web::{ App, HttpServer, web::Data };
    use actix_files::Files;
    use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "80".to_string());
    let server_url = format!("{}:{}", host, port);

    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    // Migrator::up(&conn, None).await.unwrap();
    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
    let state = AppState { templates, conn };

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("../key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("../cert.pem").unwrap();


    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let secret_key = actix_web::cookie::Key::generate();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web_flash_messages::FlashMessagesFramework::builder(
                actix_web_flash_messages::storage::CookieMessageStore::builder(secret_key.clone()).build()
            ).minimum_level(Level::Debug).build())
            .wrap(actix_session::SessionMiddleware::new(
                actix_session::storage::CookieSessionStore::default(), secret_key.clone()
            ))
            .configure(paths::add_paths)
            .service(Files::new("/static", "./static")) // needs actix_files
    });

    let server = server.bind_openssl(&server_url, builder)?;
    println!("Hosting on {}", server_url);

    server.run().await?;

    Ok(())
}


pub mod foreign_traits;
pub(crate) mod paths;
pub(crate) mod app;
pub(crate) mod tera_util;
pub(crate) mod game;
