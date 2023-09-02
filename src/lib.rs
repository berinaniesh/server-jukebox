mod appstate;
mod routes;
mod models;
mod player;
mod downloader;
mod api;

use actix_web::{web, App, HttpServer};
use actix_files;
use actix_web::middleware::Logger;
use appstate::get_appstate;
use appstate::DB;
use log::info;

pub struct ServerJukebox;

impl ServerJukebox {
    pub fn new() -> Self {
        return ServerJukebox{};
    }
    pub async fn run(&self) -> std::io::Result<()> {
        let bind_ip: String = dotenvy::var("BIND").unwrap_or(String::from("0.0.0.0"));
        let port_string: String = dotenvy::var("PORT").unwrap_or(String::from("80"));
        let port: u16 = port_string.parse::<u16>().expect("PORT should be an unsigned number less than 65536");
        let app_state = get_appstate().await;
        let db = DB::new().await;
        std::env::set_var("RUST_LOG", "info,actix_web=warn,actix_server=warn");
        env_logger::init();
        let server = HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .app_data(web::Data::new(app_state.clone()))
                .app_data(web::Data::new(db.clone()))
                .service(actix_files::Files::new("/static", "./static").show_files_listing())
                .service(routes::home)
                .service(routes::add_song)
                .service(routes::admin_login)
                .service(routes::admin_redirect)
                .service(routes::admin)
                .service(routes::admin_set)
                .service(routes::get_deflist)
                .service(routes::post_deflist)
                .service(routes::admin_delete)
                .service(api::get_songs)
                .service(api::add_song)
        })
        .bind((bind_ip.clone(), port))?
        .run();
        actix_web::rt::spawn(async move {
            crate::downloader::run_downloader().await;
        });
        actix_web::rt::spawn(async move {
            crate::player::run_player().await;
        });
        info!("Web Server started on {}:{}", &bind_ip, &port);
        return server.await;
    }
}
