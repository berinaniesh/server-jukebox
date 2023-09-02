use actix_web::{web, get, post, HttpRequest, HttpResponse};
use crate::models::{Song, Response};
use crate::appstate::DB;
use log::info;

#[get("/api/songs")]
pub async fn get_songs(db: web::Data<DB>) -> HttpResponse {
	let query = sqlx::query_as!(
        Song,
        "SELECT song FROM songs ORDER BY id"
        ).fetch_all(&db.pool)
        .await;
    if query.is_err() {
        return HttpResponse::InternalServerError().json(Response {message: String::from("Something went wrong, try again later")});
    }
    let song_objects = query.unwrap();
    let mut songs = Vec::new();
    for song_object in song_objects {
        songs.push(song_object.song);
    }
    HttpResponse::Ok().json(songs)
}

#[post("/api/songs")]
pub async fn add_song(db: web::Data<DB>, form: web::Json<Song>, req: HttpRequest) -> HttpResponse {
    let ip: String = req.peer_addr().unwrap().ip().to_string();
    let _ = sqlx::query!("INSERT INTO songs (song) VALUES (?)", &form.song).execute(&db.pool).await.unwrap();
    let _ = sqlx::query!("INSERT INTO dqueue (song) VALUES (?)", &form.song).execute(&db.pool).await.unwrap();
    let _ = sqlx::query!("INSERT into log (song, ip) VALUES (?, ?)", &form.song, &ip).execute(&db.pool).await.unwrap();
    let query = sqlx::query_as!(
        Song,
        "SELECT song FROM songs ORDER BY id"
        ).fetch_all(&db.pool)
        .await
        .unwrap();
    let mut songs = Vec::new();
    for song_object in query {
        songs.push(song_object.song);
    }
    info!("{} added {}", &ip, &form.song);
    HttpResponse::Ok().json(songs)
}