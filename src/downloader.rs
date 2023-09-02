use crate::appstate::DB;
use tokio::process::Command;
use uuid::Uuid;
use log::{warn, info};

const BLACKLIST: [&str; 2] = ["@ARRahmansongscollections", "@tn74tamilmusic91"];

async fn get_count_dqueue(db: &DB) -> i64 {
    let query = sqlx::query!("SELECT COUNT(song) AS count FROM dqueue").fetch_one(&db.pool).await;
    if query.is_err() {
        return 0;
    }
    let count = query.unwrap().count;
    return count;
}

async fn get_count_pqueue(db: &DB) -> i64 {
    let query = sqlx::query!("SELECT COUNT(song) AS count FROM pqueue").fetch_one(&db.pool).await;
    if query.is_err() {
        return 5; // To tell the loop to sleep
    }
    let count = query.unwrap().count;
    return count;
}

async fn get_song_from_dqueue(db: &DB) -> Option<String> {
    let query = sqlx::query!("SELECT song FROM dqueue ORDER BY id LIMIT 1").fetch_one(&db.pool).await;
    if query.is_err() {
        return None;
    }
    let song = query.unwrap().song;
    return Some(song);
}

async fn get_song_from_deflist(db: &DB) -> Option<String> {
    let query = sqlx::query!("SELECT song FROM deflist ORDER BY RAND() LIMIT 1").fetch_one(&db.pool).await;
    if query.is_err() {
        return None;
    }
    let song = query.unwrap().song;
    return Some(song);
}

async fn get_id_and_duration(song: &String) -> (String, u64) {
    let duration_limit_str = dotenvy::var("DURATION_LIMIT").expect("DURATION_LIMIT should be set");
    let duration_limt: u64 = duration_limit_str.parse().unwrap_or(600);
    let mut output;
    let mut stdout;
    let mut stdout_vec;
    loop {
        output = Command::new("yt-dlp")
            .arg(format!("ytsearch3:{} high quality audio", &song))
            .arg("--print")
            .arg("id,uploader_id,duration")
            .output()
            .await
            .expect("yt-dlp failed to execute");
        stdout = std::str::from_utf8(&output.stdout).unwrap().trim();
        stdout_vec = stdout.split("\n").collect::<Vec<&str>>();
        if stdout_vec.len() == 9 {
            break;
        } else {
            warn!("Something went wrong trying to download the song {}. It will be retried.\nIf there is no internet, this error is harmless, if not, you should check if yt-dlp is up to date and restart the app after removing the current song.", &song);
            actix_web::rt::time::sleep(std::time::Duration::from_secs(5)).await

        }
    }
    if !BLACKLIST.contains(&stdout_vec[1]) && stdout_vec[2].parse::<u64>().unwrap() <= duration_limt {
        return (stdout_vec[0].to_string(), stdout_vec[2].parse::<u64>().unwrap());
    } else if !BLACKLIST.contains(&stdout_vec[4]) && stdout_vec[5].parse::<u64>().unwrap() <= duration_limt {
        return (stdout_vec[3].to_string(), stdout_vec[5].parse::<u64>().unwrap());
    } else if !BLACKLIST.contains(&stdout_vec[7]) && stdout_vec[8].parse::<u64>().unwrap() <= duration_limt {
        return (stdout_vec[6].to_string(), stdout_vec[8].parse::<u64>().unwrap());
    }
    return ("0".to_string(), duration_limt+1);
}

async fn delete_from_songs(song: &String, db: &DB) {
    let _ = sqlx::query!("DELETE FROM songs WHERE song=?", song).execute(&db.pool).await;
}

async fn delete_from_dqueue(song: &String, db: &DB) {
    let _ = sqlx::query!("DELETE FROM dqueue WHERE song=?", song).execute(&db.pool).await;
}

async fn add_to_pqueue(song: &String, fname: &String, db: &DB) {
    let _ = sqlx::query!("INSERT INTO pqueue (uuid, song) VALUES (?, ?)", fname, song).execute(&db.pool).await;
}

async fn download(video_id: &String, fname: &String) -> bool{
    let output = Command::new("yt-dlp")
        .arg("-P")
        .arg("songs")
        .arg("-f")
        .arg("251")
        .arg("-x")
        .arg(format!("https://www.youtube.com/watch?v={}", video_id))
        .arg("-o")
        .arg(fname)
        .output()
        .await
        .expect("yt-dlp failed to execute");
    return output.status.success();
}

async fn download_song(song: String, db: &DB) {
    let (id, duration) = get_id_and_duration(&song).await;
    let duration_limit_str = dotenvy::var("DURATION_LIMIT").expect("DURATION_LIMIT should be set");
    let duration_limt: u64 = duration_limit_str.parse().unwrap_or(600);
    if duration > duration_limt {
        info!("Duration exceeded, deleting song {}", &song);
        delete_from_songs(&song, &db).await;
        delete_from_dqueue(&song, &db).await;
        return;
    }
    let fname = Uuid::new_v4().to_string();
    loop {
        info!("Downloading {} from https://youtube.com/watch?v={}", &song, &id);
        let success = download(&id, &fname).await;
        if success {
            info!("Downloaded complete");
            add_to_pqueue(&song, &fname, &db).await;
            delete_from_dqueue(&song, &db).await;
            break;
        } else {
            warn!("Something went wrong trying to download the song {} from https://www.youtube.com/watch?v={}. It will be retried.\nIf there is no internet, this error is harmless, if not, you should check if yt-dlp is up to date and restart the app after removing the current song.", &song, &id);
            actix_web::rt::time::sleep(std::time::Duration::from_secs(5)).await
        }
    }

}

pub async fn run_downloader() {
    let db = DB::new().await;
    info!("Downloader Started");
    loop {
        let query = sqlx::query!("SELECT value FROM admin WHERE id=2").fetch_one(&db.pool).await.unwrap();
        let use_deflist = if query.value.unwrap() == "true" {true} else {false};
        let dcount = get_count_dqueue(&db).await;
        if dcount > 0 {
            let s = get_song_from_dqueue(&db).await;
            if s.is_none() {
                actix_web::rt::time::sleep(std::time::Duration::from_secs(10)).await;
                continue;
            } else {
                let song = s.unwrap();
                download_song(song, &db).await;
            }
        } else {
            let pcount = get_count_pqueue(&db).await;
            if pcount > 1 {
                actix_web::rt::time::sleep(std::time::Duration::from_secs(10)).await;
            } else {
                let s = get_song_from_deflist(&db).await;
                if !use_deflist || s.is_none() {
                    actix_web::rt::time::sleep(std::time::Duration::from_secs(10)).await;
                    continue;
                } else {
                    let song = s.unwrap();
                    info!("Number of songs in play queue less than 2, adding {} from the default list", &song);
                    download_song(song.clone(), &db).await;
                    let _ = sqlx::query!("INSERT INTO songs (song) VALUES (?)", &song).execute(&db.pool).await;
                }
            }
        }
        actix_web::rt::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
