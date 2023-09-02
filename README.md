# Server Jukebox

Jukebox to which anyone on the local network can queue any song.

## Deployment

+ Have a working MariaDB (MySQL) installation and create a database
+ Set appropriate variables in `.env`.
+ Add the songs you want to be in the default list in `migrations/0002_insert_data.sql`
+ Run migrations with `sqlx migrate run`
+ Make sure `yt-dlp`, `ffmpeg` and `mpv` are in system's `$PATH`
+ Start the server

## Contribute

Pull requests welcome. 