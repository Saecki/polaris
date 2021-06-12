use crate::db::{
	artists, composers, directories, directory_artists, genres, lyricists, song_album_artists,
	song_artists, song_composers, song_genres, song_lyricists, songs, DB,
};
use crossbeam_channel::Receiver;
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use log::error;

const INDEX_BUILDING_INSERT_BUFFER_SIZE: usize = 1000; // Insertions in each transaction

pub struct Song {
	pub path: String,
	pub parent: String,
	pub track_number: Option<i32>,
	pub disc_number: Option<i32>,
	pub title: Option<String>,
	pub artists: Vec<String>,
	pub album_artists: Vec<String>,
	pub year: Option<i32>,
	pub album: Option<String>,
	pub artwork: Option<String>,
	pub duration: Option<i32>,
	pub lyricists: Vec<String>,
	pub composers: Vec<String>,
	pub genres: Vec<String>,
	pub label: Option<String>,
}

#[derive(Debug, Insertable)]
#[table_name = "artists"]
struct InsertArtist<'a> {
	pub name: &'a str,
}

#[derive(Debug, Insertable)]
#[table_name = "song_artists"]
struct InsertSongArtist {
	pub song: i32,
	pub artist: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "song_album_artists"]
struct InsertSongAlbumArtist {
	pub song: i32,
	pub artist: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "lyricists"]
struct InsertLyricist<'a> {
	pub name: &'a str,
}

#[derive(Debug, Insertable)]
#[table_name = "song_lyricists"]
struct InsertSongLyricist {
	pub song: i32,
	pub lyricist: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "composers"]
struct InsertComposer<'a> {
	pub name: &'a str,
}

#[derive(Debug, Insertable)]
#[table_name = "song_composers"]
struct InsertSongComposer {
	pub song: i32,
	pub composer: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "genres"]
struct InsertGenre<'a> {
	pub name: &'a str,
}

#[derive(Debug, Insertable)]
#[table_name = "song_genres"]
struct InsertSongGenre {
	pub song: i32,
	pub genre: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "songs"]
struct InsertSong<'a> {
	pub path: &'a str,
	pub parent: &'a str,
	pub track_number: Option<i32>,
	pub disc_number: Option<i32>,
	pub title: Option<&'a str>,
	pub year: Option<i32>,
	pub album: Option<&'a str>,
	pub artwork: Option<&'a str>,
	pub duration: Option<i32>,
	pub label: Option<&'a str>,
}

pub struct Directory {
	pub path: String,
	pub parent: Option<String>,
	pub artists: Vec<String>,
	pub year: Option<i32>,
	pub album: Option<String>,
	pub artwork: Option<String>,
	pub date_added: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "directory_artists"]
struct InsertDirectoryArtist {
	pub directory: i32,
	pub artist: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "directories"]
struct InsertDirectory<'a> {
	pub path: &'a str,
	pub parent: Option<&'a str>,
	pub year: Option<i32>,
	pub album: Option<&'a str>,
	pub artwork: Option<&'a str>,
	pub date_added: i32,
}

pub enum Item {
	Directory(Directory),
	Song(Song),
}

pub struct Inserter {
	receiver: Receiver<Item>,
	new_directories: Vec<Directory>,
	new_songs: Vec<Song>,
	db: DB,
}

impl Inserter {
	pub fn new(db: DB, receiver: Receiver<Item>) -> Self {
		Self {
			db,
			receiver,
			new_directories: Vec::with_capacity(INDEX_BUILDING_INSERT_BUFFER_SIZE),
			new_songs: Vec::with_capacity(INDEX_BUILDING_INSERT_BUFFER_SIZE),
		}
	}

	pub fn insert(&mut self) {
		loop {
			match self.receiver.recv() {
				Ok(item) => self.insert_item(item),
				Err(_) => break,
			}
		}
	}

	fn insert_item(&mut self, insert: Item) {
		match insert {
			Item::Directory(d) => {
				self.new_directories.push(d);
				if self.new_directories.len() >= INDEX_BUILDING_INSERT_BUFFER_SIZE {
					self.flush_directories();
				}
			}
			Item::Song(s) => {
				self.new_songs.push(s);
				if self.new_songs.len() >= INDEX_BUILDING_INSERT_BUFFER_SIZE {
					self.flush_songs();
				}
			}
		};
	}

	fn flush_directories(&mut self) {
		if let Ok(connection) = self.db.connect() {
			for d in self.new_directories.iter() {
				if insert_directory(&connection, d).is_err() {
					error!("Could not insert new directories in database");
				}
			}
		} else {
			error!("Could not connect to database to insert new directories");
		}
		self.new_directories.clear();
	}

	fn flush_songs(&mut self) {
		if let Ok(connection) = self.db.connect() {
			for s in self.new_songs.iter() {
				if insert_song(&connection, s).is_err() {
					error!("Could not insert new song in database");
				}
			}
		} else {
			error!("Could not connect to database to insert new songs");
		}
		self.new_songs.clear();
	}
}

impl Drop for Inserter {
	fn drop(&mut self) {
		if self.new_directories.len() > 0 {
			self.flush_directories();
		}
		if self.new_songs.len() > 0 {
			self.flush_songs();
		}
	}
}

fn insert_song(
	connection: &PooledConnection<ConnectionManager<SqliteConnection>>,
	song: &Song,
) -> anyhow::Result<()> {
	diesel::insert_into(songs::table)
		.values(InsertSong {
			path: &song.path,
			parent: &song.parent,
			track_number: song.track_number,
			disc_number: song.disc_number,
			title: song.title.as_deref(),
			year: song.year,
			album: song.album.as_deref(),
			artwork: song.artwork.as_deref(),
			duration: song.duration,
			label: song.label.as_deref(),
		})
		.execute(connection)?;

	let song_id: i32 = {
		use self::directories::dsl::*;
		directories
			.select(id)
			.filter(path.eq(song.path))
			.get_result(&**connection)
			.map_err(anyhow::Error::new)?
	};

	// artists
	let artists: Vec<_> = song
		.artists
		.iter()
		.map(|a| InsertArtist { name: a })
		.collect();

	diesel::insert_into(artists::table)
		.values(&artists)
		.execute(&**connection)?;

	let artist_ids: Vec<i32> = {
		use self::artists::dsl::*;
		artists
			.select(id)
			.filter(name.eq_any(&song.artists))
			.get_results(&**connection)
			.map_err(anyhow::Error::new)?
	};

	let song_artists: Vec<_> = artist_ids
		.into_iter()
		.map(|id| InsertSongArtist {
			song: song_id,
			artist: id,
		})
		.collect();

	diesel::insert_into(song_artists::table)
		.values(&song_artists)
		.execute(&**connection)
		.map_err(anyhow::Error::new)?;

	// album artists
	let album_artists: Vec<InsertArtist> = song
		.artists
		.iter()
		.map(|a| InsertArtist { name: a })
		.collect();

	diesel::insert_into(artists::table)
		.values(&album_artists)
		.execute(&**connection)?;

	let album_artist_ids: Vec<i32> = {
		use self::artists::dsl::*;
		artists
			.select(id)
			.filter(name.eq_any(&song.album_artists))
			.get_results(&**connection)
			.map_err(anyhow::Error::new)?
	};

	let song_album_artists: Vec<_> = album_artist_ids
		.into_iter()
		.map(|id| InsertSongAlbumArtist {
			song: song_id,
			artist: id,
		})
		.collect();

	diesel::insert_into(song_album_artists::table)
		.values(&song_album_artists)
		.execute(&**connection)
		.map_err(anyhow::Error::new)?;

	// lyricists
	let lyricists: Vec<_> = song
		.artists
		.iter()
		.map(|a| InsertLyricist { name: a })
		.collect();

	diesel::insert_into(lyricists::table)
		.values(&lyricists)
		.execute(&**connection)?;

	let lyricist_ids: Vec<i32> = {
		use self::artists::dsl::*;
		artists
			.select(id)
			.filter(name.eq_any(&song.lyricists))
			.get_results(&**connection)
			.map_err(anyhow::Error::new)?
	};

	let song_lyricists: Vec<_> = lyricist_ids
		.into_iter()
		.map(|id| InsertSongLyricist {
			song: song_id,
			lyricist: id,
		})
		.collect();

	diesel::insert_into(song_lyricists::table)
		.values(&song_lyricists)
		.execute(&**connection)
		.map_err(anyhow::Error::new)?;

	// composers
	let composers: Vec<_> = song
		.artists
		.iter()
		.map(|a| InsertComposer { name: a })
		.collect();

	diesel::insert_into(composers::table)
		.values(&composers)
		.execute(&**connection)?;

	let composer_ids: Vec<i32> = {
		use self::artists::dsl::*;
		artists
			.select(id)
			.filter(name.eq_any(&song.composers))
			.get_results(&**connection)
			.map_err(anyhow::Error::new)?
	};

	let song_composers: Vec<_> = composer_ids
		.into_iter()
		.map(|id| InsertSongComposer {
			song: song_id,
			composer: id,
		})
		.collect();

	diesel::insert_into(song_composers::table)
		.values(&song_composers)
		.execute(&**connection)
		.map_err(anyhow::Error::new)?;

	// genres
	let genres: Vec<_> = song
		.artists
		.iter()
		.map(|a| InsertGenre { name: a })
		.collect();

	diesel::insert_into(genres::table)
		.values(&genres)
		.execute(&**connection)?;

	let genre_ids: Vec<i32> = {
		use self::artists::dsl::*;
		artists
			.select(id)
			.filter(name.eq_any(&song.genres))
			.get_results(&**connection)
			.map_err(anyhow::Error::new)?
	};

	let song_genres: Vec<_> = genre_ids
		.into_iter()
		.map(|id| InsertSongGenre {
			song: song_id,
			genre: id,
		})
		.collect();

	diesel::insert_into(song_genres::table)
		.values(&song_genres)
		.execute(&**connection)
		.map_err(anyhow::Error::new)?;

	Ok(())
}

fn insert_directory(
	connection: &PooledConnection<ConnectionManager<SqliteConnection>>,
	dir: &Directory,
) -> anyhow::Result<()> {
	diesel::insert_into(directories::table)
		.values(InsertDirectory {
			path: &dir.path,
			parent: dir.parent.as_deref(),
			year: dir.year,
			album: dir.album.as_deref(),
			artwork: dir.artwork.as_deref(),
			date_added: dir.date_added,
		})
		.execute(connection)?;

	let dir_id: i32 = {
		use self::directories::dsl::*;
		directories
			.select(id)
			.filter(path.eq(dir.path))
			.get_result(&**connection)
			.map_err(anyhow::Error::new)?
	};

	let artists: Vec<InsertArtist> = dir
		.artists
		.iter()
		.map(|a| InsertArtist { name: a })
		.collect();

	diesel::insert_into(artists::table)
		.values(&artists)
		.execute(&**connection)?;

	let artist_ids: Vec<i32> = {
		use self::artists::dsl::*;
		artists
			.select(id)
			.filter(name.eq_any(&dir.artists))
			.get_results(&**connection)
			.map_err(anyhow::Error::new)?
	};

	let directory_artists: Vec<InsertDirectoryArtist> = artist_ids
		.into_iter()
		.map(|id| InsertDirectoryArtist {
			directory: dir_id,
			artist: id,
		})
		.collect();

	diesel::insert_into(directory_artists::table)
		.values(&directory_artists)
		.execute(&**connection)
		.map_err(anyhow::Error::new)?;

	Ok(())
}
