CREATE TEMPORARY TABLE songs_backup(id, path, parent, track_number, disc_number, title, year, album, artwork, duration, label);
INSERT INTO songs_backup SELECT id, path, parent, track_number, disc_number, title, year, album, artwork, duration, label FROM songs;
DROP TABLE songs;
CREATE TABLE songs (
    id INTEGER PRIMARY KEY NOT NULL,
    path TEXT NOT NULL,
    parent TEXT NOT NULL,
    track_number INTEGER,
    disc_number INTEGER,
    title TEXT,
    artist TEXT,
    album_artist TEXT,
    year INTEGER,
    album TEXT,
    artwork TEXT,
    duration INTEGER,
    lyricist TEXT,
    composer: TEXT,
    genre: TEXT,
    label: TEXT,
    UNIQUE(path) ON CONFLICT REPLACE
);
INSERT INTO songs SELECT * FROM songs_backup;
DROP TABLE songs_backup;
-- TODO insert first value of multi-value fields

CREATE TEMPORARY TABLE directories_backup(id, path, parent, year, album, artwork, date_added);
INSERT INTO directories_backup SELECT id, path, parent, year, album, artwork, date_added FROM directories;
DROP TABLE directories;
CREATE TABLE directories (
    id INTEGER PRIMARY KEY NOT NULL,
    path TEXT NOT NULL,
    parent TEXT,
	artist TEXT,
    year INTEGER,
    album TEXT,
    artwork TEXT,
    date_added INTEGER DEFAULT 0 NOT NULL,
    UNIQUE(path) ON CONFLICT REPLACE
);
INSERT INTO directories SELECT * FROM directories_backup;
DROP TABLE directories_backup;
-- TODO insert first value of multi-value fields

DROP TABLE artists;
DROP TABLE song_artists;
DROP TABLE song_album_artists;
DROP TABLE song_lyricists;
DROP TABLE song_composers;
DROP TABLE genres;
DROP TABLE song_genres;
DROP TABLE directory_artists;
