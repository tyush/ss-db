
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    ss_db::main().await
}