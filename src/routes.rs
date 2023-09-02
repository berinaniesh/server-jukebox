use actix_web::{web, get, post, Responder, HttpRequest, HttpResponse};
use crate::appstate::AppState;
use tera::Context;
use log::info;

use crate::models::{Song, Password, Admin};

const ADMIN_URL: &str = "/5213a1cd-da0a-49cb-a431-ae1c002a5edd";
const DEFLIST_URL: &str = "/d444620c-88dc-43f7-99dd-5f7c470755a5";

#[get("/")]
pub async fn home(data: web::Data<AppState>) -> impl Responder {
    let ctx = Context::new();
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/add")]
pub async fn add_song(data: web::Data<AppState>, form: web::Form<Song>, req: HttpRequest) -> impl Responder {
    let ip: String = req.peer_addr().unwrap().ip().to_string();
    let song = form.song.trim();
    let _ = sqlx::query!("INSERT INTO songs (song) VALUES (?)", &song).execute(&data.pool).await;
    let _ = sqlx::query!("INSERT INTO dqueue (song) VALUES (?)", &song).execute(&data.pool).await;
    let _ = sqlx::query!("INSERT into log (song, ip) VALUES (?, ?)", &song, &ip).execute(&data.pool).await;
    info!("{} added {}", &ip, &song);
    return web::Redirect::to("/").see_other();
}

#[get("/admin")]
pub async fn admin_login(data: web::Data<AppState>) -> impl Responder {
    let ctx = Context::new();
    let rendered = data.tmpl.render("admin_login.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/admin")]
pub async fn admin_redirect(form: web::Form<Password>) -> impl Responder {
    if form.password == dotenvy::var("ADMIN_PASS").expect("ADMIN_PASS should be set") {
        return web::Redirect::to(ADMIN_URL).see_other();
    } else if form.password == dotenvy::var("DEFLIST_PASS").expect("DEFLIST_PASS should be set") {
        return web::Redirect::to(DEFLIST_URL).see_other();
    }
    else { return web::Redirect::to("/admin").see_other() };
}

#[get("/5213a1cd-da0a-49cb-a431-ae1c002a5edd")]
pub async fn admin(data: web::Data<AppState>) -> impl Responder {
    let mut ctx = Context::new();
    let query = sqlx::query!("SELECT value FROM admin WHERE id=1").fetch_one(&data.pool).await.unwrap();
    let value = query.value.unwrap();
    if value == "play" {
       ctx.insert("current_status", "Play");
    } else if value == "pause" {
        ctx.insert("current_status", "Pause");
    } else {
        ctx.insert("current_status", "Default");
    }
    let rendered = data.tmpl.render("admin_settings.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/5213a1cd-da0a-49cb-a431-ae1c002a5edd")]
pub async fn admin_set(data: web::Data<AppState>, form: web::Form<Admin>) -> impl Responder {
    let _ = sqlx::query!("UPDATE admin SET value=? WHERE id=1", form.value).execute(&data.pool).await.unwrap();
    return web::Redirect::to(ADMIN_URL).see_other();
}

#[get("/d444620c-88dc-43f7-99dd-5f7c470755a5")]
pub async fn get_deflist(data: web::Data<AppState>) -> impl Responder {
    let mut ctx = Context::new();
    let query = sqlx::query!("SELECT value FROM admin WHERE id=2").fetch_one(&data.pool).await.unwrap();
    let value = query.value.unwrap();
    if value == "true" {
       ctx.insert("current_status", "Use default list when song count is low");
    } else {
        ctx.insert("current_status", "Don't use default list");
    }
    let rendered = data.tmpl.render("deflist_settings.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/d444620c-88dc-43f7-99dd-5f7c470755a5")]
pub async fn post_deflist(data: web::Data<AppState>, form: web::Form<Admin>) -> impl Responder {
    let _ = sqlx::query!("UPDATE admin SET value=? WHERE id=2", form.value).execute(&data.pool).await.unwrap();
    return web::Redirect::to(DEFLIST_URL).see_other();
}

#[get("/admindelete/{slug}")]
pub async fn admin_delete(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let song = path.into_inner();
    let _ = sqlx::query!("DELETE FROM songs WHERE song=? ORDER BY id LIMIT 1", &song).execute(&data.pool).await;
    let _ = sqlx::query!("DELETE FROM dqueue WHERE song=? ORDER BY id LIMIT 1", &song).execute(&data.pool).await;
    let _ = sqlx::query!("DELETE FROM pqueue WHERE song=? ORDER BY id LIMIT 1", &song).execute(&data.pool).await;
    return web::Redirect::to("/").see_other();
}
