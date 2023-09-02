use server_jukebox::ServerJukebox;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = ServerJukebox::new();
    return app.run().await;
}
