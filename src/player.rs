use crate::appstate::DB;
use crate::models::File;
use log::info;

use tokio::process::Command;

async fn delete_song(song: &String, fname: &String, db: &DB) {
    let _ = sqlx::query!("DELETE FROM songs WHERE song=?", song).execute(&db.pool).await;
    let _ = sqlx::query!("DELETE FROM pqueue WHERE song=?", song).execute(&db.pool).await;
    let _ = tokio::fs::remove_file(format!("songs/{}.opus", fname)).await;
    info!("Deleted song {} (filename: songs/{}.opus)", &song, &fname);
}

async fn should_song_be_played(db: &DB) -> bool {
    let query = sqlx::query!("SELECT value FROM admin WHERE id=1").fetch_one(&db.pool).await.unwrap();
    let admin_setting = query.value.unwrap();
    if admin_setting == "play" {
        return true;
    } else if admin_setting == "pause" {
        return false;
    } else {
        // Use any business logic to control when the music player plays. Here, an example for 10 to 6 week day is given. 
        // use chrono::{Datelike, Timelike};
        // use chrono::Weekday;
        // let time = chrono::Local::now();
        // let weekday = time.weekday();
        // let hour = time.hour();
        // //let minute = time.minute();
        // if weekday == Weekday::Sat || weekday == Weekday::Sun {
        //     return false;
        // }
        // if hour < 10 || hour > 17 {
        //     return false
        // }
        return true;
    }
}

async fn get_count_pqueue(db: &DB) -> i64 {
    let query = sqlx::query!("SELECT COUNT(song) AS count FROM pqueue").fetch_one(&db.pool).await;
    if query.is_err() {
        return 0;
    }
    let count = query.unwrap().count;
    return count;
}

async fn play_song(song: &File) {
    let file = format!("songs/{}.opus", &song.uuid);
    info!("Playing {} from songs/{}.opus", &song.song, &song.uuid);
    let _ = Command::new("mpv")
        .arg(file)
        .output()
        .await;
}

async fn get_song_from_pqueue(db: &DB) -> Option<File> {
    let query = sqlx::query_as!(File, "SELECT song, uuid FROM pqueue ORDER BY id LIMIT 1").fetch_one(&db.pool).await;
    if query.is_err() {
        return None;
    }
    let song = query.unwrap();
    return Some(song);
}

pub async fn run_player() {
    let db = DB::new().await;
    info!("Music Player Started");
    loop {
        let should_play = should_song_be_played(&db).await;
        if should_play {
            let count = get_count_pqueue(&db).await;
            if count == 0 {
                actix_web::rt::time::sleep(std::time::Duration::from_secs(10)).await;
                continue;
            }
            let s = get_song_from_pqueue(&db).await;
            if s.is_none() {
                actix_web::rt::time::sleep(std::time::Duration::from_secs(10)).await;
                continue;
            }
            let song = s.unwrap();
            play_song(&song).await;
            delete_song(&song.song, &song.uuid, &db).await;
        } else {
            actix_web::rt::time::sleep(std::time::Duration::from_secs(20)).await;
        }
        actix_web::rt::time::sleep(std::time::Duration::from_secs(2)).await; // silence after each song
    }
}
