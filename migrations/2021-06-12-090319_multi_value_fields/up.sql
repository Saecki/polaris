CREATE TABLE artists (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    UNIQUE(name) ON CONFLICT IGNORE
);

CREATE TABLE song_artists (
    id INTEGER PRIMARY KEY NOT NULL,
    song INTEGER NOT NULL,
    artist INTEGER NOT NULL,
    FOREIGN KEY(song) REFERENCES songs(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(artist) REFERENCES artists(id) ON DELETE CASCADE ON UPDATE CASCADE,
    UNIQUE(song, artist) ON CONFLICT IGNORE
);

CREATE TABLE song_album_artists (
    id INTEGER PRIMARY KEY NOT NULL,
    song INTEGER NOT NULL,
    artist INTEGER NOT NULL,
    FOREIGN KEY(song) REFERENCES songs(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(artist) REFERENCES artists(id) ON DELETE CASCADE ON UPDATE CASCADE,
    UNIQUE(song, artist) ON CONFLICT IGNORE
);

CREATE TABLE lyricists (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    UNIQUE(name) ON CONFLICT IGNORE
);

CREATE TABLE song_lyricists (
    id INTEGER PRIMARY KEY NOT NULL,
    song INTEGER NOT NULL,
    lyricist INTEGER NOT NULL,
    FOREIGN KEY(song) REFERENCES songs(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(lyricist) REFERENCES lyricists(id) ON DELETE CASCADE ON UPDATE CASCADE,
    UNIQUE(song, lyricist) ON CONFLICT IGNORE
);

CREATE TABLE composers (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    UNIQUE(name) ON CONFLICT IGNORE
);

CREATE TABLE song_composers (
    id INTEGER PRIMARY KEY NOT NULL,
    song INTEGER NOT NULL,
    composer INTEGER NOT NULL,
    FOREIGN KEY(song) REFERENCES songs(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(composer) REFERENCES composers(id) ON DELETE CASCADE ON UPDATE CASCADE,
    UNIQUE(song, composer) ON CONFLICT IGNORE
);

CREATE TABLE genres (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    UNIQUE(name) ON CONFLICT IGNORE
);

CREATE TABLE song_genres (
    id INTEGER PRIMARY KEY NOT NULL,
    song INTEGER NOT NULL,
    genre INTEGER NOT NULL,
    FOREIGN KEY(song) REFERENCES songs(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(genre) REFERENCES genres(id) ON DELETE CASCADE ON UPDATE CASCADE,
    UNIQUE(song, genre) ON CONFLICT IGNORE
);

CREATE TABLE directory_artists (
    id INTEGER PRIMARY KEY NOT NULL,
    directory INTEGER NOT NULL,
    artist INTEGER NOT NULL,
    FOREIGN KEY(directory) REFERENCES directories(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(artist) REFERENCES artists(id) ON DELETE CASCADE ON UPDATE CASCADE,
    UNIQUE(directory, artist) ON CONFLICT IGNORE
);

INSERT INTO artists SELECT songs.artist;
INSERT INTO artists SELECT songs.album_artist;
INSERT INTO artists SELECT songs.lyricist;
INSERT INTO artists SELECT songs.composer;
INSERT INTO song_artists SELECT songs.id AS song, artists.id AS artist FROM songs INNER JOIN artists on artists.name = songs.artist;
INSERT INTO song_album_artists SELECT songs.id AS song, artists.id AS artist FROM songs INNER JOIN artists on artists.name = songs.album_artist;
INSERT INTO song_lyricists SELECT songs.id AS song, artists.id AS artist FROM songs INNER JOIN artists on artists.name = songs.lyricist;
INSERT INTO song_composers SELECT songs.id AS song, artists.id AS artist FROM songs INNER JOIN artists on artists.name = songs.composer;

INSERT INTO genres SELECT songs.artist;
INSERT INTO song_genres SELECT songs.id AS song, genres.id AS genre FROM songs INNER JOIN genres on genres.name = songs.genre;

INSERT INTO artists SELECT directories.artist;
INSERT INTO directory_artists SELECT directories.id AS directory, artists.id AS artist FROM directories INNER JOIN artists on artists.name = directories.artist;

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
    year INTEGER,
    album TEXT,
    artwork TEXT,
    duration INTEGER,
    label TEXT,
    UNIQUE(path) ON CONFLICT REPLACE
);
INSERT INTO songs SELECT * FROM songs_backup;
DROP TABLE songs_backup;

CREATE TEMPORARY TABLE directories_backup(id, path, parent, year, album, artwork, date_added);
INSERT INTO directories_backup SELECT id, path, parent, year, album, artwork, date_added FROM directories;
DROP TABLE directories;
CREATE TABLE directories (
    id INTEGER PRIMARY KEY NOT NULL,
    path TEXT NOT NULL,
    parent TEXT,
    year INTEGER,
    album TEXT,
    artwork TEXT,
    date_added INTEGER DEFAULT 0 NOT NULL,
    UNIQUE(path) ON CONFLICT REPLACE
);
INSERT INTO songs SELECT * FROM directories_backup;
DROP TABLE directories_backup;

